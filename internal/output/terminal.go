package output

import (
	"fmt"
	"strings"

	"github.com/fatih/color"
	"github.com/thirukguru/docker-review/internal/analyzer"
	"github.com/thirukguru/docker-review/internal/types"
)

var (
	criticalColor   = color.New(color.FgRed, color.Bold)
	warningColor    = color.New(color.FgYellow)
	suggestionColor = color.New(color.FgCyan)
	headerColor     = color.New(color.FgWhite, color.Bold)
	successColor    = color.New(color.FgGreen)
)

// FormatTerminal formats the report for terminal output
func FormatTerminal(report *analyzer.Report, verbose bool, showImpact bool) string {
	var sb strings.Builder

	sb.WriteString("\n")
	sb.WriteString(headerColor.Sprint("Docker Review Report"))
	sb.WriteString("\n")
	sb.WriteString(fmt.Sprintf("File: %s\n\n", report.FilePath))

	sb.WriteString("📊 Scores\n")
	sb.WriteString(fmt.Sprintf("  Security:       %s %d/10\n", scoreBar(report.Scores.Security), report.Scores.Security))
	sb.WriteString(fmt.Sprintf("  Performance:    %s %d/10\n", scoreBar(report.Scores.Performance), report.Scores.Performance))
	sb.WriteString(fmt.Sprintf("  Maintainability:%s %d/10\n", scoreBar(report.Scores.Maintainability), report.Scores.Maintainability))
	sb.WriteString(fmt.Sprintf("  Overall:        %s %d/10\n\n", scoreBar(report.Scores.Overall), report.Scores.Overall))

	criticalCount := countSeverity(report.Issues, types.Critical)
	warningCount := countSeverity(report.Issues, types.Warning)
	suggestionCount := countSeverity(report.Issues, types.Suggestion)

	sb.WriteString("📋 Issues Summary\n")
	sb.WriteString(fmt.Sprintf("  %d Critical, %d Warnings, %d Suggestions\n\n",
		criticalCount, warningCount, suggestionCount))

	if criticalCount > 0 {
		sb.WriteString(criticalColor.Sprint("✗ Critical Issues\n"))
		for _, issue := range report.Issues {
			if issue.Severity == types.Critical {
				sb.WriteString(formatIssue(issue, verbose))
			}
		}
		sb.WriteString("\n")
	}

	if warningCount > 0 {
		sb.WriteString(warningColor.Sprint("⚠ Warnings\n"))
		for _, issue := range report.Issues {
			if issue.Severity == types.Warning {
				sb.WriteString(formatIssue(issue, verbose))
			}
		}
		sb.WriteString("\n")
	}

	if suggestionCount > 0 {
		sb.WriteString(suggestionColor.Sprint("💡 Suggestions\n"))
		for _, issue := range report.Issues {
			if issue.Severity == types.Suggestion {
				sb.WriteString(formatIssue(issue, verbose))
			}
		}
		sb.WriteString("\n")
	}

	if len(report.Issues) == 0 {
		sb.WriteString(successColor.Sprint("✓ No issues found!\n"))
	}

	return sb.String()
}

func scoreBar(score int) string {
	filled := score
	empty := 10 - score
	return strings.Repeat("█", filled) + strings.Repeat("░", empty)
}

func countSeverity(issues []types.Issue, sev types.Severity) int {
	count := 0
	for _, issue := range issues {
		if issue.Severity == sev {
			count++
		}
	}
	return count
}

func formatIssue(issue types.Issue, verbose bool) string {
	line := ""
	if issue.Line > 0 {
		line = fmt.Sprintf(":%d", issue.Line)
	}

	result := fmt.Sprintf("  [%s] %s %s\n", issue.RuleID, issue.Name, line)
	result += fmt.Sprintf("    %s\n", issue.Description)

	if verbose && issue.Fix != "" {
		result += fmt.Sprintf("    Fix: %s\n", issue.Fix)
	}

	return result
}
