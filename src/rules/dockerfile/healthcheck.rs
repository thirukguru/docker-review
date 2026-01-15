use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;

pub struct HealthcheckRule;

impl Rule for HealthcheckRule {
    fn id(&self) -> &'static str { "DF005" }
    fn name(&self) -> &'static str { "No HEALTHCHECK" }
    fn severity(&self) -> Severity { Severity::Warning }
    
    fn description(&self) -> &'static str {
        "No HEALTHCHECK instruction defined"
    }
    
    fn rationale(&self) -> &'static str {
        "Without a HEALTHCHECK, Docker and orchestrators cannot determine if your \
         application is actually healthy. A container may be running but unresponsive. \
         HEALTHCHECK enables automatic restarts and proper load balancing."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Add HEALTHCHECK instruction (e.g., HEALTHCHECK --interval=30s CMD curl -f http://localhost/ || exit 1)")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: None,
            reliability_improvement: Some("Enables automatic container recovery".to_string()),
        })
    }
}

impl DockerfileRule for HealthcheckRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if !parser.has_instruction("HEALTHCHECK") {
            issues.push(Issue {
                rule_id: self.id().to_string(),
                rule_name: self.name().to_string(),
                severity: self.severity(),
                line_number: None,
                message: "No HEALTHCHECK instruction - container health cannot be monitored".to_string(),
                fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                impact: self.impact(),
            });
        }
        
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_no_healthcheck() {
        let content = "FROM alpine\nCMD [\"./app\"]";
        let parser = DockerfileParser::parse_content(content);
        let rule = HealthcheckRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_allows_with_healthcheck() {
        let content = "FROM alpine\nHEALTHCHECK CMD curl -f http://localhost/\nCMD [\"./app\"]";
        let parser = DockerfileParser::parse_content(content);
        let rule = HealthcheckRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
