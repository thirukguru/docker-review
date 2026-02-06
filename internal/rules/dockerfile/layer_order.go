package dockerfile

import (
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// LayerOrderRule checks for bad layer ordering (DF004)
type LayerOrderRule struct {
	types.BaseRule
}

func NewLayerOrderRule() *LayerOrderRule {
	return &LayerOrderRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF004",
			RuleName:        "Bad layer ordering",
			RuleSeverity:    types.Warning,
			RuleCategory:    "performance",
			RuleDescription: "Copying source code before installing dependencies breaks Docker layer caching.",
			RuleFix:         "Copy dependency files first (package.json, requirements.txt), install dependencies, then copy source code.",
		},
	}
}

func (r *LayerOrderRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	var lastCopyAllLine int
	var installAfterCopy int

	for _, instr := range ctx.Instructions {
		if instr.Name == "COPY" {
			args := instr.Arguments
			if strings.Contains(args, " . ") || strings.HasSuffix(args, " .") ||
				strings.HasSuffix(args, " ./") || strings.Contains(args, " ./ ") {
				if !strings.Contains(args, "--from") {
					lastCopyAllLine = instr.Line
				}
			}
		}

		if instr.Name == "RUN" && lastCopyAllLine > 0 {
			args := strings.ToLower(instr.Arguments)
			if strings.Contains(args, "npm install") ||
				strings.Contains(args, "pip install") ||
				strings.Contains(args, "apt-get install") ||
				strings.Contains(args, "apk add") ||
				strings.Contains(args, "yarn install") {
				installAfterCopy = instr.Line
			}
		}
	}

	if lastCopyAllLine > 0 && installAfterCopy > lastCopyAllLine {
		issues = append(issues, types.Issue{
			RuleID:      r.ID(),
			Name:        r.Name(),
			Severity:    r.Severity(),
			Line:        lastCopyAllLine,
			Description: "COPY of all source files before package install breaks layer caching",
			Fix:         r.Fix(),
			Category:    r.Category(),
		})
	}

	return issues
}
