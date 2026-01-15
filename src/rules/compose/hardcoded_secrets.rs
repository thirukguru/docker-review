use super::{ComposeRule, Issue, Severity, ImpactEstimate};
use crate::parser::{ComposeFile, Environment};
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

static SECRET_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(password|passwd|secret|api_key|apikey|auth_token|access_token|private_key|credentials?)").unwrap()
});

pub struct HardcodedSecretsRule;

impl Rule for HardcodedSecretsRule {
    fn id(&self) -> &'static str { "DC005" }
    fn name(&self) -> &'static str { "Hardcoded secrets" }
    fn severity(&self) -> Severity { Severity::Critical }
    
    fn description(&self) -> &'static str {
        "Service has hardcoded secrets in environment variables"
    }
    
    fn rationale(&self) -> &'static str {
        "Hardcoded secrets in docker-compose files are stored in version control, \
         visible to anyone with repo access, and cannot be rotated without updating \
         the file. Use environment variables, env_file, or Docker secrets."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Use env_file, Docker secrets, or environment variable substitution (${VAR})")
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

impl ComposeRule for HardcodedSecretsRule {
    fn check(&self, compose: &ComposeFile) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if let Some(services) = &compose.services {
            for (name, service) in services {
                if let Some(env) = &service.environment {
                    let has_hardcoded_secret = match env {
                        Environment::List(list) => {
                            list.iter().any(|item| {
                                // Check if this is a secret and has a value (not a reference)
                                if SECRET_PATTERN.is_match(item) {
                                    // Check if it's KEY=VALUE (not just KEY for env passthrough)
                                    if item.contains('=') && !item.contains("${") {
                                        let value = item.split('=').nth(1).unwrap_or("");
                                        return !value.is_empty();
                                    }
                                }
                                false
                            })
                        }
                        Environment::Map(map) => {
                            map.iter().any(|(key, value)| {
                                if SECRET_PATTERN.is_match(key) {
                                    // Check if value is hardcoded (not a reference)
                                    if let Some(val) = value {
                                        return !val.is_empty() && !val.contains("${");
                                    }
                                }
                                false
                            })
                        }
                    };
                    
                    if has_hardcoded_secret {
                        issues.push(Issue {
                            rule_id: self.id().to_string(),
                            rule_name: self.name().to_string(),
                            severity: self.severity(),
                            line_number: None,
                            message: format!("Service '{}' has hardcoded secret in environment", name),
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
