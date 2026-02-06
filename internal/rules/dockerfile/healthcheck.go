package dockerfile

import (
	"github.com/thirukguru/docker-review/internal/types"
)

// HealthcheckRule checks for HEALTHCHECK instruction (DF005)
type HealthcheckRule struct {
	types.BaseRule
}

func NewHealthcheckRule() *HealthcheckRule {
	return &HealthcheckRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF005",
			RuleName:        "No HEALTHCHECK",
			RuleSeverity:    types.Warning,
			RuleCategory:    "maintainability",
			RuleDescription: "No HEALTHCHECK instruction. Container health cannot be monitored.",
			RuleFix:         "Add HEALTHCHECK instruction (e.g., HEALTHCHECK CMD curl -f http://localhost/ || exit 1)",
		},
	}
}

func (r *HealthcheckRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue
	hasHealthcheck := false

	for _, instr := range ctx.Instructions {
		if instr.Name == "HEALTHCHECK" {
			hasHealthcheck = true
			break
		}
	}

	if !hasHealthcheck && len(ctx.Instructions) > 0 {
		issues = append(issues, types.Issue{
			RuleID:      r.ID(),
			Name:        r.Name(),
			Severity:    r.Severity(),
			Line:        1,
			Description: r.Description(),
			Fix:         r.Fix(),
			Category:    r.Category(),
		})
	}

	return issues
}
