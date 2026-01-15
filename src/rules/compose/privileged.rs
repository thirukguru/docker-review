use super::{ComposeRule, Issue, Severity, ImpactEstimate};
use crate::parser::ComposeFile;
use crate::rules::Rule;

pub struct PrivilegedRule;

impl Rule for PrivilegedRule {
    fn id(&self) -> &'static str { "DC002" }
    fn name(&self) -> &'static str { "Privileged container" }
    fn severity(&self) -> Severity { Severity::Critical }
    
    fn description(&self) -> &'static str {
        "Service runs in privileged mode"
    }
    
    fn rationale(&self) -> &'static str {
        "Privileged mode gives the container full access to the host system, \
         bypassing all security isolation. A compromised privileged container \
         can trivially escape to the host. Only use if absolutely necessary."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Remove 'privileged: true' and use specific capabilities (cap_add) if needed")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: Some("Critical - prevents container escape".to_string()),
            reliability_improvement: None,
        })
    }
}

impl ComposeRule for PrivilegedRule {
    fn check(&self, compose: &ComposeFile) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if let Some(services) = &compose.services {
            for (name, service) in services {
                if service.privileged == Some(true) {
                    issues.push(Issue {
                        rule_id: self.id().to_string(),
                        rule_name: self.name().to_string(),
                        severity: self.severity(),
                        line_number: None,
                        message: format!("Service '{}' runs in privileged mode", name),
                        fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                        impact: self.impact(),
                    });
                }
            }
        }
        
        issues
    }
}
