package compose

import (
	"github.com/thirukguru/docker-review/internal/types"
)

// ResourceLimitsRule checks for resource limits (DC003)
type ResourceLimitsRule struct {
	types.BaseRule
}

func NewResourceLimitsRule() *ResourceLimitsRule {
	return &ResourceLimitsRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC003",
			RuleName:        "No resource limits",
			RuleSeverity:    types.Warning,
			RuleCategory:    "performance",
			RuleDescription: "Service has no resource limits. Can consume unlimited host resources.",
			RuleFix:         "Add deploy.resources.limits with cpus and memory settings.",
		},
	}
}

func (r *ResourceLimitsRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		hasLimits := false
		if svc.Deploy != nil && svc.Deploy.Resources != nil && svc.Deploy.Resources.Limits != nil {
			if svc.Deploy.Resources.Limits.CPUs != "" || svc.Deploy.Resources.Limits.Memory != "" {
				hasLimits = true
			}
		}

		if !hasLimits {
			issues = append(issues, types.Issue{
				RuleID:      r.ID(),
				Name:        r.Name(),
				Severity:    r.Severity(),
				Line:        0,
				Description: "Service '" + name + "' has no resource limits",
				Fix:         r.Fix(),
				Category:    r.Category(),
			})
		}
	}

	return issues
}
