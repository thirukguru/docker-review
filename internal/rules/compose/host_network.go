package compose

import (
	"github.com/thirukguru/docker-review/internal/types"
)

// HostNetworkRule checks for host network mode (DC007)
type HostNetworkRule struct {
	types.BaseRule
}

func NewHostNetworkRule() *HostNetworkRule {
	return &HostNetworkRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC007",
			RuleName:        "Host network mode",
			RuleSeverity:    types.Critical,
			RuleCategory:    "security",
			RuleDescription: "Container uses host network, bypassing network isolation.",
			RuleFix:         "Remove network_mode: host. Use bridge networking with explicit port mappings.",
		},
	}
}

func (r *HostNetworkRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		if networkMode, ok := svc.Raw["network_mode"].(string); ok {
			if networkMode == "host" {
				issues = append(issues, types.Issue{
					RuleID:      r.ID(),
					Name:        r.Name(),
					Severity:    r.Severity(),
					Line:        0,
					Description: "Service '" + name + "' uses host network mode - no network isolation",
					Fix:         r.Fix(),
					Category:    r.Category(),
				})
			}
		}
	}

	return issues
}
