use super::{ComposeRule, Issue, Severity, ImpactEstimate};
use crate::parser::ComposeFile;
use crate::rules::Rule;

pub struct ComposeLatestTagRule;

impl Rule for ComposeLatestTagRule {
    fn id(&self) -> &'static str { "DC004" }
    fn name(&self) -> &'static str { "Using latest tag" }
    fn severity(&self) -> Severity { Severity::Critical }
    
    fn description(&self) -> &'static str {
        "Service uses 'latest' tag or implicit tag in image reference"
    }
    
    fn rationale(&self) -> &'static str {
        "Using 'latest' or implicit tags causes unpredictable deployments. \
         The same compose file may deploy different versions on different days. \
         Pin to specific versions for reproducible deployments."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Pin to a specific version tag (e.g., image: nginx:1.25.3)")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: Some("Prevents unexpected vulnerability introduction".to_string()),
            reliability_improvement: Some("100% reproducible deployments".to_string()),
        })
    }
}

impl ComposeRule for ComposeLatestTagRule {
    fn check(&self, compose: &ComposeFile) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if let Some(services) = &compose.services {
            for (name, service) in services {
                if let Some(image) = &service.image {
                    // Check for explicit :latest
                    if image.ends_with(":latest") {
                        issues.push(Issue {
                            rule_id: self.id().to_string(),
                            rule_name: self.name().to_string(),
                            severity: self.severity(),
                            line_number: None,
                            message: format!("Service '{}' uses image with ':latest' tag: {}", name, image),
                            fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                            impact: self.impact(),
                        });
                    }
                    // Check for missing tag (implicit latest)
                    else if !image.contains(':') && !image.contains('@') {
                        issues.push(Issue {
                            rule_id: self.id().to_string(),
                            rule_name: self.name().to_string(),
                            severity: self.severity(),
                            line_number: None,
                            message: format!("Service '{}' uses image without tag (implicitly 'latest'): {}", name, image),
                            fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                            impact: self.impact(),
                        });
                    }
                }
            }
        }
        
        issues
    }
}
