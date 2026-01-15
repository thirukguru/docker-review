use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;

pub struct RootUserRule;

impl Rule for RootUserRule {
    fn id(&self) -> &'static str { "DF002" }
    fn name(&self) -> &'static str { "Running as root" }
    fn severity(&self) -> Severity { Severity::Critical }
    
    fn description(&self) -> &'static str {
        "Container runs as root user without specifying a non-root USER"
    }
    
    fn rationale(&self) -> &'static str {
        "Running containers as root is a significant security risk. If an attacker \
         compromises the container, they have root privileges which can be used to \
         escape to the host system or access sensitive data. Always run containers \
         as a non-privileged user."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Add a USER instruction to switch to a non-root user (e.g., USER node, USER 1000)")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: Some("Major security improvement - reduces container breakout risk".to_string()),
            reliability_improvement: None,
        })
    }
}

impl DockerfileRule for RootUserRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Check if there's a USER instruction that sets a non-root user
        let user_instructions = parser.get_instructions("USER");
        
        if user_instructions.is_empty() {
            // No USER instruction at all
            issues.push(Issue {
                rule_id: self.id().to_string(),
                rule_name: self.name().to_string(),
                severity: self.severity(),
                line_number: None,
                message: "No USER instruction found - container will run as root".to_string(),
                fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                impact: self.impact(),
            });
        } else {
            // Check if the last USER instruction sets root
            if let Some(last_user) = user_instructions.last() {
                let user = last_user.arguments.trim().to_lowercase();
                if user == "root" || user == "0" {
                    issues.push(Issue {
                        rule_id: self.id().to_string(),
                        rule_name: self.name().to_string(),
                        severity: self.severity(),
                        line_number: Some(last_user.line_number),
                        message: "Container explicitly set to run as root".to_string(),
                        fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
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
    fn test_detects_no_user() {
        let content = "FROM alpine\nRUN apk add curl";
        let parser = DockerfileParser::parse_content(content);
        let rule = RootUserRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_detects_explicit_root() {
        let content = "FROM alpine\nUSER root\nRUN apk add curl";
        let parser = DockerfileParser::parse_content(content);
        let rule = RootUserRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_allows_non_root_user() {
        let content = "FROM alpine\nRUN adduser -D app\nUSER app";
        let parser = DockerfileParser::parse_content(content);
        let rule = RootUserRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
