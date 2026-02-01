use crate::parser::DockerfileParser;
use regex::Regex;
use once_cell::sync::Lazy;

/// ML/AI framework stack types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MLStack {
    PyTorch,
    TensorFlow,
    Transformers,
    VLLM,
    JAX,
}

impl std::fmt::Display for MLStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MLStack::PyTorch => write!(f, "PyTorch"),
            MLStack::TensorFlow => write!(f, "TensorFlow"),
            MLStack::Transformers => write!(f, "Transformers/HuggingFace"),
            MLStack::VLLM => write!(f, "vLLM"),
            MLStack::JAX => write!(f, "JAX"),
        }
    }
}

impl MLStack {
    /// Get recommended base image for this ML stack
    pub fn recommended_base_image(&self) -> &'static str {
        match self {
            MLStack::PyTorch => "pytorch/pytorch:2.1.0-cuda12.1-cudnn8-runtime",
            MLStack::TensorFlow => "tensorflow/tensorflow:2.15.0-gpu",
            MLStack::Transformers => "huggingface/transformers-pytorch-gpu:latest",
            MLStack::VLLM => "vllm/vllm-openai:latest",
            MLStack::JAX => "python:3.11-slim",  // JAX doesn't have official image
        }
    }
    
    /// Get CPU-only recommended base image
    pub fn recommended_cpu_image(&self) -> &'static str {
        match self {
            MLStack::PyTorch => "pytorch/pytorch:2.1.0-cpu",
            MLStack::TensorFlow => "tensorflow/tensorflow:2.15.0",
            MLStack::Transformers => "huggingface/transformers-pytorch-cpu:latest",
            MLStack::VLLM => "python:3.11-slim",  // vLLM really needs GPU
            MLStack::JAX => "python:3.11-slim",
        }
    }
}

// Detection patterns for ML frameworks
static PYTORCH_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(torch|pytorch|torchvision|torchaudio)\b").unwrap()
});

static TENSORFLOW_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(tensorflow|tensorflow-gpu|tf-nightly|keras)\b").unwrap()
});

static TRANSFORMERS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(transformers|huggingface|datasets|tokenizers|accelerate|peft)\b").unwrap()
});

static VLLM_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\bvllm\b").unwrap()
});

static JAX_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(jax|flax|optax)\b").unwrap()
});

/// Detects ML/AI frameworks in Dockerfiles
pub struct MLStackDetector;

impl MLStackDetector {
    /// Detect ML frameworks used in the Dockerfile
    pub fn detect(parser: &DockerfileParser) -> Vec<MLStack> {
        let mut detected = Vec::new();
        
        // Collect all RUN instructions that might contain pip install
        let content = Self::get_install_commands(parser);
        
        // Check for each framework
        if PYTORCH_RE.is_match(&content) {
            detected.push(MLStack::PyTorch);
        }
        
        if TENSORFLOW_RE.is_match(&content) {
            detected.push(MLStack::TensorFlow);
        }
        
        if TRANSFORMERS_RE.is_match(&content) {
            detected.push(MLStack::Transformers);
        }
        
        if VLLM_RE.is_match(&content) {
            detected.push(MLStack::VLLM);
        }
        
        if JAX_RE.is_match(&content) {
            detected.push(MLStack::JAX);
        }
        
        // Also check COPY commands for requirements.txt content references
        // and FROM instructions for ML base images
        let from_content = Self::get_from_images(parser);
        
        if from_content.contains("pytorch") {
            if !detected.contains(&MLStack::PyTorch) {
                detected.push(MLStack::PyTorch);
            }
        }
        
        if from_content.contains("tensorflow") {
            if !detected.contains(&MLStack::TensorFlow) {
                detected.push(MLStack::TensorFlow);
            }
        }
        
        if from_content.contains("huggingface") || from_content.contains("transformers") {
            if !detected.contains(&MLStack::Transformers) {
                detected.push(MLStack::Transformers);
            }
        }
        
        if from_content.contains("vllm") {
            if !detected.contains(&MLStack::VLLM) {
                detected.push(MLStack::VLLM);
            }
        }
        
        detected
    }
    
    /// Check if the Dockerfile already uses an ML-optimized base image
    pub fn has_ml_base_image(parser: &DockerfileParser) -> bool {
        let from_images = Self::get_from_images(parser);
        let ml_images = [
            "pytorch/pytorch",
            "tensorflow/tensorflow",
            "huggingface/",
            "nvidia/cuda",
            "nvcr.io/nvidia",
            "vllm/vllm",
        ];
        
        ml_images.iter().any(|img| from_images.contains(img))
    }
    
    /// Check if using a generic Python image
    pub fn uses_generic_python_image(parser: &DockerfileParser) -> bool {
        for instruction in parser.get_instructions("FROM") {
            let image = instruction.arguments.split_whitespace().next().unwrap_or("");
            if image.starts_with("python:") || image == "python" {
                return true;
            }
        }
        false
    }
    
    fn get_install_commands(parser: &DockerfileParser) -> String {
        parser.get_instructions("RUN")
            .iter()
            .map(|i| i.arguments.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    fn get_from_images(parser: &DockerfileParser) -> String {
        parser.get_instructions("FROM")
            .iter()
            .map(|i| i.arguments.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_pytorch() {
        let content = r#"
FROM python:3.10
RUN pip install torch torchvision
COPY . /app
CMD ["python", "train.py"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let stacks = MLStackDetector::detect(&parser);
        assert!(stacks.contains(&MLStack::PyTorch));
    }

    #[test]
    fn test_detect_tensorflow() {
        let content = r#"
FROM python:3.10
RUN pip install tensorflow-gpu keras
COPY . /app
CMD ["python", "train.py"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let stacks = MLStackDetector::detect(&parser);
        assert!(stacks.contains(&MLStack::TensorFlow));
    }

    #[test]
    fn test_detect_transformers() {
        let content = r#"
FROM python:3.10
RUN pip install transformers datasets
COPY . /app
CMD ["python", "train.py"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let stacks = MLStackDetector::detect(&parser);
        assert!(stacks.contains(&MLStack::Transformers));
    }

    #[test]
    fn test_detect_from_base_image() {
        let content = r#"
FROM pytorch/pytorch:2.0.0-cuda11.7-cudnn8-runtime
COPY . /app
CMD ["python", "train.py"]
"#;
        let parser = DockerfileParser::parse_content(content);
        let stacks = MLStackDetector::detect(&parser);
        assert!(stacks.contains(&MLStack::PyTorch));
        assert!(MLStackDetector::has_ml_base_image(&parser));
    }

    #[test]
    fn test_uses_generic_python_image() {
        let content = r#"
FROM python:3.10
RUN pip install torch
"#;
        let parser = DockerfileParser::parse_content(content);
        assert!(MLStackDetector::uses_generic_python_image(&parser));
    }
}
