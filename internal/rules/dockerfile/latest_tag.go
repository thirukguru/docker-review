package dockerfile

import (
	"regexp"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// LatestTagRule checks for :latest or missing tags (DF001)
type LatestTagRule struct {
	types.BaseRule
}

func NewLatestTagRule() *LatestTagRule {
	return &LatestTagRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF001",
			RuleName:        "Using latest tag",
			RuleSeverity:    types.Critical,
			RuleCategory:    "maintainability",
			RuleDescription: "Using 'latest' tag or no tag creates unpredictable builds.",
			RuleFix:         "Pin to a specific version tag (e.g., FROM node:18.17.0-alpine)",
		},
	}
}

var fromRegex = regexp.MustCompile(`^FROM\s+([^\s]+)`)

func (r *LatestTagRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	for _, instr := range ctx.Instructions {
		if instr.Name != "FROM" {
			continue
		}

		matches := fromRegex.FindStringSubmatch(instr.Raw)
		if matches == nil {
			continue
		}

		image := matches[1]

		// Skip scratch and build stages
		if image == "scratch" || strings.Contains(image, "$") {
			continue
		}

		// Check for AS alias
		imageParts := strings.Fields(instr.Arguments)
		if len(imageParts) > 0 {
			image = imageParts[0]
		}

		// Check if using latest or no tag
		if !strings.Contains(image, ":") || strings.HasSuffix(image, ":latest") {
			desc := ""
			if strings.HasSuffix(image, ":latest") {
				desc = "Image '" + strings.TrimSuffix(image, ":latest") + "' uses 'latest' tag"
			} else {
				desc = "Image '" + image + "' has no tag (implicitly uses 'latest')"
			}
			issues = append(issues, types.Issue{
				RuleID:      r.ID(),
				Name:        r.Name(),
				Severity:    r.Severity(),
				Line:        instr.Line,
				Description: desc,
				Fix:         r.Fix(),
				Category:    r.Category(),
			})
		}
	}

	return issues
}
