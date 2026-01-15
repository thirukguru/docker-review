use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;

pub struct LayerOrderRule;

impl Rule for LayerOrderRule {
    fn id(&self) -> &'static str { "DF004" }
    fn name(&self) -> &'static str { "Bad layer ordering" }
    fn severity(&self) -> Severity { Severity::Warning }
    
    fn description(&self) -> &'static str {
        "COPY/ADD of frequently changing files before package installation"
    }
    
    fn rationale(&self) -> &'static str {
        "Docker caches each layer. If you COPY source files before installing dependencies, \
         any source change invalidates the cache for dependency installation. Order layers \
         from least to most frequently changing: system packages → dependencies → source code."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Reorder: 1) System packages, 2) COPY dependency files (package.json, requirements.txt), 3) Install dependencies, 4) COPY source code")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: Some("Can reduce rebuild time by 60-90%".to_string()),
            image_size_reduction: None,
            security_improvement: None,
            reliability_improvement: Some("More consistent CI builds".to_string()),
        })
    }
}

impl DockerfileRule for LayerOrderRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Find COPY instructions that copy many files before RUN apt-get/npm/pip
        let mut found_broad_copy = false;
        let mut copy_line: Option<usize> = None;
        
        for instruction in &parser.instructions {
            match instruction.name.as_str() {
                "COPY" | "ADD" => {
                    let args = &instruction.arguments;
                    // Check if this copies "." or broad patterns (not just package files)
                    if args.contains(". ") || args.starts_with(". ") || args == "." {
                        // Copying entire context
                        found_broad_copy = true;
                        copy_line = Some(instruction.line_number);
                    } else if !is_dependency_file_copy(args) && !args.contains("--from=") {
                        // Not a dependency file copy and not multi-stage copy
                        found_broad_copy = true;
                        copy_line = Some(instruction.line_number);
                    }
                }
                "RUN" => {
                    if found_broad_copy {
                        let args = instruction.arguments.to_lowercase();
                        // Check if this is a package install command
                        if is_package_install(&args) {
                            issues.push(Issue {
                                rule_id: self.id().to_string(),
                                rule_name: self.name().to_string(),
                                severity: self.severity(),
                                line_number: copy_line,
                                message: "COPY of source files before package installation invalidates cache".to_string(),
                                fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                                impact: self.impact(),
                            });
                            break; // Only report once
                        }
                    }
                }
                _ => {}
            }
        }
        
        issues
    }
}

fn is_dependency_file_copy(args: &str) -> bool {
    let dependency_files = [
        "package.json",
        "package-lock.json",
        "yarn.lock",
        "requirements.txt",
        "Pipfile",
        "Pipfile.lock",
        "Gemfile",
        "Gemfile.lock",
        "go.mod",
        "go.sum",
        "Cargo.toml",
        "Cargo.lock",
        "pom.xml",
        "build.gradle",
        "composer.json",
        "composer.lock",
    ];
    
    dependency_files.iter().any(|f| args.contains(f))
}

fn is_package_install(args: &str) -> bool {
    args.contains("apt-get install")
        || args.contains("apk add")
        || args.contains("npm install")
        || args.contains("npm ci")
        || args.contains("yarn install")
        || args.contains("pip install")
        || args.contains("gem install")
        || args.contains("go mod download")
        || args.contains("cargo build")
        || args.contains("mvn install")
        || args.contains("gradle build")
        || args.contains("composer install")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_bad_order() {
        let content = r#"
FROM node:18
COPY . /app
RUN npm install
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = LayerOrderRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_allows_good_order() {
        let content = r#"
FROM node:18
COPY package.json package-lock.json ./
RUN npm install
COPY . .
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = LayerOrderRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
