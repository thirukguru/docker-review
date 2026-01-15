use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;

pub struct MultistageRule;

impl Rule for MultistageRule {
    fn id(&self) -> &'static str { "DF008" }
    fn name(&self) -> &'static str { "Missing multi-stage build" }
    fn severity(&self) -> Severity { Severity::Suggestion }
    
    fn description(&self) -> &'static str {
        "Consider using multi-stage builds for compiled languages"
    }
    
    fn rationale(&self) -> &'static str {
        "For compiled languages (Go, Rust, Java, etc.), multi-stage builds dramatically \
         reduce image size by separating build dependencies from runtime. The final image \
         only contains the compiled binary, not compilers, build tools, or source code."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Use multi-stage build: compile in one stage, copy binary to minimal runtime image")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: None,
            image_size_reduction: Some("Can reduce image size by 80-95%".to_string()),
            security_improvement: Some("Smaller attack surface".to_string()),
            reliability_improvement: None,
        })
    }
}

impl DockerfileRule for MultistageRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        let from_count = parser.get_instructions("FROM").len();
        
        // Only suggest if single-stage build
        if from_count <= 1 {
            let has_compile = parser.instructions.iter().any(|i| {
                if i.name != "RUN" {
                    return false;
                }
                let args = i.arguments.to_lowercase();
                // Check for compilation commands
                args.contains("go build")
                    || args.contains("cargo build")
                    || args.contains("mvn package")
                    || args.contains("gradle build")
                    || args.contains("npm run build")
                    || args.contains("yarn build")
                    || args.contains("make ")
                    || args.contains("gcc ")
                    || args.contains("g++ ")
                    || args.contains("rustc ")
                    || args.contains("javac ")
            });
            
            if has_compile {
                issues.push(Issue {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    line_number: None,
                    message: "Single-stage build with compilation detected - consider multi-stage build".to_string(),
                    fix_suggestion: self.fix_suggestion().map(|s| s.to_string()),
                    impact: self.impact(),
                });
            }
        }
        
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggests_multistage_for_go() {
        let content = "FROM golang:1.21\nWORKDIR /app\nCOPY . .\nRUN go build -o main\nCMD [\"./main\"]";
        let parser = DockerfileParser::parse_content(content);
        let rule = MultistageRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn test_no_suggestion_for_multistage() {
        let content = r#"
FROM golang:1.21 AS builder
WORKDIR /app
COPY . .
RUN go build -o main

FROM alpine:3.18
COPY --from=builder /app/main /main
CMD ["/main"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = MultistageRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_no_suggestion_without_compile() {
        let content = "FROM node:18\nWORKDIR /app\nCOPY . .\nRUN npm install\nCMD [\"node\", \"app.js\"]";
        let parser = DockerfileParser::parse_content(content);
        let rule = MultistageRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }
}
