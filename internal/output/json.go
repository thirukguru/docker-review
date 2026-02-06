package output

import (
	"encoding/json"

	"github.com/thirukguru/docker-review/internal/analyzer"
)

// FormatJSON formats the report as JSON
func FormatJSON(report *analyzer.Report) string {
	data, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		return `{"error": "failed to marshal report"}`
	}
	return string(data)
}
