use clap::{Parser, Subcommand, ValueEnum};
use crate::rules::Severity;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "docker-review")]
#[command(author, version, about = "Analyze Dockerfiles and docker-compose files for best practices")]
#[command(long_about = "A fast, offline-first CLI tool that reviews Docker configurations.\n\n\
    It detects performance issues, security vulnerabilities, and maintainability problems,\n\
    providing actionable suggestions and impact estimates.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze a Dockerfile or docker-compose file
    Analyze(AnalyzeArgs),

    /// List all available rules
    Rules,

    /// Explain a specific rule
    Explain {
        /// Rule ID to explain (e.g., DF001, DC001)
        rule_id: String,
    },
}

#[derive(Parser)]
pub struct AnalyzeArgs {
    /// Path to Dockerfile or docker-compose.yml
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,

    /// Generate HTML report
    #[arg(long)]
    pub html: bool,

    /// Generate PDF report
    #[arg(long)]
    pub pdf: bool,

    /// Output file path for reports
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Minimum severity to report
    #[arg(long, value_enum)]
    pub severity: Option<Severity>,

    /// Enable CI mode (machine-readable output)
    #[arg(long)]
    pub ci: bool,

    /// Exit with non-zero code if issues at this severity or higher are found
    #[arg(long, value_enum)]
    pub fail_on: Option<Severity>,

    /// Show only summary, not individual issues
    #[arg(long)]
    pub summary_only: bool,

    /// Show estimated impact of issues
    #[arg(long)]
    pub estimate_impact: bool,
}
