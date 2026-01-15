mod terminal;
mod json;

pub use terminal::TerminalOutput;
pub use json::JsonOutput;

use crate::analyzer::Report;

pub trait OutputFormatter {
    fn format(&self, report: &Report) -> String;
}
