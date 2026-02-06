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
	{Name: "TensorFlow", Packages: []string{"tensorflow", "keras"}, Suggestion: "Consider tensorflow/tensorflow:gpu"},
	{Name: "PyTorch", Packages: []string{"torch", "torchvision"}, Suggestion: "Consider pytorch/pytorch"},
	{Name: "JAX", Packages: []string{"jax", "jaxlib"}, Suggestion: "Consider gcr.io/jax-releases/jax"},
	{Name: "Hugging Face", Packages: []string{"transformers", "datasets"}, Suggestion: "Consider huggingface/transformers-pytorch-gpu"},
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
