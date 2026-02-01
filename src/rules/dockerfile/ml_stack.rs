use super::{DockerfileRule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;
use crate::rules::Rule;
use crate::detector::{MLStack, MLStackDetector};

pub struct MLStackRule;

impl Rule for MLStackRule {
    fn id(&self) -> &'static str { "DF012" }
    fn name(&self) -> &'static str { "ML stack optimization" }
    fn severity(&self) -> Severity { Severity::Suggestion }
    
    fn description(&self) -> &'static str {
        "Detect ML/AI frameworks and suggest optimized base images"
    }
    
    fn rationale(&self) -> &'static str {
        "ML frameworks like PyTorch, TensorFlow, and Transformers benefit significantly from \
         using pre-built, optimized Docker images. These images come with proper CUDA/cuDNN \
         configurations, optimized library builds, and are regularly maintained. Using generic \
         Python images for ML workloads often leads to larger images, longer build times, \
         and potential compatibility issues with GPU drivers."
    }
    
    fn fix_suggestion(&self) -> Option<&'static str> {
        Some("Use an official ML framework base image (e.g., pytorch/pytorch, tensorflow/tensorflow)")
    }
    
    fn impact(&self) -> Option<ImpactEstimate> {
        Some(ImpactEstimate {
            build_time_improvement: Some("30-60% faster builds with pre-compiled binaries".to_string()),
            image_size_reduction: Some("Up to 2-5GB smaller with slim ML images".to_string()),
            security_improvement: Some("Regular security updates from maintainers".to_string()),
            reliability_improvement: Some("Tested CUDA/cuDNN compatibility".to_string()),
        })
    }
}

impl DockerfileRule for MLStackRule {
    fn check(&self, parser: &DockerfileParser, _context_dir: Option<&std::path::Path>) -> Vec<Issue> {
        let mut issues = Vec::new();
        
        // Detect ML stacks
        let detected_stacks = MLStackDetector::detect(parser);
        
        if detected_stacks.is_empty() {
            return issues;
        }
        
        // Check if already using an ML-optimized base image
        if MLStackDetector::has_ml_base_image(parser) {
            return issues;
        }
        
        // Check if using generic Python image
        if MLStackDetector::uses_generic_python_image(parser) {
            let stacks_str = detected_stacks.iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            
            // Get the first FROM instruction line number
            let line_number = parser.get_instructions("FROM")
                .first()
                .map(|i| i.line_number);
            
            // Build recommendation
            let primary_stack = &detected_stacks[0];
            let recommended = primary_stack.recommended_base_image();
            let cpu_alternative = primary_stack.recommended_cpu_image();
            
            issues.push(Issue {
                rule_id: self.id().to_string(),
                rule_name: self.name().to_string(),
                severity: self.severity(),
                line_number,
                message: format!(
                    "Detected {} stack(s) with generic Python image. Consider using an optimized base image.",
                    stacks_str
                ),
                fix_suggestion: Some(format!(
                    "For GPU: FROM {}\nFor CPU: FROM {}",
                    recommended, cpu_alternative
                )),
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
    fn test_detects_pytorch_with_generic_image() {
        let content = r#"
FROM python:3.10
RUN pip install torch torchvision
COPY . /app
CMD ["python", "train.py"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = MLStackRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("PyTorch"));
    }

    #[test]
    fn test_no_issue_with_pytorch_image() {
        let content = r#"
FROM pytorch/pytorch:2.0.0-cuda11.7-cudnn8-runtime
RUN pip install torchvision
COPY . /app
CMD ["python", "train.py"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = MLStackRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_no_issue_for_non_ml_project() {
        let content = r#"
FROM python:3.10
RUN pip install flask requests
COPY . /app
CMD ["python", "app.py"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = MLStackRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_detects_multiple_frameworks() {
        let content = r#"
FROM python:3.10
RUN pip install torch transformers datasets
COPY . /app
CMD ["python", "train.py"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let rule = MLStackRule;
        let issues = rule.check(&parser, None);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("PyTorch"));
        assert!(issues[0].message.contains("Transformers"));
    }
}
