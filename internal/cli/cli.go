package cli

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/fatih/color"
	"github.com/spf13/cobra"
	"github.com/thirukguru/docker-review/internal/analyzer"
	"github.com/thirukguru/docker-review/internal/fixer"
	"github.com/thirukguru/docker-review/internal/output"
	"github.com/thirukguru/docker-review/internal/rules"
	"github.com/thirukguru/docker-review/internal/types"
)

var (
	noColor        bool
	verbose        bool
	jsonOutput     bool
	ciMode         bool
	failOn         string
	severity       string
	summaryOnly    bool
	estimateImpact bool
	fix            bool
	fixOutput      string
	showDiff       bool
)

// Version info - set by main.go
var (
	version   = "dev"
	buildTime = "unknown"
)

// SetVersion sets version info from main
func SetVersion(v, bt string) {
	version = v
	buildTime = bt
}

var rootCmd = &cobra.Command{
	Use:   "docker-review",
	Short: "A fast CLI tool for analyzing Dockerfiles and docker-compose files",
	Long: `Docker Review analyzes your Docker configurations like a Senior DevOps Engineer.
It detects performance issues, security vulnerabilities, and maintainability problems,
providing actionable suggestions and impact estimates.`,
	PersistentPreRun: func(cmd *cobra.Command, args []string) {
		if noColor {
			color.NoColor = true
		}
	},
}

var analyzeCmd = &cobra.Command{
	Use:   "analyze [path]",
	Short: "Analyze a Dockerfile or docker-compose.yml",
	Args:  cobra.ExactArgs(1),
	RunE:  runAnalyze,
}

var rulesCmd = &cobra.Command{
	Use:   "rules",
	Short: "List all available rules",
	Run: func(cmd *cobra.Command, args []string) {
		rules.PrintAllRules()
	},
}

var explainCmd = &cobra.Command{
	Use:   "explain [rule-id]",
	Short: "Explain a specific rule",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		rule := rules.GetRuleByID(args[0])
		if rule == nil {
			return fmt.Errorf("unknown rule: %s", args[0])
		}
		fmt.Println(rule.Explain())
		return nil
	},
}

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print version information",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("docker-review %s\n", version)
		fmt.Printf("Built: %s\n", buildTime)
	},
}

func init() {
	rootCmd.PersistentFlags().BoolVar(&noColor, "no-color", false, "Disable colored output")
	rootCmd.PersistentFlags().BoolVarP(&verbose, "verbose", "v", false, "Enable verbose output")

	analyzeCmd.Flags().BoolVar(&jsonOutput, "json", false, "Output in JSON format")
	analyzeCmd.Flags().BoolVar(&ciMode, "ci", false, "CI mode with exit codes")
	analyzeCmd.Flags().StringVar(&failOn, "fail-on", "critical", "Minimum severity to fail on")
	analyzeCmd.Flags().StringVar(&severity, "severity", "", "Filter issues by minimum severity")
	analyzeCmd.Flags().BoolVar(&summaryOnly, "summary-only", false, "Show only summary")
	analyzeCmd.Flags().BoolVar(&estimateImpact, "estimate-impact", false, "Show impact estimates")
	analyzeCmd.Flags().BoolVar(&fix, "fix", false, "Generate optimized Dockerfile")
	analyzeCmd.Flags().StringVar(&fixOutput, "fix-output", "", "Output path for fixed file")
	analyzeCmd.Flags().BoolVar(&showDiff, "diff", false, "Show diff of changes")

	rootCmd.AddCommand(analyzeCmd)
	rootCmd.AddCommand(rulesCmd)
	rootCmd.AddCommand(explainCmd)
	rootCmd.AddCommand(versionCmd)
}

func Execute() error {
	return rootCmd.Execute()
}

func runAnalyze(cmd *cobra.Command, args []string) error {
	path := args[0]

	absPath, err := filepath.Abs(path)
	if err != nil {
		return fmt.Errorf("invalid path: %w", err)
	}

	info, err := os.Stat(absPath)
	if err != nil {
		return fmt.Errorf("cannot access path: %w", err)
	}

	report, err := analyzer.Analyze(absPath, info.IsDir())
	if err != nil {
		return err
	}

	if severity != "" {
		sev := types.ParseSeverity(severity)
		report.Issues = filterBySeverity(report.Issues, sev)
	}

	if fix {
		if report.FileType != "dockerfile" {
			return fmt.Errorf("--fix only works with Dockerfiles")
		}
		result := fixer.Fix(absPath, report.Issues)

		if showDiff {
			fmt.Println(result.GenerateDiff())
		}

		outputPath := fixOutput
		if outputPath == "" {
			outputPath = "Dockerfile.optimized"
		}

		if outputPath == "-" {
			fmt.Println(result.OptimizedContent)
		} else {
			if err := os.WriteFile(outputPath, []byte(result.OptimizedContent), 0644); err != nil {
				return fmt.Errorf("error writing optimized Dockerfile: %w", err)
			}
			fmt.Printf("✓ Optimized Dockerfile written to: %s\n", outputPath)
			fmt.Printf("  %d change(s) made:\n", len(result.Changes))
			for _, change := range result.Changes {
				fmt.Printf("    Line %d: %s\n", change.LineNumber, change.Description)
			}
		}
		return nil
	}

	if jsonOutput {
		fmt.Println(output.FormatJSON(report))
	} else if summaryOnly {
		fmt.Printf("Issues found: %d\n", len(report.Issues))
		fmt.Printf("  Critical: %d\n", countBySeverity(report.Issues, types.Critical))
		fmt.Printf("  Warning: %d\n", countBySeverity(report.Issues, types.Warning))
		fmt.Printf("  Suggestion: %d\n", countBySeverity(report.Issues, types.Suggestion))
	} else {
		fmt.Println(output.FormatTerminal(report, verbose, estimateImpact))
	}

	if ciMode {
		failSev := types.ParseSeverity(failOn)
		for _, issue := range report.Issues {
			if issue.Severity >= failSev {
				os.Exit(1)
			}
		}
	}

	return nil
}

func filterBySeverity(issues []types.Issue, minSev types.Severity) []types.Issue {
	var filtered []types.Issue
	for _, issue := range issues {
		if issue.Severity >= minSev {
			filtered = append(filtered, issue)
		}
	}
	return filtered
}

func countBySeverity(issues []types.Issue, sev types.Severity) int {
	count := 0
	for _, issue := range issues {
		if issue.Severity == sev {
			count++
		}
	}
	return count
}
