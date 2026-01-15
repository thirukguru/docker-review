use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

static CURL_BASH_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(curl|wget)\s+[^\|]+\|\s*(sh|bash|zsh|/bin/sh|/bin/bash)").unwrap()
});

pub struct CurlBashRule;

impl Rule for CurlBashRule {
    fn id(&self) -> &'static str { "DF010" }
    fn name(&self) -> &'static str { "Curl pipe to shell" }
    fn severity(&self) -> Severity { Severity::Critical }
    
    fn description(&self) -> &'static str {
        "Piping curl or wget output directly to shell"
    }
    
    fn rationale(&self) -> &'static str {
        "Executing remote scripts via curl | bash is extremely dangerous. You cannot \
         verify what the script does before execution, the script could be modified \
         between when you test and when you deploy, and HTTPS doesn't prevent MITM \
         attacks if certificate validation is disabled."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Download the script, verify its contents and checksum, then execute")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: Some("Critical - prevents remote code execution vulnerabilities".to_string()),
            reliability_improvement: Some("Reproducible builds".to_string()),
        })
    }
}

impl DockerfileRule for CurlBashRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        for instruction in parser.get_instructions("RUN") {
            if CURL_BASH_PATTERN.is_match(&instruction.arguments) {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: Some(instruction.line_number),
                    message: "Piping curl/wget to shell - remote code execution risk".to_string(),
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
    fn test_detects_curl_bash() {
        let content = "FROM alpine\nRUN curl -sSL https://example.com/install.sh | bash";
        let parser = DockerfileParser::parse_content(content);
        let rule = CurlBashRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_detects_wget_sh() {
        let content = "FROM alpine\nRUN wget -qO- https://example.com/install.sh | sh";
        let parser = DockerfileParser::parse_content(content);
        let rule = CurlBashRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_allows_curl_to_file() {
        let content = "FROM alpine\nRUN curl -o /tmp/script.sh https://example.com/install.sh && chmod +x /tmp/script.sh && /tmp/script.sh";
        let parser = DockerfileParser::parse_content(content);
        let rule = CurlBashRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
