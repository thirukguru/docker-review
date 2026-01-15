use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Nice to have improvements
    Suggestion,
    /// Should fix for better practices
    Warning,
    /// Must fix - critical issues
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Critical => write!(f, "Critical"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Suggestion => write!(f, "Suggestion"),
        }
    }
}

impl Severity {
    pub fn color(&self) -> colored::Color {
        match self {
            Severity::Critical => colored::Color::Red,
            Severity::Warning => colored::Color::Yellow,
            Severity::Suggestion => colored::Color::Blue,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Severity::Critical => "✗",
            Severity::Warning => "⚠",
            Severity::Suggestion => "ℹ",
        }
    }
}
