package detector

import (
	"strings"
)

// MLFramework represents a detected ML framework
type MLFramework struct {
	Name       string
	Confidence float64
	Suggestion string
}

// DetectMLStack detects ML frameworks in a Dockerfile
func DetectMLStack(content string) []MLFramework {
	var detected []MLFramework
	lower := strings.ToLower(content)

	frameworks := []struct {
		name       string
		patterns   []string
		suggestion string
	}{
		{
			name:       "TensorFlow",
			patterns:   []string{"tensorflow", "tf-nightly", "keras"},
			suggestion: "Use tensorflow/tensorflow:latest-gpu for GPU support",
		},
		{
			name:       "PyTorch",
			patterns:   []string{"torch", "torchvision", "torchaudio"},
			suggestion: "Use pytorch/pytorch for optimized PyTorch",
		},
		{
			name:       "JAX",
			patterns:   []string{"jax", "jaxlib", "flax"},
			suggestion: "Use gcr.io/jax-releases/jax for optimized JAX",
		},
		{
			name:       "Hugging Face",
			patterns:   []string{"transformers", "datasets", "accelerate"},
			suggestion: "Use huggingface/transformers-pytorch-gpu",
		},
	}

	for _, fw := range frameworks {
		for _, pattern := range fw.patterns {
			if strings.Contains(lower, pattern) {
				detected = append(detected, MLFramework{
					Name:       fw.name,
					Confidence: 0.9,
					Suggestion: fw.suggestion,
				})
				break
			}
		}
	}

	return detected
}
