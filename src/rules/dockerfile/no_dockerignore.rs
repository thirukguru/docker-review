use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;
use std::path::Path;

pub struct NoDockerignoreRule;

impl Rule for NoDockerignoreRule {
    fn id(&self) -> &'static str { "DF003" }
    fn name(&self) -> &'static str { "No .dockerignore" }
    fn severity(&self) -> Severity { Severity::Warning }
    
    fn description(&self) -> &'static str {
        "No .dockerignore file found in the build context"
    }
    
    fn rationale(&self) -> &'static str {
        "Without a .dockerignore file, Docker copies all files in the build context, \
         including node_modules, .git, build artifacts, and other unnecessary files. \
         This bloats the image, slows down builds, and may leak sensitive files."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Create a .dockerignore file with patterns like: .git, node_modules, *.log, .env")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: Some("Can reduce build context transfer by 50-90%".to_string()),
            image_size_reduction: Some("Can reduce image size by 20-80%".to_string()),
            security_improvement: Some("Prevents accidental inclusion of secrets".to_string()),
            reliability_improvement: None,
        })
    }
}

impl DockerfileRule for NoDockerignoreRule {
    fn check(&self, _parser: &DockerfileParser, context_dir: Option<&Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        if let Some(dir) = context_dir {
            if !crate::parser::check_dockerignore(dir) {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: None,
                    message: "No .dockerignore file found in the build context".to_string(),
                    fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                    impact: self.impact(),
                });
            }
        }
        
        issues
    }
}
