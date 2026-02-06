package compose

import (
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// CapabilitiesRule checks for added Linux capabilities (DC009)
type CapabilitiesRule struct {
	types.BaseRule
}

func NewCapabilitiesRule() *CapabilitiesRule {
	return &CapabilitiesRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC009",
			RuleName:        "Capability additions",
			RuleSeverity:    types.Warning,
			RuleCategory:    "security",
			RuleDescription: "Linux capabilities added - expands container privileges.",
			RuleFix:         "Remove cap_add if not required. Use cap_drop: [ALL] and add only needed capabilities.",
		},
	}
}

var dangerousCaps = map[string]bool{
	"SYS_ADMIN":    true,
	"NET_ADMIN":    true,
	"SYS_PTRACE":   true,
	"DAC_OVERRIDE": true,
	"SETUID":       true,
	"SETGID":       true,
	"NET_RAW":      true,
	"SYS_RAWIO":    true,
	"MKNOD":        true,
	"ALL":          true,
}

func (r *CapabilitiesRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		if capAdd, ok := svc.Raw["cap_add"].([]interface{}); ok {
			for _, cap := range capAdd {
				capStr := strings.ToUpper(cap.(string))
				if dangerousCaps[capStr] {
					issues = append(issues, types.Issue{
						RuleID:      r.ID(),
						Name:        r.Name(),
						Severity:    r.Severity(),
						Line:        0,
						Description: "Service '" + name + "' adds dangerous capability: " + capStr,
						Fix:         r.Fix(),
						Category:    r.Category(),
					})
				}
			}
		}
	}

	return issues
}
