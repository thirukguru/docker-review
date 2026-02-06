package compose

import (
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// DangerousVolumesRule checks for dangerous volume mounts (DC008)
type DangerousVolumesRule struct {
	types.BaseRule
}

func NewDangerousVolumesRule() *DangerousVolumesRule {
	return &DangerousVolumesRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC008",
			RuleName:        "Dangerous volume mount",
			RuleSeverity:    types.Critical,
			RuleCategory:    "security",
			RuleDescription: "System directory mounted - potential host compromise.",
			RuleFix:         "Mount only specific directories needed. Avoid mounting /, /etc, /var, /usr.",
		},
	}
}

var dangerousPaths = []string{
	"/:/",    // Root mount
	"/etc:",  // System config
	"/var:",  // System data
	"/usr:",  // System binaries
	"/root:", // Root home
	"/proc:", // Process info
	"/sys:",  // Kernel interfaces
	"/dev:",  // Devices
	"/boot:", // Boot files
}

func (r *DangerousVolumesRule) Check(ctx *types.ComposeContext) []types.Issue {
	var issues []types.Issue

	for name, svc := range ctx.Services {
		if volumes, ok := svc.Raw["volumes"].([]interface{}); ok {
			for _, vol := range volumes {
				volStr := ""
				switch v := vol.(type) {
				case string:
					volStr = v
				case map[string]interface{}:
					if source, ok := v["source"].(string); ok {
						volStr = source + ":"
					}
				}

				for _, dangerous := range dangerousPaths {
					if strings.HasPrefix(volStr, dangerous) {
						path := strings.Split(dangerous, ":")[0]
						issues = append(issues, types.Issue{
							RuleID:      r.ID(),
							Name:        r.Name(),
							Severity:    r.Severity(),
							Line:        0,
							Description: "Service '" + name + "' mounts dangerous path: " + path,
							Fix:         r.Fix(),
							Category:    r.Category(),
						})
						break
					}
				}
			}
		}
	}

	return issues
}
