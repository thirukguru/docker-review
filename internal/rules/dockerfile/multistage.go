package dockerfile

import (
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// MultistageRule suggests multi-stage builds (DF008)
type MultistageRule struct {
	types.BaseRule
}

func NewMultistageRule() *MultistageRule {
	return &MultistageRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF008",
			RuleName:        "Missing multi-stage build",
			RuleSeverity:    types.Suggestion,
			RuleCategory:    "performance",
			RuleDescription: "Dockerfile could benefit from multi-stage build to reduce final image size.",
			RuleFix:         "Use multi-stage builds: build in one stage, copy only artifacts to final stage.",
		},
	}
}

func (r *MultistageRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	fromCount := 0
	hasBuildTools := false

	for _, instr := range ctx.Instructions {
		if instr.Name == "FROM" {
			fromCount++
		}
		if instr.Name == "RUN" {
			args := instr.Arguments
			if strings.Contains(args, "npm run build") || strings.Contains(args, "yarn build") ||
				strings.Contains(args, "go build") || strings.Contains(args, "cargo build") ||
				strings.Contains(args, "mvn package") || strings.Contains(args, "gradle build") ||
				strings.Contains(args, "make") || strings.Contains(args, "gcc") ||
				strings.Contains(args, "pip install") || strings.Contains(args, "npm install") {
				hasBuildTools = true
			}
		}
	}

	if fromCount == 1 && hasBuildTools {
		issues = append(issues, types.Issue{
			RuleID:      r.ID(),
			Name:        r.Name(),
			Severity:    r.Severity(),
			Line:        1,
			Description: "Build tools detected but no multi-stage build used",
			Fix:         r.Fix(),
			Category:    r.Category(),
		})
	}

	return issues
}
