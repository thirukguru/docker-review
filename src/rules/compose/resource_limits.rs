use super::{ComposeRule, Issue, Severity, ImpactEstimate};
use crate::parser::ComposeFile;
use crate::rules::Rule;

pub struct ResourceLimitsRule;

impl Rule for ResourceLimitsRule {
    fn id(&self) -> &'static str { "DC003" }
    fn name(&self) -> &'static str { "No resource limits" }
    fn severity(&self) -> Severity { Severity::Warning }
    
    fn description(&self) -> &'static str {
        "Service has no memory or CPU limits defined"
    }
    
    fn rationale(&self) -> &'static str {
        "Without resource limits, a single container can consume all host resources, \
         starving other containers and potentially crashing the host. Always set \
         memory and CPU limits in production."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Add 'deploy.resources.limits' or 'mem_limit' and 'cpus' to the service")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: Some("Prevents denial of service".to_string()),
            reliability_improvement: Some("Prevents resource exhaustion".to_string()),
        })
    }
}

impl ComposeRule for ResourceLimitsRule {
    fn check(&self, compose: &ComposeFile) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if let Some(services) = &compose.services {
            for (name, service) in services {
                // Check for resource limits (v2 or v3 format)
                let has_limits = service.mem_limit.is_some()
                    || service.cpus.is_some()
                    || service.deploy.as_ref()
                        .and_then(|d| d.resources.as_ref())
                        .and_then(|r| r.limits.as_ref())
                        .is_some();
                
                if !has_limits {
                    issues.push(Issue {
                        rule_id: self.id().to_string(),
                        rule_name: self.name().to_string(),
                        severity: self.severity(),
                        line_number: None,
                        message: format!("Service '{}' has no resource limits", name),
                        fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                        impact: self.impact(),
                    });
                }
            }
        }
        
        issues
    }
}
