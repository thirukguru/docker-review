use crate::parser::{DockerfileParser, Instruction};
use crate::rules::Issue;
use std::collections::HashSet;

/// Generates optimized Dockerfiles based on detected issues
pub struct DockerfileFixer;

impl DockerfileFixer {
/// Generate an optimized Dockerfile based on the original and detected issues
pub fn fix(parser: &DockerfileParser, issues: &[Issue]) -> FixResult {
let mut result = FixResult::new(&parser.raw_content);

// Collect rule IDs that have issues
let issue_rules: HashSet<&str> = issues.iter()
.map(|i| i.rule_id.as_str())
.collect();

let mut lines: Vec<String> = parser.raw_content.lines().map(|l| l.to_string()).collect();
let mut insertions: Vec<(usize, usize, String)> = Vec::new();
let mut has_user_instruction = parser.has_instruction("USER");
let mut has_healthcheck = parser.has_instruction("HEALTHCHECK");

// Track which lines to modify
let mut line_modifications: std::collections::HashMap<usize, String> = std::collections::HashMap::new();

// Process each instruction
for instruction in &parser.instructions {
let line_idx = instruction.line_number.saturating_sub(1);

// DF001: Fix latest/missing tags
if issue_rules.contains("DF001") && instruction.name == "FROM" {
if let Some(fixed) = Self::fix_latest_tag(instruction) {
line_modifications.insert(line_idx, fixed.clone());
result.add_change(instruction.line_number, "Fixed image tag");
}
}

// DF006: Remove secrets from ENV
if issue_rules.contains("DF006") && instruction.name == "ENV" {
if Self::is_secret_env(&instruction.arguments) {
let commented = format!("# REMOVED (secret detected): {}", instruction.raw_line);
line_modifications.insert(line_idx, commented);
result.add_change(instruction.line_number, "Removed secret from ENV");
}
}

// DF010: Warn about curl|bash
if issue_rules.contains("DF010") && instruction.name == "RUN" {
if instruction.arguments.contains("curl") && instruction.arguments.contains("| bash")
|| instruction.arguments.contains("| sh")
|| instruction.arguments.contains("|bash")
|| instruction.arguments.contains("|sh") {
let warning = format!("# SECURITY WARNING: curl|bash pattern detected - review manually\n{}", instruction.raw_line);
line_modifications.insert(line_idx, warning);
result.add_change(instruction.line_number, "Added security warning for curl|bash");
}
}
}

// Apply line modifications
for (idx, new_content) in &line_modifications {
if *idx < lines.len() {
lines[*idx] = new_content.clone();
}
}

// DF005: Add HEALTHCHECK if missing (insert first, before USER)
if issue_rules.contains("DF005") && !has_healthcheck {
if let Some(insert_line) = Self::find_healthcheck_insertion_point(parser) {
insertions.push((insert_line, 0, "\nHEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \\".to_string()));
insertions.push((insert_line, 1, "    CMD curl -f http://localhost:8080/health || exit 1".to_string()));
result.add_change(insert_line + 1, "Added HEALTHCHECK instruction");
}
}

// DF002: Add non-root user if missing (insert after HEALTHCHECK)
if issue_rules.contains("DF002") && !has_user_instruction {
// Find the last instruction before CMD/ENTRYPOINT
if let Some(insert_line) = Self::find_user_insertion_point(parser) {
insertions.push((insert_line, 2, "\n# Run as non-root user for security".to_string()));
insertions.push((insert_line, 3, "RUN useradd -m -s /bin/bash appuser".to_string()));
insertions.push((insert_line, 4, "USER appuser".to_string()));
result.add_change(insert_line + 1, "Added non-root user");
}
}

// Sort insertions: first by line number (descending), then by order (descending)
// This ensures we insert from bottom-up, and within same line, in correct order
insertions.sort_by(|a, b| {
match b.0.cmp(&a.0) {
std::cmp::Ordering::Equal => b.1.cmp(&a.1),
other => other,
}
});

// Apply insertions
for (line_num, _order, content) in insertions {
if line_num <= lines.len() {
lines.insert(line_num, content);
}
}

result.optimized_content = lines.join("\n");

// Ensure trailing newline
if !result.optimized_content.ends_with('\n') {
result.optimized_content.push('\n');
}

result
}

/// Fix latest tag issue - add a recommended version
fn fix_latest_tag(instruction: &Instruction) -> Option<String> {
let args = &instruction.arguments;
let image_ref = args.split_whitespace().next().unwrap_or("");

// Common image recommendations
let recommendations: &[(&str, &str)] = &[
("ubuntu", "ubuntu:22.04"),
("debian", "debian:bookworm-slim"),
("alpine", "alpine:3.19"),
("python", "python:3.11-slim"),
("node", "node:20-alpine"),
("golang", "golang:1.21-alpine"),
("rust", "rust:1.75-slim"),
("nginx", "nginx:1.25-alpine"),
("redis", "redis:7-alpine"),
("postgres", "postgres:16-alpine"),
("mysql", "mysql:8.0"),
];

let base_image = image_ref.split(':').next().unwrap_or(image_ref);
let base_image = base_image.split('/').last().unwrap_or(base_image);

for (img, recommended) in recommendations {
if base_image == *img {
let rest = args.strip_prefix(image_ref).unwrap_or("");
return Some(format!("FROM {}{}", recommended, rest));
}
}

// If not a known image, just add :latest explicitly with a comment
if !image_ref.contains(':') && !image_ref.contains('@') {
let rest = args.strip_prefix(image_ref).unwrap_or("");
return Some(format!("# TODO: Pin to specific version\nFROM {}:latest{}", image_ref, rest));
}

None
}

/// Check if an ENV instruction contains secrets
fn is_secret_env(args: &str) -> bool {
let secret_patterns = [
"PASSWORD", "SECRET", "API_KEY", "APIKEY", "TOKEN", 
"PRIVATE_KEY", "CREDENTIAL", "AUTH", "AWS_", "DB_PASS"
];

let upper = args.to_uppercase();
secret_patterns.iter().any(|p| upper.contains(p))
}

/// Find the best line to insert USER instruction (before CMD/ENTRYPOINT)
fn find_user_insertion_point(parser: &DockerfileParser) -> Option<usize> {
// Find CMD or ENTRYPOINT
for instruction in &parser.instructions {
if instruction.name == "CMD" || instruction.name == "ENTRYPOINT" {
return Some(instruction.line_number.saturating_sub(1));
}
}
// If no CMD/ENTRYPOINT, insert at end
Some(parser.raw_content.lines().count())
}

/// Find the best line to insert HEALTHCHECK (before CMD/ENTRYPOINT)
fn find_healthcheck_insertion_point(parser: &DockerfileParser) -> Option<usize> {
Self::find_user_insertion_point(parser)
}
}

/// Result of fixing a Dockerfile
pub struct FixResult {
pub original_content: String,
pub optimized_content: String,
pub changes: Vec<FixChange>,
}

impl FixResult {
fn new(original: &str) -> Self {
Self {
original_content: original.to_string(),
optimized_content: String::new(),
changes: Vec::new(),
}
}

fn add_change(&mut self, line: usize, description: &str) {
self.changes.push(FixChange {
line_number: line,
description: description.to_string(),
});
}

/// Generate a unified diff between original and optimized
pub fn generate_diff(&self) -> String {
let mut diff = String::new();
diff.push_str("--- Original Dockerfile\n");
diff.push_str("+++ Optimized Dockerfile\n");

let original_lines: Vec<&str> = self.original_content.lines().collect();
let optimized_lines: Vec<&str> = self.optimized_content.lines().collect();

// Simple line-by-line diff
let max_len = original_lines.len().max(optimized_lines.len());

for i in 0..max_len {
let orig = original_lines.get(i).map(|s| *s);
let opt = optimized_lines.get(i).map(|s| *s);

match (orig, opt) {
(Some(o), Some(n)) if o == n => {
diff.push_str(&format!(" {}\n", o));
}
(Some(o), Some(n)) => {
diff.push_str(&format!("-{}\n", o));
diff.push_str(&format!("+{}\n", n));
}
(Some(o), None) => {
diff.push_str(&format!("-{}\n", o));
}
(None, Some(n)) => {
diff.push_str(&format!("+{}\n", n));
}
(None, None) => {}
}
}

diff
}
}

/// A single change made to the Dockerfile
pub struct FixChange {
pub line_number: usize,
pub description: String,
}
