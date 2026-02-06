package compose

import (
	"regexp"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// HardcodedSecretsRule checks for hardcoded secrets (DC005)
type HardcodedSecretsRule struct {
	types.BaseRule
}

func NewHardcodedSecretsRule() *HardcodedSecretsRule {
	return &HardcodedSecretsRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC005",
			RuleName:        "Hardcoded secrets",
			RuleSeverity:    types.Critical,
			RuleCategory:    "security",
			RuleDescription: "Environment variables contain hardcoded secrets.",
			RuleFix:         "Use Docker secrets or environment variables from .env file.",
		},
	}
}

var secretKeyPatterns = []*regexp.Regexp{
	regexp.MustCompile(`(?i)(password|passwd|pwd)`),
	regexp.MustCompile(`(?i)(secret|api_key|apikey)`),
	regexp.MustCompile(`(?i)(token|auth|credential)`),
}

func (r *HardcodedSecretsRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		for key, value := range svc.Environment {
			isSecretKey := false
			for _, pattern := range secretKeyPatterns {
				if pattern.MatchString(key) {
					isSecretKey = true
					break
				}
			}

			if !isSecretKey {
				continue
			}

			if value != "" && !strings.HasPrefix(value, "${") && !strings.HasPrefix(value, "$") {
				issues = append(issues, types.Issue{
					RuleID:      r.ID(),
					Name:        r.Name(),
					Severity:    r.Severity(),
					Line:        0,
					Description: "Service '" + name + "' has hardcoded secret: " + key,
					Fix:         r.Fix(),
					Category:    r.Category(),
				})
			}
		}
	}

	return issues
}
