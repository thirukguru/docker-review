package dockerfile

import (
	"github.com/thirukguru/docker-review/internal/types"
)

// RootUserRule checks for running as root (DF002)
type RootUserRule struct {
	types.BaseRule
}

func NewRootUserRule() *RootUserRule {
	return &RootUserRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF002",
			RuleName:        "Running as root",
			RuleSeverity:    types.Critical,
			RuleCategory:    "security",
			RuleDescription: "Container runs as root user, which is a security risk.",
			RuleFix:         "Add USER directive with non-root user (e.g., USER node or USER 1000:1000)",
		},
	}
}

func (r *RootUserRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue
	hasUser := false
	lastFromLine := 0

	for _, instr := range ctx.Instructions {
		if instr.Name == "FROM" {
			hasUser = false
			lastFromLine = instr.Line
		}
		if instr.Name == "USER" {
			hasUser = true
		}
	}

	if !hasUser && lastFromLine > 0 {
		issues = append(issues, types.Issue{
			RuleID:      r.ID(),
			Name:        r.Name(),
			Severity:    r.Severity(),
			Line:        lastFromLine,
			Description: "No USER directive found - container will run as root",
			Fix:         r.Fix(),
			Category:    r.Category(),
		})
	}

	return issues
}
