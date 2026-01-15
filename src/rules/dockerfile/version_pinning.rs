use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;
use regex::Regex;
use once_cell::sync::Lazy;

static UNPINNED_APT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"apt-get\s+install\s+(?:-[^\s]+\s+)*([a-zA-Z][a-zA-Z0-9._+-]*)(?:\s|$)").unwrap()
});

static UNPINNED_APK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"apk\s+add\s+(?:--[^\s]+\s+)*([a-zA-Z][a-zA-Z0-9._+-]*)(?:\s|$)").unwrap()
});

static UNPINNED_PIP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"pip\s+install\s+(?:-[^\s]+\s+)*([a-zA-Z][a-zA-Z0-9._-]*)(?:\s|$)").unwrap()
});

pub struct VersionPinningRule;

impl Rule for VersionPinningRule {
    fn id(&self) -> &'static str { "DF007" }
    fn name(&self) -> &'static str { "No version pinning" }
    fn severity(&self) -> Severity { Severity::Warning }
    
    fn description(&self) -> &'static str {
        "Packages installed without version pinning"
    }
    
    fn rationale(&self) -> &'static str {
        "Installing packages without version pinning leads to unpredictable builds. \
         The same Dockerfile may install different package versions on different days, \
         causing subtle bugs and security issues. Pin versions for reproducibility."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Pin package versions (e.g., 'apt-get install curl=7.68.0-1ubuntu2' or 'pip install requests==2.28.0')")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: None,
            security_improvement: Some("Prevents unexpected package changes".to_string()),
            reliability_improvement: Some("100% reproducible builds".to_string()),
        })
    }
}

impl DockerfileRule for VersionPinningRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        for instruction in parser.get_instructions("RUN") {
            let args = instruction.arguments.to_lowercase();
            
            // Check for unpinned apt-get packages
            if args.contains("apt-get install") && !args.contains("=") {
                if UNPINNED_APT.is_match(&args) {
                    issues.push(Issue {
                        rule_id: self.id().to_string(),
                        rule_name: self.name().to_string(),
                        severity: self.severity(),
                        line_number: Some(instruction.line_number),
                        message: "apt-get install without version pinning".to_string(),
                        fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                        impact: self.impact(),
                    });
                }
            }
            
            // Check for unpinned pip packages (not using -r requirements.txt)
            if args.contains("pip install") && !args.contains("-r ") && !args.contains("==") {
                if UNPINNED_PIP.is_match(&args) {
                    issues.push(Issue {
                        rule_id: self.id().to_string(),
                        rule_name: self.name().to_string(),
                        severity: self.severity(),
                        line_number: Some(instruction.line_number),
                        message: "pip install without version pinning".to_string(),
                        fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                        impact: self.impact(),
                    });
                }
            }
        }
        
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_unpinned_apt() {
        let content = "FROM ubuntu\nRUN apt-get update && apt-get install -y curl";
        let parser = DockerfileParser::parse_content(content);
        let rule = VersionPinningRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_detects_unpinned_pip() {
        let content = "FROM python\nRUN pip install requests flask";
        let parser = DockerfileParser::parse_content(content);
        let rule = VersionPinningRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_allows_pinned_pip() {
        let content = "FROM python\nRUN pip install requests==2.28.0";
        let parser = DockerfileParser::parse_content(content);
        let rule = VersionPinningRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_allows_requirements_file() {
        let content = "FROM python\nRUN pip install -r requirements.txt";
        let parser = DockerfileParser::parse_content(content);
        let rule = VersionPinningRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
