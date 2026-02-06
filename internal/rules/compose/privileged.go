package compose

import (
	"github.com/thirukguru/docker-review/internal/types"
)

// PrivilegedRule checks for privileged containers (DC002)
type PrivilegedRule struct {
	types.BaseRule
}

func NewPrivilegedRule() *PrivilegedRule {
	return &PrivilegedRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC002",
			RuleName:        "Privileged container",
			RuleSeverity:    types.Critical,
			RuleCategory:    "security",
			RuleDescription: "Container running in privileged mode has full access to host.",
			RuleFix:         "Remove privileged: true. Use specific capabilities if needed.",
		},
	}
}

func (r *PrivilegedRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		if svc.Privileged {
			issues = append(issues, types.Issue{
				RuleID:      r.ID(),
				Name:        r.Name(),
				Severity:    r.Severity(),
				Line:        0,
				Description: "Service '" + name + "' is running in privileged mode",
				Fix:         r.Fix(),
				Category:    r.Category(),
			})
		}
	}

	return issues
}
