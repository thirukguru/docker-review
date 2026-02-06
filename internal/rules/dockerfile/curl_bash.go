package dockerfile

import (
	"regexp"

	"github.com/thirukguru/docker-review/internal/types"
)

// CurlBashRule detects curl piped to shell (DF010)
type CurlBashRule struct {
	types.BaseRule
}

func NewCurlBashRule() *CurlBashRule {
	return &CurlBashRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF010",
			RuleName:        "Curl pipe to shell",
			RuleSeverity:    types.Critical,
			RuleCategory:    "security",
			RuleDescription: "Piping curl/wget output directly to shell is a security risk.",
			RuleFix:         "Download scripts first, verify checksums, then execute.",
		},
	}
}

var curlBashPatterns = []*regexp.Regexp{
	regexp.MustCompile(`curl[^|]*\|\s*(ba)?sh`),
	regexp.MustCompile(`wget[^|]*\|\s*(ba)?sh`),
}

func (r *CurlBashRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	for _, instr := range ctx.Instructions {
		if instr.Name != "RUN" {
			continue
		}

		for _, pattern := range curlBashPatterns {
			if pattern.MatchString(instr.Arguments) {
				issues = append(issues, types.Issue{
					RuleID:      r.ID(),
					Name:        r.Name(),
					Severity:    r.Severity(),
					Line:        instr.Line,
					Description: "Remote script piped directly to shell without verification",
					Fix:         r.Fix(),
					Category:    r.Category(),
				})
				break
			}
		}
	}

	return issues
}
