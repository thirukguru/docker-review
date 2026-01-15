use super::OutputFormatter;
use crate::analyzer::Report;

pub struct JsonOutput;

impl OutputFormatter for JsonOutput {
    fn format(&self, report: &Report) -> String {
        serde_json::to_string_pretty(report).unwrap_or_else(|e| {
            format!("{{\"error\": \"Failed to serialize report: {}\"}}", e)
        })
    }
}
