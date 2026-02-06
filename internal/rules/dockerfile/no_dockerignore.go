package dockerfile

import (
	"github.com/thirukguru/docker-review/internal/types"
)

// NoDockerignoreRule checks for .dockerignore (DF003)
type NoDockerignoreRule struct {
	types.BaseRule
}

func NewNoDockerignoreRule() *NoDockerignoreRule {
	return &NoDockerignoreRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF003",
			RuleName:        "No .dockerignore",
			RuleSeverity:    types.Warning,
			RuleCategory:    "performance",
			RuleDescription: "No .dockerignore file found. Build context may include unnecessary files.",
			RuleFix:         "Create a .dockerignore file to exclude node_modules, .git, etc.",
		},
	}
}

func (r *NoDockerignoreRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	if !ctx.HasDockerignore {
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
