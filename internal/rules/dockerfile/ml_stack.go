package dockerfile

import (
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// MLStackRule suggests ML-optimized base images (DF012)
type MLStackRule struct {
	types.BaseRule
}

func NewMLStackRule() *MLStackRule {
	return &MLStackRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF012",
			RuleName:        "ML stack optimization",
			RuleSeverity:    types.Suggestion,
			RuleCategory:    "performance",
			RuleDescription: "ML framework detected. Consider using optimized base images.",
			RuleFix:         "Use official ML images (tensorflow/tensorflow:gpu, pytorch/pytorch)",
		},
	}
}

type mlFramework struct {
	Name       string
	Packages   []string
	Suggestion string
}

var mlFrameworks = []mlFramework{
	// Deep Learning
	{Name: "TensorFlow", Packages: []string{"tensorflow", "keras"}, Suggestion: "Consider tensorflow/tensorflow:gpu"},
	{Name: "PyTorch", Packages: []string{"torch", "torchvision", "torchaudio"}, Suggestion: "Consider pytorch/pytorch"},
	{Name: "JAX", Packages: []string{"jax", "jaxlib", "flax"}, Suggestion: "Consider gcr.io/jax-releases/jax"},
	{Name: "Hugging Face", Packages: []string{"transformers", "datasets", "accelerate"}, Suggestion: "Consider huggingface/transformers-pytorch-gpu"},
	{Name: "FastAI", Packages: []string{"fastai"}, Suggestion: "Consider fastdotai/fastai"},
	{Name: "MXNet", Packages: []string{"mxnet"}, Suggestion: "Consider mxnet/python"},

	// Classical ML
	{Name: "scikit-learn", Packages: []string{"scikit-learn", "sklearn"}, Suggestion: "Consider jupyter/scipy-notebook"},
	{Name: "XGBoost", Packages: []string{"xgboost"}, Suggestion: "Consider jupyter/scipy-notebook"},
	{Name: "LightGBM", Packages: []string{"lightgbm"}, Suggestion: "Consider jupyter/scipy-notebook"},
	{Name: "CatBoost", Packages: []string{"catboost"}, Suggestion: "Consider jupyter/scipy-notebook"},

	// Data Science
	{Name: "Pandas", Packages: []string{"pandas", "numpy", "scipy"}, Suggestion: "Consider jupyter/datascience-notebook"},
	{Name: "Polars", Packages: []string{"polars"}, Suggestion: "Consider python:3.11-slim"},

	// Computer Vision
	{Name: "OpenCV", Packages: []string{"opencv-python", "cv2"}, Suggestion: "Consider jjanzic/docker-python3-opencv"},
	{Name: "Detectron2", Packages: []string{"detectron2"}, Suggestion: "Consider pytorch/pytorch with CUDA"},

	// NLP
	{Name: "SpaCy", Packages: []string{"spacy"}, Suggestion: "Consider spacyio/spacy"},
	{Name: "NLTK", Packages: []string{"nltk"}, Suggestion: "Consider jupyter/scipy-notebook"},

	// MLOps
	{Name: "MLflow", Packages: []string{"mlflow"}, Suggestion: "Consider ghcr.io/mlflow/mlflow"},
	{Name: "ONNX", Packages: []string{"onnx", "onnxruntime"}, Suggestion: "Consider mcr.microsoft.com/onnxruntime"},
	{Name: "TensorRT", Packages: []string{"tensorrt"}, Suggestion: "Consider nvcr.io/nvidia/tensorrt"},
}

func (r *MLStackRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue
	detected := make(map[string]bool)

	for _, instr := range ctx.Instructions {
		if instr.Name != "RUN" {
			continue
		}

		args := strings.ToLower(instr.Arguments)
		if !strings.Contains(args, "pip install") {
			continue
		}

		for _, fw := range mlFrameworks {
			for _, pkg := range fw.Packages {
				if strings.Contains(args, pkg) && !detected[fw.Name] {
					detected[fw.Name] = true
					issues = append(issues, types.Issue{
						RuleID:      r.ID(),
						Name:        r.Name(),
						Severity:    r.Severity(),
						Line:        instr.Line,
						Description: fw.Name + " detected: " + fw.Suggestion,
						Fix:         r.Fix(),
						Category:    r.Category(),
					})
					break
				}
			}
		}
	}

	return issues
}
