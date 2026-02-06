package compose

import (
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// DockerSocketRule checks for Docker socket mount (DC006)
type DockerSocketRule struct {
	types.BaseRule
}

func NewDockerSocketRule() *DockerSocketRule {
	return &DockerSocketRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC006",
			RuleName:        "Docker socket mount",
			RuleSeverity:    types.Critical,
			RuleCategory:    "security",
			RuleDescription: "Docker socket mounted - allows container escape and full host control.",
			RuleFix:         "Remove /var/run/docker.sock mount. Use Docker API over TCP with TLS if needed.",
		},
	}
}

func (r *DockerSocketRule) Check(ctx *types.ComposeContext) []types.Issue {
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
						volStr = source
					}
				}
				if strings.Contains(volStr, "docker.sock") {
					issues = append(issues, types.Issue{
						RuleID:      r.ID(),
						Name:        r.Name(),
						Severity:    r.Severity(),
						Line:        0,
						Description: "Service '" + name + "' mounts Docker socket - container escape risk",
						Fix:         r.Fix(),
						Category:    r.Category(),
					})
					break
				}
			}
		}
	}

	return issues
}
