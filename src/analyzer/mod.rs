use crate::parser::{DockerfileParser, ComposeParser};
use crate::rules::{Issue, Severity};
use crate::rules::dockerfile::*;
use crate::rules::compose::*;
use crate::scoring::{Scores, calculate_scores};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzerError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse compose file: {0}")]
    ComposeParseError(String),
    #[error("Unknown file type: {0}")]
    UnknownFileType(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub file_path: String,
    pub issues: Vec<Issue>,
    pub scores: Scores,
}

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze<P: AsRef<Path>>(&self, path: P) -> Result<Report, AnalyzerError> {
        let path = path.as_ref();
        
        // Determine file type and analyze
        if !path.exists() {
            return Err(AnalyzerError::FileNotFound(path.display().to_string()));
        }

        if path.is_dir() {
            // Look for Dockerfile or docker-compose.yml in directory
            let dockerfile = path.join("Dockerfile");
            let compose = path.join("docker-compose.yml");
            let compose_alt = path.join("docker-compose.yaml");
            let compose_short = path.join("compose.yml");
            let compose_short_alt = path.join("compose.yaml");

            if dockerfile.exists() {
                return self.analyze_dockerfile(&dockerfile, Some(path));
            } else if compose.exists() {
                return self.analyze_compose(&compose);
            } else if compose_alt.exists() {
                return self.analyze_compose(&compose_alt);
            } else if compose_short.exists() {
                return self.analyze_compose(&compose_short);
            } else if compose_short_alt.exists() {
                return self.analyze_compose(&compose_short_alt);
            } else {
                return Err(AnalyzerError::FileNotFound(
                    "No Dockerfile or docker-compose.yml found in directory".to_string()
                ));
            }
        }

        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if file_name == "Dockerfile" || file_name.starts_with("Dockerfile.") || file_name.ends_with("_dockerfile") || file_name.ends_with("dockerfile") {
            self.analyze_dockerfile(path, path.parent())
        } else if file_name.contains("compose") && (file_name.ends_with(".yml") || file_name.ends_with(".yaml")) {
            self.analyze_compose(path)
        } else {
            // Try to detect from content
            let content = std::fs::read_to_string(path)?;
            if content.contains("FROM ") && (content.contains("RUN ") || content.contains("COPY ") || content.contains("CMD ")) {
                self.analyze_dockerfile(path, path.parent())
            } else if content.contains("services:") || content.contains("version:") {
                self.analyze_compose(path)
            } else {
                Err(AnalyzerError::UnknownFileType(path.display().to_string()))
            }
        }
    }

    fn analyze_dockerfile<P: AsRef<Path>>(&self, path: P, context_dir: Option<&Path>) -> Result<Report, AnalyzerError> {
        let path = path.as_ref();
        let parser = DockerfileParser::parse(path)?;
        
        let mut issues = Vec::new();
        
        // Run all Dockerfile rules directly
        issues.extend(LatestTagRule.check(&parser, context_dir));
        issues.extend(RootUserRule.check(&parser, context_dir));
        issues.extend(NoDockerignoreRule.check(&parser, context_dir));
        issues.extend(LayerOrderRule.check(&parser, context_dir));
        issues.extend(HealthcheckRule.check(&parser, context_dir));
        issues.extend(SecretsInEnvRule.check(&parser, context_dir));
        issues.extend(VersionPinningRule.check(&parser, context_dir));
        issues.extend(MultistageRule.check(&parser, context_dir));
        issues.extend(LargeBaseImageRule.check(&parser, context_dir));
        issues.extend(CurlBashRule.check(&parser, context_dir));

        // Sort by severity (critical first)
        issues.sort_by(|a, b| b.severity.cmp(&a.severity));

        let scores = calculate_scores(&issues);

        Ok(Report {
            file_path: path.display().to_string(),
            issues,
            scores,
        })
    }

    fn analyze_compose<P: AsRef<Path>>(&self, path: P) -> Result<Report, AnalyzerError> {
        let path = path.as_ref();
        let compose = ComposeParser::parse(path)
            .map_err(|e| AnalyzerError::ComposeParseError(e.to_string()))?;
        
        let mut issues = Vec::new();
        
        // Run all Compose rules directly
        issues.extend(RestartPolicyRule.check(&compose));
        issues.extend(PrivilegedRule.check(&compose));
        issues.extend(ResourceLimitsRule.check(&compose));
        issues.extend(ComposeLatestTagRule.check(&compose));
        issues.extend(HardcodedSecretsRule.check(&compose));

        // Sort by severity (critical first)
        issues.sort_by(|a, b| b.severity.cmp(&a.severity));

        let scores = calculate_scores(&issues);

        Ok(Report {
            file_path: path.display().to_string(),
            issues,
            scores,
        })
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}
