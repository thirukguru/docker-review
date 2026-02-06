package compose

import (
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// LatestTagRule checks for latest tag in compose (DC004)
type LatestTagRule struct {
	types.BaseRule
}

func NewLatestTagRule() *LatestTagRule {
	return &LatestTagRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC004",
			RuleName:        "Using latest tag",
			RuleSeverity:    types.Critical,
			RuleCategory:    "maintainability",
			RuleDescription: "Service uses 'latest' tag or no tag, creating unpredictable deployments.",
			RuleFix:         "Pin images to specific versions (e.g., nginx:1.25.0)",
		},
	}
}

func (r *LatestTagRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		if svc.Image == "" {
			continue
		}

		image := svc.Image
		if !strings.Contains(image, ":") || strings.HasSuffix(image, ":latest") {
			desc := ""
			if strings.HasSuffix(image, ":latest") {
				desc = "Service '" + name + "' uses ':latest' tag"
			} else {
				desc = "Service '" + name + "' image has no tag (implicitly 'latest')"
			}
			issues = append(issues, types.Issue{
				RuleID:      r.ID(),
				Name:        r.Name(),
				Severity:    r.Severity(),
				Line:        0,
				Description: desc,
				Fix:         r.Fix(),
				Category:    r.Category(),
			})
		}
	}

	return issues
}
