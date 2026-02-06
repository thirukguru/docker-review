package compose

import (
	"fmt"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// ExposedPortsRule checks for ports exposed to all interfaces (DC011)
type ExposedPortsRule struct {
	types.BaseRule
}

func NewExposedPortsRule() *ExposedPortsRule {
	return &ExposedPortsRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC011",
			RuleName:        "Port bound to all interfaces",
			RuleSeverity:    types.Suggestion,
			RuleCategory:    "security",
			RuleDescription: "Port bound to 0.0.0.0 (all interfaces) - accessible from any network.",
			RuleFix:         "Bind to 127.0.0.1 for local-only access, or use specific interface IP.",
		},
	}
}

func (r *ExposedPortsRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		if ports, ok := svc.Raw["ports"].([]interface{}); ok {
			for _, port := range ports {
				portStr := ""
				switch p := port.(type) {
				case string:
					portStr = p
				case map[string]interface{}:
					if published, ok := p["published"]; ok {
						portStr = fmt.Sprintf("%v", published)
					}
				}

				// Explicit 0.0.0.0 binding
				if strings.HasPrefix(portStr, "0.0.0.0:") {
					issues = append(issues, types.Issue{
						RuleID:      r.ID(),
						Name:        r.Name(),
						Severity:    r.Severity(),
						Line:        0,
						Description: fmt.Sprintf("Service '%s' explicitly binds to 0.0.0.0: %s", name, portStr),
						Fix:         r.Fix(),
						Category:    r.Category(),
					})
				}
			}
		}
	}

	return issues
}
