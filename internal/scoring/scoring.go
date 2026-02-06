package scoring

import (
	"github.com/thirukguru/docker-review/internal/types"
)

// Scores represents the scoring for a Docker configuration
type Scores struct {
	Security        int `json:"security"`
	Performance     int `json:"performance"`
	Maintainability int `json:"maintainability"`
	Overall         int `json:"overall"`
}

// Calculate computes scores based on issues
func Calculate(issues []types.Issue) Scores {
	scores := Scores{
		Security:        10,
		Performance:     10,
		Maintainability: 10,
	}

	for _, issue := range issues {
		penalty := getPenalty(issue.Severity)

		switch issue.Category {
		case "security":
			scores.Security = max(0, scores.Security-penalty)
		case "performance":
			scores.Performance = max(0, scores.Performance-penalty)
		case "maintainability":
			scores.Maintainability = max(0, scores.Maintainability-penalty)
		}
	}

	scores.Overall = (scores.Security*4 + scores.Performance*3 + scores.Maintainability*3) / 10

	return scores
}

func getPenalty(severity types.Severity) int {
	switch severity {
	case types.Critical:
		return 3
	case types.Warning:
		return 2
	case types.Suggestion:
		return 1
	default:
		return 0
	}
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
