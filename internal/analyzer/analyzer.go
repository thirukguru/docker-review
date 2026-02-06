package analyzer

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/thirukguru/docker-review/internal/parser"
	"github.com/thirukguru/docker-review/internal/rules"
	"github.com/thirukguru/docker-review/internal/scoring"
	"github.com/thirukguru/docker-review/internal/types"
)

// Report contains analysis results
type Report struct {
	FilePath string         `json:"file_path"`
	FileType string         `json:"file_type"`
	Issues   []types.Issue  `json:"issues"`
	Scores   scoring.Scores `json:"scores"`
}

// Analyze analyzes a file or directory
func Analyze(path string, isDir bool) (*Report, error) {
	if isDir {
		dockerfile := filepath.Join(path, "Dockerfile")
		if _, err := os.Stat(dockerfile); err == nil {
			return analyzeDockerfile(dockerfile)
		}

		compose := filepath.Join(path, "docker-compose.yml")
		if _, err := os.Stat(compose); err == nil {
			return analyzeCompose(compose)
		}

		compose = filepath.Join(path, "docker-compose.yaml")
		if _, err := os.Stat(compose); err == nil {
			return analyzeCompose(compose)
		}

		return nil, fmt.Errorf("no Dockerfile or docker-compose.yml found in %s", path)
	}

	if parser.IsDockerfile(path) {
		return analyzeDockerfile(path)
	}
	if parser.IsComposeFile(path) {
		return analyzeCompose(path)
	}

	return nil, fmt.Errorf("unknown file type: %s", path)
}

func analyzeDockerfile(path string) (*Report, error) {
	ctx, err := parser.ParseDockerfile(path)
	if err != nil {
		return nil, fmt.Errorf("failed to parse Dockerfile: %w", err)
	}

	var allIssues []types.Issue
	for _, rule := range rules.GetDockerfileRules() {
		issues := rule.Check(ctx)
		allIssues = append(allIssues, issues...)
	}

	scores := scoring.Calculate(allIssues)

	return &Report{
		FilePath: path,
		FileType: "dockerfile",
		Issues:   allIssues,
		Scores:   scores,
	}, nil
}

func analyzeCompose(path string) (*Report, error) {
	ctx, err := parser.ParseCompose(path)
	if err != nil {
		return nil, fmt.Errorf("failed to parse compose file: %w", err)
	}

	var allIssues []types.Issue
	for _, rule := range rules.GetComposeRules() {
		issues := rule.Check(ctx)
		allIssues = append(allIssues, issues...)
	}

	scores := scoring.Calculate(allIssues)

	return &Report{
		FilePath: path,
		FileType: "compose",
		Issues:   allIssues,
		Scores:   scores,
	}, nil
}
