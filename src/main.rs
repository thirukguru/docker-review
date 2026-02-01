use clap::Parser;
use docker_review::cli::{Cli, Commands};
use docker_review::analyzer::Analyzer;
use docker_review::output::{JsonOutput, TerminalOutput, OutputFormatter};
use docker_review::rules::Severity;
use docker_review::fixer::DockerfileFixer;
use docker_review::parser::DockerfileParser;
use std::process::ExitCode;
use std::path::PathBuf;

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Handle --no-color globally
    if cli.no_color {
        colored::control::set_override(false);
    }

    match cli.command {
        Commands::Analyze(args) => {
            let analyzer = Analyzer::new();
            
            match analyzer.analyze(&args.path) {
                Ok(report) => {
                    // Filter by severity if specified
                    let filtered_issues: Vec<_> = if let Some(ref min_severity) = args.severity {
                        report.issues.iter()
                            .filter(|issue| issue.severity >= *min_severity)
                            .cloned()
                            .collect()
                    } else {
                        report.issues.clone()
                    };

                    let filtered_report = docker_review::analyzer::Report {
                        issues: filtered_issues,
                        scores: report.scores.clone(),
                        file_path: report.file_path.clone(),
                    };

                    // Output format (skip if --fix is the primary action)
                    if !args.fix {
                        if args.json {
                            let output = JsonOutput;
                            println!("{}", output.format(&filtered_report));
                        } else if !args.summary_only {
                            let output = TerminalOutput::new(cli.verbose, args.estimate_impact);
                            println!("{}", output.format(&filtered_report));
                        }

                        // Summary for --summary-only or always show summary
                        if args.summary_only {
                            println!("Issues found: {}", filtered_report.issues.len());
                            println!("  Critical: {}", filtered_report.issues.iter().filter(|i| i.severity == Severity::Critical).count());
                            println!("  Warning: {}", filtered_report.issues.iter().filter(|i| i.severity == Severity::Warning).count());
                            println!("  Suggestion: {}", filtered_report.issues.iter().filter(|i| i.severity == Severity::Suggestion).count());
                        }
                    }

                    // Handle --fix flag
                    if args.fix {
                        // Only works for Dockerfiles
                        if let Ok(parser) = DockerfileParser::parse(&args.path) {
                            let fix_result = DockerfileFixer::fix(&parser, &report.issues);
                            
                            // Show diff if requested
                            if args.diff {
                                println!("{}", fix_result.generate_diff());
                            }
                            
                            // Determine output path
                            let output_path = args.fix_output.unwrap_or_else(|| {
                                PathBuf::from("Dockerfile.optimized")
                            });
                            
                            // Write or print the optimized Dockerfile
                            if output_path.as_os_str() == "-" {
                                // Output to stdout
                                println!("{}", fix_result.optimized_content);
                            } else {
                                match std::fs::write(&output_path, &fix_result.optimized_content) {
                                    Ok(_) => {
                                        println!("✓ Optimized Dockerfile written to: {}", output_path.display());
                                        println!("  {} change(s) made:", fix_result.changes.len());
                                        for change in &fix_result.changes {
                                            println!("    Line {}: {}", change.line_number, change.description);
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error writing optimized Dockerfile: {}", e);
                                        return ExitCode::from(1);
                                    }
                                }
                            }
                        } else {
                            eprintln!("Error: --fix only works with Dockerfiles, not compose files");
                            return ExitCode::from(1);
                        }
                    }

                    // Exit code for CI
                    if args.ci {
                        let fail_severity = args.fail_on.unwrap_or(Severity::Critical);
                        let has_failures = filtered_report.issues.iter()
                            .any(|issue| issue.severity >= fail_severity);
                        if has_failures {
                            return ExitCode::from(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return ExitCode::from(1);
                }
            }
        }
        Commands::Rules => {
            docker_review::rules::print_all_rules();
        }
        Commands::Explain { rule_id } => {
            if let Some(rule) = docker_review::rules::get_rule_by_id(&rule_id) {
                println!("{}", rule.explain());
            } else {
                eprintln!("Unknown rule: {}", rule_id);
                return ExitCode::from(1);
            }
        }
    }

    ExitCode::SUCCESS
}

