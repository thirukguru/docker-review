use super::OutputFormatter;
use crate::analyzer::Report;
use crate::rules::Severity;
use colored::Colorize;

pub struct TerminalOutput {
    verbose: bool,
    show_impact: bool,
}

impl TerminalOutput {
    pub fn new(verbose: bool, show_impact: bool) -> Self {
        Self { verbose, show_impact }
    }
}

impl OutputFormatter for TerminalOutput {
    fn format(&self, report: &Report) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!("\n{}\n", "Docker Review Report".bold().underline()));
        output.push_str(&format!("File: {}\n\n", report.file_path.cyan()));

        // Scores summary
        output.push_str(&format!("{}\n", "📊 Scores".bold()));
        output.push_str(&format!("  Security:       {}\n", format_score_bar(report.scores.security.current)));
        output.push_str(&format!("  Performance:    {}\n", format_score_bar(report.scores.performance.current)));
        output.push_str(&format!("  Maintainability:{}\n", format_score_bar(report.scores.maintainability.current)));
        output.push_str(&format!("  Overall:        {}\n\n", format_score_bar(report.scores.overall.current)));

        if report.issues.is_empty() {
            output.push_str(&format!("{}\n", "✅ No issues found! Great job!".green().bold()));
            return output;
        }

        // Group issues by severity
        let critical: Vec<_> = report.issues.iter()
            .filter(|i| i.severity == Severity::Critical)
            .collect();
        let warnings: Vec<_> = report.issues.iter()
            .filter(|i| i.severity == Severity::Warning)
            .collect();
        let suggestions: Vec<_> = report.issues.iter()
            .filter(|i| i.severity == Severity::Suggestion)
            .collect();

        // Summary line
        output.push_str(&format!("{}\n", "📋 Issues Summary".bold()));
        output.push_str(&format!(
            "  {} Critical, {} Warnings, {} Suggestions\n\n",
            critical.len().to_string().red().bold(),
            warnings.len().to_string().yellow().bold(),
            suggestions.len().to_string().blue().bold()
        ));

        // Critical issues
        if !critical.is_empty() {
            output.push_str(&format!("{}\n", "✗ Critical Issues".red().bold()));
            for issue in &critical {
                output.push_str(&format_issue(issue, self.verbose, self.show_impact));
            }
            output.push('\n');
        }

        // Warnings
        if !warnings.is_empty() {
            output.push_str(&format!("{}\n", "⚠ Warnings".yellow().bold()));
            for issue in &warnings {
                output.push_str(&format_issue(issue, self.verbose, self.show_impact));
            }
            output.push('\n');
        }

        // Suggestions
        if !suggestions.is_empty() {
            output.push_str(&format!("{}\n", "ℹ Suggestions".blue().bold()));
            for issue in &suggestions {
                output.push_str(&format_issue(issue, self.verbose, self.show_impact));
            }
            output.push('\n');
        }

        // Impact estimation
        if self.show_impact {
            output.push_str(&format!("{}\n", "📈 Estimated Impact".bold()));
            output.push_str("  Fixing all issues could improve:\n");
            output.push_str(&format!("  • Security score:       {} → {}\n", 
                report.scores.security.current, 
                report.scores.security.potential));
            output.push_str(&format!("  • Performance score:    {} → {}\n", 
                report.scores.performance.current, 
                report.scores.performance.potential));
            output.push_str(&format!("  • Maintainability score:{} → {}\n\n", 
                report.scores.maintainability.current, 
                report.scores.maintainability.potential));
        }

        output
    }
}

fn format_issue(issue: &crate::rules::Issue, verbose: bool, show_impact: bool) -> String {
    let mut s = String::new();
    
    let line_info = issue.line_number
        .map(|l| format!(":{}", l))
        .unwrap_or_default();
    
    let severity_badge = match issue.severity {
        Severity::Critical => format!("[{}]", issue.rule_id).red(),
        Severity::Warning => format!("[{}]", issue.rule_id).yellow(),
        Severity::Suggestion => format!("[{}]", issue.rule_id).blue(),
    };

    s.push_str(&format!("  {} {} {}\n", 
        severity_badge,
        issue.rule_name.bold(),
        line_info.dimmed()
    ));
    s.push_str(&format!("    {}\n", issue.message));

    if let Some(fix) = &issue.fix_suggestion {
        s.push_str(&format!("    {} {}\n", "Fix:".green(), fix));
    }

    if verbose {
        if let Some(impact) = &issue.impact {
            if show_impact {
                if let Some(ref build) = impact.build_time_improvement {
                    s.push_str(&format!("    {} {}\n", "Build:".dimmed(), build));
                }
                if let Some(ref size) = impact.image_size_reduction {
                    s.push_str(&format!("    {} {}\n", "Size:".dimmed(), size));
                }
                if let Some(ref security) = impact.security_improvement {
                    s.push_str(&format!("    {} {}\n", "Security:".dimmed(), security));
                }
            }
        }
    }

    s.push('\n');
    s
}

fn format_score_bar(score: u8) -> String {
    let filled = score as usize;
    let empty = 10 - filled;
    
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    let colored_bar = if score >= 8 {
        bar.green()
    } else if score >= 5 {
        bar.yellow()
    } else {
        bar.red()
    };
    
    format!("{} {}/10", colored_bar, score)
}
