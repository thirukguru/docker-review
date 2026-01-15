use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

static LATEST_TAG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([^:@\s]+)(?::latest)?(?:\s|$)").unwrap()
});

static EXPLICIT_TAG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[^:@\s]+:[^@\s]+").unwrap()
});

pub struct LatestTagRule;

impl Rule for LatestTagRule {
    fn id(&self) -> &'static str { "DF001" }
    fn name(&self) -> &'static str { "Using latest tag" }
    fn severity(&self) -> Severity { Severity::Critical }
    
    fn description(&self) -> &'static str {
        "Avoid using the 'latest' tag or omitting tags in FROM instructions"
    }
    
    fn rationale(&self) -> &'static str {
        "Using 'latest' or implicit tags causes unpredictable builds. The same Dockerfile \
         may build differently on different days as the base image changes. This breaks \
         reproducibility, makes debugging harder, and can introduce security vulnerabilities \
         without warning."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Pin to a specific version tag (e.g., FROM node:18.17.0-alpine)")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: Some("Prevents unexpected vulnerability introduction".to_string()),
            reliability_improvement: Some("100% build reproducibility".to_string()),
        })
    }
}

impl DockerfileRule for LatestTagRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        for instruction in parser.get_instructions("FROM") {
            let args = &instruction.arguments;
            
            // Extract image reference (handle AS alias)
            let image_ref = args.split_whitespace().next().unwrap_or("");
            
            // Skip scratch (special case)
            if image_ref == "scratch" {
                continue;
            }
            
            // Skip build stage references (e.g., FROM builder)
            if !image_ref.contains('/') && !image_ref.contains(':') && !image_ref.contains('.') {
                // Could be a build stage reference, check if it's a common image
                let common_images = ["alpine", "ubuntu", "debian", "node", "python", "golang", "rust", "nginx", "redis", "postgres", "mysql"];
                if !common_images.iter().any(|img| image_ref.starts_with(img)) {
                    continue; // Likely a build stage reference
                }
            }
            
            // Check for explicit :latest
            if image_ref.ends_with(":latest") {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: Some(instruction.line_number),
                    message: format!("Image '{}' explicitly uses ':latest' tag", image_ref),
                    fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                    impact: self.impact(),
                });
            }
            // Check for missing tag (implicit latest)
            else if !EXPLICIT_TAG_RE.is_match(image_ref) && !image_ref.contains('@') {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: Some(instruction.line_number),
                    message: format!("Image '{}' has no tag (implicitly uses 'latest')", image_ref),
                    fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                    impact: self.impact(),
                });
            }
        }
        
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_explicit_latest() {
        let content = "FROM node:latest\nRUN npm install";
        let parser = DockerfileParser::parse_content(content);
        let rule = LatestTagRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("latest"));
    }

    #[test]
    fn test_detects_implicit_latest() {
        let content = "FROM ubuntu\nRUN apt-get update";
        let parser = DockerfileParser::parse_content(content);
        let rule = LatestTagRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("no tag"));
    }

    #[test]
    fn test_allows_pinned_version() {
        let content = "FROM node:18.17.0-alpine\nRUN npm install";
        let parser = DockerfileParser::parse_content(content);
        let rule = LatestTagRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_allows_digest() {
        let content = "FROM node@sha256:abc123\nRUN npm install";
        let parser = DockerfileParser::parse_content(content);
        let rule = LatestTagRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
