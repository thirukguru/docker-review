use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;

const LARGE_IMAGES: &[&str] = &[
    "ubuntu",
    "debian",
    "centos",
    "fedora",
    "amazonlinux",
    "oraclelinux",
];

const SLIM_ALTERNATIVES: &[(&str, &str)] = &[
    ("ubuntu", "ubuntu:*-slim or alpine"),
    ("debian", "debian:*-slim or alpine"),
    ("node", "node:*-alpine or node:*-slim"),
    ("python", "python:*-alpine or python:*-slim"),
    ("golang", "golang:*-alpine"),
    ("ruby", "ruby:*-alpine or ruby:*-slim"),
];

pub struct LargeBaseImageRule;

impl Rule for LargeBaseImageRule {
    fn id(&self) -> &'static str { "DF009" }
    fn name(&self) -> &'static str { "Large base image" }
    fn severity(&self) -> Severity { Severity::Suggestion }
    
    fn description(&self) -> &'static str {
        "Using a large base image when smaller alternatives exist"
    }
    
    fn rationale(&self) -> &'static str {
        "Large base images (ubuntu, debian, centos) are often 100MB-1GB. Alpine-based \
         or slim images are typically 5-50MB. Smaller images download faster, have \
         smaller attack surface, and reduce storage/transfer costs."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Use alpine or slim variants (e.g., node:18-alpine, python:3.11-slim)")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: Some("Faster image pulls".to_string()),
            image_size_reduction: Some("Can reduce base image by 70-95%".to_string()),
            security_improvement: Some("Smaller attack surface".to_string()),
            reliability_improvement: None,
        })
    }
}

impl DockerfileRule for LargeBaseImageRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        for instruction in parser.get_instructions("FROM") {
            let image_ref = instruction.arguments.split_whitespace().next().unwrap_or("");
            let image_name = image_ref.split(':').next().unwrap_or("");
            let image_name_lower = image_name.to_lowercase();
            
            // Extract base name without registry
            let base_name = image_name_lower.rsplit('/').next().unwrap_or(&image_name_lower);
            
            // Check for known large images
            if LARGE_IMAGES.iter().any(|&img| base_name == img) {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: Some(instruction.line_number),
                    message: format!("'{}' is a large base image - consider alpine or slim variants", image_name),
                    fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                    impact: self.impact(),
                });
            }
            // Check for non-slim/alpine variants of common images
            else if !image_ref.contains("alpine") && !image_ref.contains("slim") && !image_ref.contains("distroless") {
                if let Some((_, suggestion)) = SLIM_ALTERNATIVES.iter().find(|(name, _)| base_name.starts_with(name)) {
                    issues.push(Issue {
                        rule_id: self.id().to_string(),
                        rule_name: self.name().to_string(),
                        severity: self.severity(),
                        line_number: Some(instruction.line_number),
                        message: format!("Consider using {} for '{}' to reduce image size", suggestion, image_name),
                        fix_suggestion: Some(format!("Use {}", suggestion)),
                        impact: self.impact(),
                    });
                }
            }
        }
        
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_ubuntu() {
        let content = "FROM ubuntu:20.04\nRUN apt-get update";
        let parser = DockerfileParser::parse_content(content);
        let rule = LargeBaseImageRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_detects_non_alpine_node() {
        let content = "FROM node:18\nRUN npm install";
        let parser = DockerfileParser::parse_content(content);
        let rule = LargeBaseImageRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_allows_alpine() {
        let content = "FROM node:18-alpine\nRUN npm install";
        let parser = DockerfileParser::parse_content(content);
        let rule = LargeBaseImageRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_allows_slim() {
        let content = "FROM python:3.11-slim\nRUN pip install flask";
        let parser = DockerfileParser::parse_content(content);
        let rule = LargeBaseImageRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
