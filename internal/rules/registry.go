package rules

import (
	"fmt"

	"github.com/thirukguru/docker-review/internal/rules/compose"
	"github.com/thirukguru/docker-review/internal/rules/dockerfile"
	"github.com/thirukguru/docker-review/internal/types"
)

// Re-export types for convenience
type Issue = types.Issue
type Severity = types.Severity

const (
	Suggestion = types.Suggestion
	Warning    = types.Warning
	Critical   = types.Critical
)

func ParseSeverity(s string) Severity {
	return types.ParseSeverity(s)
}

var (
	dockerfileRules []types.DockerfileRule
	composeRules    []types.ComposeRule
	allRulesMap     map[string]types.Rule
)

func init() {
	// Initialize rule slices
	dockerfileRules = []types.DockerfileRule{
		dockerfile.NewLatestTagRule(),
		dockerfile.NewRootUserRule(),
		dockerfile.NewNoDockerignoreRule(),
		dockerfile.NewLayerOrderRule(),
		dockerfile.NewHealthcheckRule(),
		dockerfile.NewSecretsInEnvRule(),
		dockerfile.NewVersionPinningRule(),
		dockerfile.NewMultistageRule(),
		dockerfile.NewLargeBaseImageRule(),
		dockerfile.NewCurlBashRule(),
		dockerfile.NewLayerOptimizationRule(),
		dockerfile.NewMLStackRule(),
	}

	composeRules = []types.ComposeRule{
		compose.NewRestartPolicyRule(),
		compose.NewPrivilegedRule(),
		compose.NewResourceLimitsRule(),
		compose.NewLatestTagRule(),
		compose.NewHardcodedSecretsRule(),
	}

	// Build lookup map
	allRulesMap = make(map[string]types.Rule)
	for _, r := range dockerfileRules {
		allRulesMap[r.ID()] = r
	}
	for _, r := range composeRules {
		allRulesMap[r.ID()] = r
	}
}

// GetDockerfileRules returns all Dockerfile rules
func GetDockerfileRules() []types.DockerfileRule {
	return dockerfileRules
}

// GetComposeRules returns all compose rules
func GetComposeRules() []types.ComposeRule {
	return composeRules
}

// GetRuleByID returns a rule by its ID
func GetRuleByID(id string) types.Rule {
	return allRulesMap[id]
}

// PrintAllRules prints all available rules
func PrintAllRules() {
	fmt.Println("Dockerfile Rules (DF001-DF012):")
	fmt.Println("================================")
	for _, r := range dockerfileRules {
		fmt.Printf("  %-6s %-30s %s\n", r.ID(), r.Name(), r.Severity())
	}
	fmt.Println()
	fmt.Println("Docker Compose Rules (DC001-DC005):")
	fmt.Println("====================================")
	for _, r := range composeRules {
		fmt.Printf("  %-6s %-30s %s\n", r.ID(), r.Name(), r.Severity())
	}
}
