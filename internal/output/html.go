package output

import (
	"fmt"
	"html"
	"strings"

	"github.com/thirukguru/docker-review/internal/analyzer"
	"github.com/thirukguru/docker-review/internal/types"
)

// FormatHTML generates a standalone HTML report
func FormatHTML(report *analyzer.Report) string {
	var sb strings.Builder

	sb.WriteString(`<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Docker Review Report</title>
    <style>
        :root {
            --bg-primary: #1a1a2e;
            --bg-secondary: #16213e;
            --bg-card: #0f3460;
            --text-primary: #eaeaea;
            --text-secondary: #a0a0a0;
            --accent: #e94560;
            --success: #00d9a0;
            --warning: #ffc107;
            --error: #e94560;
            --info: #00b4d8;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: var(--bg-primary);
            color: var(--text-primary);
            line-height: 1.6;
            padding: 2rem;
        }
        .container { max-width: 1000px; margin: 0 auto; }
        h1 { 
            font-size: 2rem; 
            margin-bottom: 0.5rem;
            background: linear-gradient(90deg, var(--accent), var(--info));
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        .file-path { color: var(--text-secondary); margin-bottom: 2rem; font-family: monospace; }
        .scores {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }
        .score-card {
            background: var(--bg-secondary);
            padding: 1.5rem;
            border-radius: 12px;
            text-align: center;
        }
        .score-card h3 { font-size: 0.9rem; color: var(--text-secondary); margin-bottom: 0.5rem; }
        .score-value { font-size: 2.5rem; font-weight: bold; }
        .score-value.high { color: var(--success); }
        .score-value.medium { color: var(--warning); }
        .score-value.low { color: var(--error); }
        .bar {
            height: 8px;
            background: var(--bg-card);
            border-radius: 4px;
            overflow: hidden;
            margin-top: 0.5rem;
        }
        .bar-fill { height: 100%; border-radius: 4px; }
        .bar-fill.high { background: var(--success); }
        .bar-fill.medium { background: var(--warning); }
        .bar-fill.low { background: var(--error); }
        .summary {
            background: var(--bg-secondary);
            padding: 1rem 1.5rem;
            border-radius: 12px;
            margin-bottom: 2rem;
            display: flex;
            gap: 2rem;
        }
        .summary-item { display: flex; align-items: center; gap: 0.5rem; }
        .summary-count { font-weight: bold; font-size: 1.2rem; }
        .summary-count.critical { color: var(--error); }
        .summary-count.warning { color: var(--warning); }
        .summary-count.suggestion { color: var(--info); }
        .issues { margin-top: 2rem; }
        .issue {
            background: var(--bg-secondary);
            border-left: 4px solid;
            padding: 1rem 1.5rem;
            margin-bottom: 1rem;
            border-radius: 0 8px 8px 0;
        }
        .issue.critical { border-color: var(--error); }
        .issue.warning { border-color: var(--warning); }
        .issue.suggestion { border-color: var(--info); }
        .issue-header { display: flex; justify-content: space-between; margin-bottom: 0.5rem; }
        .issue-id { 
            font-family: monospace; 
            font-weight: bold;
            padding: 0.2rem 0.5rem;
            border-radius: 4px;
            font-size: 0.85rem;
        }
        .issue.critical .issue-id { background: var(--error); color: white; }
        .issue.warning .issue-id { background: var(--warning); color: black; }
        .issue.suggestion .issue-id { background: var(--info); color: white; }
        .issue-line { color: var(--text-secondary); font-family: monospace; }
        .issue-name { font-weight: 600; margin-bottom: 0.5rem; }
        .issue-desc { color: var(--text-secondary); }
        .issue-fix {
            margin-top: 0.75rem;
            padding: 0.75rem;
            background: var(--bg-card);
            border-radius: 6px;
            font-size: 0.9rem;
        }
        .issue-fix::before { content: '💡 '; }
        footer {
            margin-top: 3rem;
            text-align: center;
            color: var(--text-secondary);
            font-size: 0.85rem;
        }
        footer a { color: var(--info); text-decoration: none; }
        @media print {
            body { background: white; color: black; }
            .score-card, .issue, .summary { border: 1px solid #ddd; }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🐳 Docker Review Report</h1>
        <div class="file-path">`)
	sb.WriteString(html.EscapeString(report.FilePath))
	sb.WriteString(`</div>

        <div class="scores">`)

	// Score cards
	scores := []struct {
		name  string
		value int
	}{
		{"Security", report.Scores.Security},
		{"Performance", report.Scores.Performance},
		{"Maintainability", report.Scores.Maintainability},
		{"Overall", report.Scores.Overall},
	}
	for _, s := range scores {
		class := "low"
		if s.value >= 7 {
			class = "high"
		} else if s.value >= 4 {
			class = "medium"
		}
		sb.WriteString(fmt.Sprintf(`
            <div class="score-card">
                <h3>%s</h3>
                <div class="score-value %s">%d/10</div>
                <div class="bar"><div class="bar-fill %s" style="width: %d%%"></div></div>
            </div>`, s.name, class, s.value, class, s.value*10))
	}

	sb.WriteString(`</div>

        <div class="summary">`)

	// Count by severity
	critCount, warnCount, suggCount := 0, 0, 0
	for _, issue := range report.Issues {
		switch issue.Severity {
		case types.Critical:
			critCount++
		case types.Warning:
			warnCount++
		default:
			suggCount++
		}
	}
	sb.WriteString(fmt.Sprintf(`
            <div class="summary-item"><span class="summary-count critical">%d</span> Critical</div>
            <div class="summary-item"><span class="summary-count warning">%d</span> Warnings</div>
            <div class="summary-item"><span class="summary-count suggestion">%d</span> Suggestions</div>
        </div>`, critCount, warnCount, suggCount))

	sb.WriteString(`<div class="issues">`)

	// Issues grouped by severity
	for _, sev := range []types.Severity{types.Critical, types.Warning, types.Suggestion} {
		for _, issue := range report.Issues {
			if issue.Severity != sev {
				continue
			}
			class := "suggestion"
			if sev == types.Critical {
				class = "critical"
			}
			if sev == types.Warning {
				class = "warning"
			}

			sb.WriteString(fmt.Sprintf(`
            <div class="issue %s">
                <div class="issue-header">
                    <span class="issue-id">%s</span>
                    <span class="issue-line">Line %d</span>
                </div>
                <div class="issue-name">%s</div>
                <div class="issue-desc">%s</div>
                <div class="issue-fix">%s</div>
            </div>`, class, issue.RuleID, issue.Line,
				html.EscapeString(issue.Name),
				html.EscapeString(issue.Description),
				html.EscapeString(issue.Fix)))
		}
	}

	sb.WriteString(`
        </div>
        <footer>
            Generated by <a href="https://github.com/thirukguru/docker-review">docker-review</a>
            <br>To convert to PDF: Open in browser → Print → Save as PDF
        </footer>
    </div>
</body>
</html>`)

	return sb.String()
}
