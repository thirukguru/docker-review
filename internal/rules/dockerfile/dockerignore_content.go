package dockerfile

import (
	"bufio"
	"os"
	"path/filepath"
	"strings"

	"github.com/thirukguru/docker-review/internal/types"
)

// DockerignoreContentRule checks .dockerignore content (DF013)
type DockerignoreContentRule struct {
	types.BaseRule
}

func NewDockerignoreContentRule() *DockerignoreContentRule {
	return &DockerignoreContentRule{
		BaseRule: types.BaseRule{
			RuleID:          "DF013",
			RuleName:        "Incomplete .dockerignore",
			RuleSeverity:    types.Warning,
			RuleCategory:    "performance",
			RuleDescription: ".dockerignore is missing recommended patterns that could bloat image.",
			RuleFix:         "Add missing patterns to .dockerignore to reduce build context size.",
		},
	}
}

// Pattern categories with descriptions
type patternInfo struct {
	pattern     string
	description string
	category    string // common, node, python, git, secrets
}

var recommendedPatterns = []patternInfo{
	// Git
	{".git", "Git history (often 50MB+)", "git"},
	{".gitignore", "Git config", "git"},

	// Common
	{"*.log", "Log files", "common"},
	{"*.tmp", "Temporary files", "common"},
	{"*.swp", "Editor swap files", "common"},
	{".DS_Store", "macOS metadata", "common"},
	{"Thumbs.db", "Windows thumbnails", "common"},

	// Secrets
	{".env", "Environment secrets!", "secrets"},
	{"*.pem", "Private keys!", "secrets"},
	{"*.key", "Private keys!", "secrets"},
	{".env.*", "Environment files", "secrets"},

	// Node.js
	{"node_modules", "Node dependencies (use npm ci instead)", "node"},
	{"npm-debug.log", "NPM debug logs", "node"},
	{".npm", "NPM cache", "node"},

	// Python
	{"__pycache__", "Python bytecode cache", "python"},
	{"*.pyc", "Compiled Python", "python"},
	{".venv", "Python virtualenv", "python"},
	{"venv", "Python virtualenv", "python"},
	{".pytest_cache", "Pytest cache", "python"},

	// Build artifacts
	{"dist", "Build output", "build"},
	{"build", "Build output", "build"},
	{"target", "Build output (Rust/Java)", "build"},
	{"*.tar", "Archive files", "build"},
	{"*.zip", "Archive files", "build"},

	// IDE
	{".idea", "IntelliJ settings", "ide"},
	{".vscode", "VS Code settings", "ide"},
	{"*.iml", "IntelliJ module files", "ide"},

	// Docker
	{"Dockerfile*", "Dockerfile itself", "docker"},
	{"docker-compose*.yml", "Compose files", "docker"},
}

func (r *DockerignoreContentRule) Check(ctx *types.DockerfileContext) []types.Issue {
	var issues []types.Issue

	dockerignorePath := filepath.Join(filepath.Dir(ctx.Path), ".dockerignore")

	// If no .dockerignore, DF003 handles it
	if _, err := os.Stat(dockerignorePath); os.IsNotExist(err) {
		return issues
	}

	// Read existing patterns
	existingPatterns := readDockerignore(dockerignorePath)

	// Detect project type from Dockerfile
	projectTypes := detectProjectTypes(ctx)

	// Find missing patterns
	var missingPatterns []patternInfo
	for _, rec := range recommendedPatterns {
		// Skip patterns not relevant to this project
		if !isRelevantPattern(rec, projectTypes) {
			continue
		}

		if !hasPattern(existingPatterns, rec.pattern) {
			missingPatterns = append(missingPatterns, rec)
		}
	}

	// Group by priority
	var secretsMissing, commonMissing []string
	for _, p := range missingPatterns {
		if p.category == "secrets" {
			secretsMissing = append(secretsMissing, p.pattern+" ("+p.description+")")
		} else {
			commonMissing = append(commonMissing, p.pattern)
		}
	}

	// Report secrets as separate critical issue
	if len(secretsMissing) > 0 {
		issues = append(issues, types.Issue{
			RuleID:      r.ID(),
			Name:        "Secrets not in .dockerignore",
			Severity:    types.Critical,
			Line:        1,
			Description: "Missing: " + strings.Join(secretsMissing, ", "),
			Fix:         "Add secret patterns to .dockerignore immediately",
			Category:    "security",
		})
	}

	// Report common patterns as warning
	if len(commonMissing) > 3 {
		issues = append(issues, types.Issue{
			RuleID:      r.ID(),
			Name:        r.Name(),
			Severity:    r.Severity(),
			Line:        1,
			Description: "Missing " + string(rune(len(commonMissing))) + "+ patterns: " + strings.Join(commonMissing[:3], ", ") + "...",
			Fix:         r.Fix(),
			Category:    r.Category(),
		})
	}

	return issues
}

func readDockerignore(path string) []string {
	file, err := os.Open(path)
	if err != nil {
		return nil
	}
	defer file.Close()

	var patterns []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line != "" && !strings.HasPrefix(line, "#") {
			patterns = append(patterns, line)
		}
	}
	return patterns
}

func detectProjectTypes(ctx *types.DockerfileContext) map[string]bool {
	types := make(map[string]bool)
	types["common"] = true
	types["git"] = true
	types["secrets"] = true
	types["ide"] = true

	for _, instr := range ctx.Instructions {
		args := strings.ToLower(instr.Arguments)

		if strings.Contains(args, "node") || strings.Contains(args, "npm") || strings.Contains(args, "yarn") {
			types["node"] = true
		}
		if strings.Contains(args, "python") || strings.Contains(args, "pip") {
			types["python"] = true
		}
		if strings.Contains(args, "go ") || strings.Contains(args, "golang") {
			types["build"] = true
		}
		if strings.Contains(args, "cargo") || strings.Contains(args, "rust") {
			types["build"] = true
		}
	}

	return types
}

func isRelevantPattern(p patternInfo, projectTypes map[string]bool) bool {
	return projectTypes[p.category]
}

func hasPattern(existing []string, pattern string) bool {
	patternLower := strings.ToLower(pattern)
	for _, e := range existing {
		eLower := strings.ToLower(e)
		// Exact match or wildcard covers it
		if eLower == patternLower {
			return true
		}
		// Check if existing pattern covers the recommended one
		if strings.HasPrefix(patternLower, strings.TrimSuffix(eLower, "*")) {
			return true
		}
		// node_modules/ covers node_modules
		if strings.TrimSuffix(eLower, "/") == patternLower {
			return true
		}
	}
	return false
}
