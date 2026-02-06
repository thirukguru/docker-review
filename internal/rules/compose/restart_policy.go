package compose

import (
	"github.com/thirukguru/docker-review/internal/types"
)

// RestartPolicyRule checks for restart policy (DC001)
type RestartPolicyRule struct {
	types.BaseRule
}

func NewRestartPolicyRule() *RestartPolicyRule {
	return &RestartPolicyRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC001",
			RuleName:        "No restart policy",
			RuleSeverity:    types.Warning,
			RuleCategory:    "maintainability",
			RuleDescription: "Service has no restart policy. Container won't restart on failure.",
			RuleFix:         "Add restart: unless-stopped or restart: always",
		},
	}
}

func (r *RestartPolicyRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		if svc.Restart == "" {
			issues = append(issues, types.Issue{
				RuleID:      r.ID(),
				Name:        r.Name(),
				Severity:    r.Severity(),
				Line:        0,
				Description: "Service '" + name + "' has no restart policy",
				Fix:         r.Fix(),
				Category:    r.Category(),
			})
		}
	}

	return issues
}
