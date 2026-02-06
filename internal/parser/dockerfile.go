package parser

import (
"bufio"
"os"
"path/filepath"
"regexp"
"strings"

"github.com/thirukguru/docker-review/internal/types"
)

var instructionRegex = regexp.MustCompile(`^([A-Z]+)\s+(.*)$`)
var ignoreRegex = regexp.MustCompile(`(?i)#\s*docker-review:ignore\s+(.*)`)

// ParseDockerfile parses a Dockerfile into a DockerfileContext
func ParseDockerfile(path string) (*types.DockerfileContext, error) {
file, err := os.Open(path)
if err != nil {
return nil, err
}
defer file.Close()

ctx := &types.DockerfileContext{
Path:            path,
Lines:           []string{},
Instructions:    []types.Instruction{},
HasDockerignore: checkDockerignore(path),
IgnoredRules:    make(map[int][]string), // line -> rule IDs to ignore
}

scanner := bufio.NewScanner(file)
lineNum := 0
var continuationLine string
var continuationStart int
var pendingIgnores []string

for scanner.Scan() {
lineNum++
line := scanner.Text()
ctx.Lines = append(ctx.Lines, line)

trimmed := strings.TrimSpace(line)

// Check for ignore comments
if matches := ignoreRegex.FindStringSubmatch(trimmed); matches != nil {
ruleIDs := strings.Fields(matches[1])
pendingIgnores = append(pendingIgnores, ruleIDs...)
continue
}

if trimmed == "" || strings.HasPrefix(trimmed, "#") {
continue
}

// Handle line continuation
if strings.HasSuffix(trimmed, "\\") {
if continuationLine == "" {
continuationStart = lineNum
}
continuationLine += strings.TrimSuffix(trimmed, "\\") + " "
continue
}

actualLine := lineNum
if continuationLine != "" {
trimmed = continuationLine + trimmed
actualLine = continuationStart
continuationLine = ""
}

// Apply pending ignores to this line
if len(pendingIgnores) > 0 {
ctx.IgnoredRules[actualLine] = pendingIgnores
pendingIgnores = nil
}

if matches := instructionRegex.FindStringSubmatch(trimmed); matches != nil {
ctx.Instructions = append(ctx.Instructions, types.Instruction{
Name:      matches[1],
Arguments: matches[2],
Line:      actualLine,
Raw:       trimmed,
})
}
}

return ctx, scanner.Err()
}

func checkDockerignore(dockerfilePath string) bool {
dir := filepath.Dir(dockerfilePath)
_, err := os.Stat(filepath.Join(dir, ".dockerignore"))
return err == nil
}

// IsDockerfile checks if a file is a Dockerfile
func IsDockerfile(path string) bool {
base := strings.ToLower(filepath.Base(path))
return base == "dockerfile" || strings.HasPrefix(base, "dockerfile.")
}
