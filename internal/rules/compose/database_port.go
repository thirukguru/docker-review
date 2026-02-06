package compose

import (
	"fmt"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// DatabasePortRule checks for exposed database ports (DC010)
type DatabasePortRule struct {
	types.BaseRule
}

func NewDatabasePortRule() *DatabasePortRule {
	return &DatabasePortRule{
		BaseRule: types.BaseRule{
			RuleID:          "DC010",
			RuleName:        "Database port exposed",
			RuleSeverity:    types.Warning,
			RuleCategory:    "security",
			RuleDescription: "Database port exposed externally - risk of unauthorized access.",
			RuleFix:         "Use internal networks. If external access needed, bind to 127.0.0.1 or use VPN.",
		},
	}
}

var databasePorts = map[string]string{
	"3306":  "MySQL/MariaDB",
	"5432":  "PostgreSQL",
	"27017": "MongoDB",
	"6379":  "Redis",
	"9042":  "Cassandra",
	"9200":  "Elasticsearch",
	"5984":  "CouchDB",
	"1433":  "SQL Server",
	"1521":  "Oracle",
	"7474":  "Neo4j",
	"8529":  "ArangoDB",
	"11211": "Memcached",
	"26257": "CockroachDB",
}

func (r *DatabasePortRule) Check(ctx *types.ComposeContext) []types.Issue {
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

				// Check if binding to 0.0.0.0 (external)
				isExternal := !strings.HasPrefix(portStr, "127.0.0.1:") &&
					!strings.HasPrefix(portStr, "localhost:")

				for dbPort, dbName := range databasePorts {
					if isExternal && strings.Contains(portStr, dbPort) {
						issues = append(issues, types.Issue{
							RuleID:      r.ID(),
							Name:        r.Name(),
							Severity:    r.Severity(),
							Line:        0,
							Description: fmt.Sprintf("Service '%s' exposes %s port (%s) externally", name, dbName, dbPort),
							Fix:         r.Fix(),
							Category:    r.Category(),
						})
					}
				}
			}
		}
	}

	return issues
}
