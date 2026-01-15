use super::{ComposeRule, Issue, Severity, ImpactEstimate};
use crate::parser::ComposeFile;
use crate::rules::Rule;

pub struct RestartPolicyRule;

impl Rule for RestartPolicyRule {
    fn id(&self) -> &'static str { "DC001" }
    fn name(&self) -> &'static str { "No restart policy" }
    fn severity(&self) -> Severity { Severity::Warning }
    
    fn description(&self) -> &'static str {
        "Service has no restart policy defined"
    }
    
    fn rationale(&self) -> &'static str {
        "Without a restart policy, crashed containers stay down. This leads to \
         service outages that require manual intervention. Use 'unless-stopped' \
         or 'always' for production services."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Add 'restart: unless-stopped' or 'restart: always' to the service")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: None,
            reliability_improvement: Some("Automatic recovery from crashes".to_string()),
        })
    }
}

impl ComposeRule for RestartPolicyRule {
    fn check(&self, compose: &ComposeFile) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if let Some(services) = &compose.services {
            for (name, service) in services {
                // Check for restart policy in service or deploy section
                let has_restart = service.restart.is_some() 
                    || service.deploy.as_ref()
                        .and_then(|d| d.restart_policy.as_ref())
                        .is_some();
                
                if !has_restart {
                    issues.push(Issue {
                        rule_id: self.id().to_string(),
                        rule_name: self.name().to_string(),
                        severity: self.severity(),
                        line_number: None,
                        message: format!("Service '{}' has no restart policy", name),
                        fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                        impact: self.impact(),
                    });
                }
            }
        }
        
        issues
    }
}
