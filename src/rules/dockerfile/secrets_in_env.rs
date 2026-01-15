use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

static SECRET_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(password|passwd|secret|api_key|apikey|auth_token|access_token|private_key|credentials?)\s*=").unwrap()
});

pub struct SecretsInEnvRule;

impl Rule for SecretsInEnvRule {
    fn id(&self) -> &'static str { "DF006" }
    fn name(&self) -> &'static str { "Secrets in ENV" }
    fn severity(&self) -> Severity { Severity::Critical }
    
    fn description(&self) -> &'static str {
        "Potential secrets or passwords hardcoded in ENV instructions"
    }
    
    fn rationale(&self) -> &'static str {
        "Secrets in ENV instructions are baked into the image and visible to anyone \
         with access to the image. They appear in docker history, can be extracted, \
         and cannot be rotated without rebuilding. Use runtime secrets instead."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Use runtime environment variables, Docker secrets, or a secrets manager instead")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: Some("Critical - prevents credential exposure".to_string()),
            reliability_improvement: None,
        })
    }
}

impl DockerfileRule for SecretsInEnvRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        for instruction in parser.get_instructions("ENV") {
            if SECRET_PATTERN.is_match(&instruction.arguments) {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: Some(instruction.line_number),
                    message: "Potential secret detected in ENV instruction".to_string(),
                    fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                    impact: self.impact(),
                });
            }
        }
        
        // Also check ARG for secrets
        for instruction in parser.get_instructions("ARG") {
            if SECRET_PATTERN.is_match(&instruction.arguments) {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: Some(instruction.line_number),
                    message: "Potential secret detected in ARG instruction (visible in image history)".to_string(),
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
    fn test_detects_password_in_env() {
        let content = "FROM alpine\nENV PASSWORD=mysecret123";
        let parser = DockerfileParser::parse_content(content);
        let rule = SecretsInEnvRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_detects_api_key_in_env() {
        let content = "FROM alpine\nENV API_KEY=abc123xyz";
        let parser = DockerfileParser::parse_content(content);
        let rule = SecretsInEnvRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_allows_regular_env() {
        let content = "FROM alpine\nENV NODE_ENV=production\nENV PORT=3000";
        let parser = DockerfileParser::parse_content(content);
        let rule = SecretsInEnvRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
