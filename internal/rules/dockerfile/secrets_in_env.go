package dockerfile

import (
	"regexp"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// SecretsInEnvRule checks for secrets in ENV (DF006)
type SecretsInEnvRule struct {
	types.BaseRule
}

func NewSecretsInEnvRule() *SecretsInEnvRule {
	return &SecretsInEnvRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF006",
			RuleName:        "Secrets in ENV",
			RuleSeverity:    types.Critical,
			RuleCategory:    "security",
			RuleDescription: "Sensitive data (passwords, API keys, tokens) found in ENV instruction.",
			RuleFix:         "Use Docker secrets, environment variables at runtime, or --secret flag in BuildKit.",
		},
	}
}

var secretPatterns = []*regexp.Regexp{
	regexp.MustCompile(`(?i)(password|passwd|pwd)\s*=`),
	regexp.MustCompile(`(?i)(secret|api_key|apikey|api-key)\s*=`),
	regexp.MustCompile(`(?i)(token|auth|credential)\s*=`),
	regexp.MustCompile(`(?i)(private_key|privatekey)\s*=`),
	regexp.MustCompile(`(?i)(aws_secret|aws_access)\s*=`),
}

func (r *SecretsInEnvRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	for _, instr := range ctx.Instructions {
		if instr.Name != "ENV" && instr.Name != "ARG" {
			continue
		}

		args := instr.Arguments
		for _, pattern := range secretPatterns {
			if pattern.MatchString(args) {
				varName := extractVarName(args, pattern)
				issues = append(issues, types.Issue{
					RuleID:      r.ID(),
					Name:        r.Name(),
					Severity:    r.Severity(),
					Line:        instr.Line,
					Description: "Potential secret found in " + instr.Name + ": " + varName,
					Fix:         r.Fix(),
					Category:    r.Category(),
				})
				break
			}
		}
	}

	return issues
}

func extractVarName(args string, pattern *regexp.Regexp) string {
	match := pattern.FindStringSubmatch(args)
	if len(match) > 1 {
		return strings.ToUpper(match[1])
	}
	return "sensitive variable"
}
