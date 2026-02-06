package dockerfile

import (
	"fmt"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// LayerOptimizationRule detects inefficient layer usage (DF011)
type LayerOptimizationRule struct {
	types.BaseRule
}

func NewLayerOptimizationRule() *LayerOptimizationRule {
	return &LayerOptimizationRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF011",
			RuleName:        "Inefficient layer usage",
			RuleSeverity:    types.Warning,
			RuleCategory:    "performance",
			RuleDescription: "Multiple RUN commands that could be combined, or install without cleanup.",
			RuleFix:         "Combine RUN commands with && and clean up in the same layer.",
		},
	}
}

func (r *LayerOptimizationRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	consecutiveRuns := 0
	var firstRunLine int

	for _, instr := range ctx.Instructions {
		if instr.Name == "RUN" {
			if consecutiveRuns == 0 {
				firstRunLine = instr.Line
			}
			consecutiveRuns++

			args := strings.ToLower(instr.Arguments)
			if strings.Contains(args, "apt-get install") || strings.Contains(args, "apt install") {
				if !strings.Contains(args, "rm -rf") && !strings.Contains(args, "apt-get clean") {
					issues = append(issues, types.Issue{
						RuleID:      r.ID(),
						Name:        r.Name(),
						Severity:    r.Severity(),
						Line:        instr.Line,
						Description: "apt-get install without cleanup in the same layer",
						Fix:         "Add 'rm -rf /var/lib/apt/lists/*' in the same RUN command",
						Category:    r.Category(),
					})
				}
			}
		} else {
			if consecutiveRuns >= 3 {
				issues = append(issues, types.Issue{
					RuleID:      r.ID(),
					Name:        r.Name(),
					Severity:    r.Severity(),
					Line:        firstRunLine,
					Description: fmt.Sprintf("%d consecutive RUN commands could be combined", consecutiveRuns),
					Fix:         "Combine related RUN commands using && to reduce layer count",
					Category:    r.Category(),
				})
			}
			consecutiveRuns = 0
		}
	}

	return issues
}
