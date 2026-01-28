use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;

pub struct LayerOptimizationRule;

impl Rule for LayerOptimizationRule {
    fn id(&self) -> &'static str { "DF011" }
    fn name(&self) -> &'static str { "Inefficient layer usage" }
    fn severity(&self) -> Severity { Severity::Warning }
    
    fn description(&self) -> &'static str {
        "Inefficient Dockerfile layering wastes space and slows builds"
    }
    
    fn rationale(&self) -> &'static str {
        "Every RUN instruction creates a layer. When apt-get clean or rm -rf runs in a \
         separate layer from the install, space is NOT saved - files are already persisted \
         in the previous layer. Combine related operations in a single RUN and clean up \
         in the same layer. Use --no-install-recommends to skip unnecessary packages."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Combine apt-get update, install, and cleanup in single RUN with && and use --no-install-recommends")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: Some("Fewer layers = faster builds".to_string()),
            image_size_reduction: Some("Proper cleanup can save 50-200MB".to_string()),
            security_improvement: Some("Smaller attack surface".to_string()),
            reliability_improvement: None,
        })
    }
}

impl DockerfileRule for LayerOptimizationRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        let mut apt_install_line: Option<usize> = None;
        let mut has_apt_install = false;
        let mut has_no_install_recommends = false;
        
        for instruction in &parser.instructions {
            if instruction.name != "RUN" {
                continue;
            }
            
            let args = instruction.arguments.to_lowercase();
            
            // Track apt-get install
            if args.contains("apt-get install") {
                has_apt_install = true;
                apt_install_line = Some(instruction.line_number);
                
                if args.contains("--no-install-recommends") {
                    has_no_install_recommends = true;
                }
                
                // Check if cleanup is in the same RUN command (good)
                let has_cleanup_same_layer = args.contains("rm -rf /var/lib/apt/lists") 
                    || args.contains("apt-get clean");
                
                if !has_cleanup_same_layer {
                    // Will check if cleanup is in a later separate RUN
                }
            }
            
            // Detect cleanup in separate layer (bad pattern)
            if (args.contains("apt-get clean") || args.contains("rm -rf /var/lib/apt/lists"))
                && !args.contains("apt-get install")
            {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: Some(instruction.line_number),
                    message: "Cleanup in separate layer doesn't save space - combine with install command".to_string(),
                    fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                    impact: self.impact(),
                });
            }
            
            // Detect multiple consecutive apt-get commands (apt-get update separate from install)
            if args.contains("apt-get update") && !args.contains("apt-get install") {
                // apt-get update alone in a RUN - check next instruction
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: Some(instruction.line_number),
                    message: "apt-get update in separate RUN - combine with apt-get install using &&".to_string(),
                    fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                    impact: self.impact(),
                });
            }
        }
        
        // Check for missing --no-install-recommends
        if has_apt_install && !has_no_install_recommends {
            issues.push(Issue {
                rule_id: self.id().to_string(),
                rule_name: self.name().to_string(),
                severity: Severity::Suggestion,
                line_number: apt_install_line,
                message: "apt-get install without --no-install-recommends may install unnecessary packages".to_string(),
                fix_suggestion: Some("Add --no-install-recommends flag to apt-get install".to_string()),
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
    fn test_detects_separate_cleanup_layer() {
        let content = r#"
FROM ubuntu:20.04
RUN apt-get update && apt-get install -y curl
RUN apt-get clean
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = LayerOptimizationRule;
        let issues = rule.check(&parser, None);
        // Should detect: separate cleanup layer + missing --no-install-recommends
        assert!(issues.iter().any(|i| i.message.contains("separate layer")));
    }

    #[test]
    fn test_detects_separate_apt_update() {
        let content = r#"
FROM ubuntu:20.04
RUN apt-get update
RUN apt-get install -y curl
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = LayerOptimizationRule;
        let issues = rule.check(&parser, None);
        assert!(issues.iter().any(|i| i.message.contains("apt-get update in separate RUN")));
    }

    #[test]
    fn test_detects_missing_no_install_recommends() {
        let content = r#"
FROM ubuntu:20.04
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = LayerOptimizationRule;
        let issues = rule.check(&parser, None);
        assert!(issues.iter().any(|i| i.message.contains("--no-install-recommends")));
    }

    #[test]
    fn test_allows_good_pattern() {
        let content = r#"
FROM ubuntu:20.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl && \
    rm -rf /var/lib/apt/lists/*
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = LayerOptimizationRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_no_issues_without_apt() {
        let content = r#"
FROM node:18-alpine
RUN npm install
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = LayerOptimizationRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
