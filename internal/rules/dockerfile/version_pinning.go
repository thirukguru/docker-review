package dockerfile

import (
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// VersionPinningRule checks for unpinned package versions (DF007)
type VersionPinningRule struct {
	types.BaseRule
}

func NewVersionPinningRule() *VersionPinningRule {
	return &VersionPinningRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF007",
			RuleName:        "No version pinning",
			RuleSeverity:    types.Warning,
			RuleCategory:    "maintainability",
			RuleDescription: "Package installations without version pinning can lead to non-reproducible builds.",
			RuleFix:         "Pin package versions (e.g., apt-get install curl=7.68.0, pip install requests==2.28.0)",
		},
	}
}

func (r *VersionPinningRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	for _, instr := range ctx.Instructions {
		if instr.Name != "RUN" {
			continue
		}

		args := instr.Arguments

		if strings.Contains(args, "apt-get install") || strings.Contains(args, "apt install") {
			if !strings.Contains(args, "=") && !strings.Contains(args, "--no-install-recommends") {
				issues = append(issues, types.Issue{
					RuleID:      r.ID(),
					Name:        r.Name(),
					Severity:    r.Severity(),
					Line:        instr.Line,
					Description: "apt-get packages installed without version pinning",
					Fix:         r.Fix(),
					Category:    r.Category(),
				})
			}
		}

		if strings.Contains(args, "pip install") {
			if !strings.Contains(args, "==") && !strings.Contains(args, "-r requirements") {
				issues = append(issues, types.Issue{
					RuleID:      r.ID(),
					Name:        r.Name(),
					Severity:    r.Severity(),
					Line:        instr.Line,
					Description: "pip packages installed without version pinning",
					Fix:         r.Fix(),
					Category:    r.Category(),
				})
			}
		}
	}

	return issues
}
