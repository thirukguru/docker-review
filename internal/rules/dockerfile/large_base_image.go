package dockerfile

import (
	"regexp"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// LargeBaseImageRule detects large base images (DF009)
type LargeBaseImageRule struct {
	types.BaseRule
}

func NewLargeBaseImageRule() *LargeBaseImageRule {
	return &LargeBaseImageRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF009",
			RuleName:        "Large base image",
			RuleSeverity:    types.Suggestion,
			RuleCategory:    "performance",
			RuleDescription: "Using a large base image. Consider using Alpine or slim variants.",
			RuleFix:         "Use smaller base images (e.g., node:18-alpine instead of node:18)",
		},
	}
}

var largeImages = map[string]string{
	"ubuntu":  "ubuntu is ~77MB, consider alpine (~5MB)",
	"debian":  "debian is ~124MB, consider debian-slim or alpine",
	"python":  "python is ~900MB, consider python:slim or python:alpine",
	"node":    "node is ~900MB, consider node:slim or node:alpine",
	"golang":  "golang is ~800MB, use multi-stage builds",
	"openjdk": "openjdk is ~400MB, consider eclipse-temurin:alpine",
	"ruby":    "ruby is ~850MB, consider ruby:slim or ruby:alpine",
	"php":     "php is ~400MB, consider php:alpine",
}

var fromImageRegex = regexp.MustCompile(`^FROM\s+([^\s:]+)`)

func (r *LargeBaseImageRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	for _, instr := range ctx.Instructions {
		if instr.Name != "FROM" {
			continue
		}

		matches := fromImageRegex.FindStringSubmatch(instr.Raw)
		if matches == nil {
			continue
		}

		image := strings.ToLower(matches[1])

		if strings.Contains(instr.Arguments, "alpine") ||
			strings.Contains(instr.Arguments, "slim") ||
			strings.Contains(instr.Arguments, "distroless") {
			continue
		}

		for baseImage, suggestion := range largeImages {
			if image == baseImage || strings.HasPrefix(image, baseImage+":") {
				issues = append(issues, types.Issue{
					RuleID:      r.ID(),
					Name:        r.Name(),
					Severity:    r.Severity(),
					Line:        instr.Line,
					Description: suggestion,
					Fix:         r.Fix(),
					Category:    r.Category(),
				})
				break
			}
		}
	}

	return issues
}
