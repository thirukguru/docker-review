package fixer

import (
"os"
"regexp"
"strings"

"github.com/thirukguru/docker-review/internal/types"
)

// FixResult contains the result of auto-fixing
type FixResult struct {
OptimizedContent string
Changes          []Change
OriginalContent  string
}

// Change represents a single change made
type Change struct {
LineNumber  int
Description string
Before      string
After       string
}

// Fix applies fixes to a Dockerfile
func Fix(path string, issues []types.Issue) *FixResult {
content, err := os.ReadFile(path)
if err != nil {
return &FixResult{OptimizedContent: string(content)}
}

original := string(content)
lines := strings.Split(original, "\n")
var changes []Change

for _, issue := range issues {
switch issue.RuleID {
case "DF001":
if issue.Line > 0 && issue.Line <= len(lines) {
line := lines[issue.Line-1]
fixed, change := fixLatestTag(line, issue.Line)
if change != nil {
lines[issue.Line-1] = fixed
changes = append(changes, *change)
}
}
case "DF005":
for i := len(lines) - 1; i >= 0; i-- {
trimmed := strings.TrimSpace(lines[i])
if strings.HasPrefix(trimmed, "CMD") || strings.HasPrefix(trimmed, "ENTRYPOINT") {
healthcheck := "HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost/ || exit 1"
lines = insertLine(lines, i, healthcheck)
changes = append(changes, Change{
LineNumber:  i + 1,
Description: "Added HEALTHCHECK instruction",
Before:      "",
After:       healthcheck,
})
break
}
}
case "DF002":
for i := len(lines) - 1; i >= 0; i-- {
trimmed := strings.TrimSpace(lines[i])
if strings.HasPrefix(trimmed, "CMD") || strings.HasPrefix(trimmed, "ENTRYPOINT") {
userLine := "USER 1000:1000"
lines = insertLine(lines, i, userLine)
changes = append(changes, Change{
LineNumber:  i + 1,
Description: "Added non-root USER directive",
Before:      "",
After:       userLine,
})
break
}
}
}
}

return &FixResult{
OptimizedContent: strings.Join(lines, "\n"),
Changes:          changes,
OriginalContent:  original,
}
}

var fromFixRegex = regexp.MustCompile(`^(FROM\s+)([^\s:]+)(\s+.*)?$`)

func fixLatestTag(line string, lineNum int) (string, *Change) {
matches := fromFixRegex.FindStringSubmatch(line)
if matches == nil {
return line, nil
}

prefix := matches[1]
image := matches[2]
suffix := ""
if len(matches) > 3 {
suffix = matches[3]
}

version := getDefaultVersion(image)
fixed := prefix + image + ":" + version + suffix

return fixed, &Change{
LineNumber:  lineNum,
Description: "Pinned image to specific version",
Before:      line,
After:       fixed,
}
}

func getDefaultVersion(image string) string {
defaults := map[string]string{
"node": "20-alpine", "python": "3.11-slim", "golang": "1.21-alpine",
"ubuntu": "22.04", "debian": "bookworm-slim", "alpine": "3.18",
"nginx": "1.25-alpine", "redis": "7-alpine", "postgres": "16-alpine",
}
if v, ok := defaults[image]; ok {
return v
}
return "latest"
}

func insertLine(lines []string, index int, line string) []string {
lines = append(lines, "")
copy(lines[index+1:], lines[index:])
lines[index] = line
return lines
}

// GenerateDiff generates a unified diff of changes
func (r *FixResult) GenerateDiff() string {
if len(r.Changes) == 0 {
return "No changes made."
}

var sb strings.Builder
sb.WriteString("--- original\n")
sb.WriteString("+++ optimized\n")

for _, change := range r.Changes {
sb.WriteString(strings.Repeat("-", 40) + "\n")
if change.Before != "" {
sb.WriteString("- " + change.Before + "\n")
}
sb.WriteString("+ " + change.After + "\n")
}

return sb.String()
}
