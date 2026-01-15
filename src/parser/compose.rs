use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct ComposeFile {
    pub version: Option<String>,
    pub services: Option<HashMap<String, Service>>,
    pub volumes: Option<HashMap<String, serde_yaml::Value>>,
    pub networks: Option<HashMap<String, serde_yaml::Value>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Service {
    pub image: Option<String>,
    pub build: Option<BuildConfig>,
    pub environment: Option<Environment>,
    pub env_file: Option<EnvFile>,
    pub ports: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
    pub depends_on: Option<DependsOn>,
    pub restart: Option<String>,
    pub privileged: Option<bool>,
    pub user: Option<String>,
    pub command: Option<serde_yaml::Value>,
    pub entrypoint: Option<serde_yaml::Value>,
    pub healthcheck: Option<HealthCheck>,
    pub deploy: Option<Deploy>,
    pub mem_limit: Option<String>,
    pub cpus: Option<f64>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    pub security_opt: Option<Vec<String>>,
    pub networks: Option<serde_yaml::Value>,
    pub labels: Option<serde_yaml::Value>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum BuildConfig {
    Simple(String),
    Extended {
        context: Option<String>,
        dockerfile: Option<String>,
        args: Option<HashMap<String, String>>,
        target: Option<String>,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Environment {
    List(Vec<String>),
    Map(HashMap<String, Option<String>>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum EnvFile {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum DependsOn {
    Simple(Vec<String>),
    Extended(HashMap<String, DependsOnCondition>),
}

#[derive(Debug, Deserialize, Clone)]
pub struct DependsOnCondition {
    pub condition: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HealthCheck {
    pub test: Option<serde_yaml::Value>,
    pub interval: Option<String>,
    pub timeout: Option<String>,
    pub retries: Option<i32>,
    pub start_period: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Deploy {
    pub replicas: Option<i32>,
    pub resources: Option<Resources>,
    pub restart_policy: Option<RestartPolicy>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Resources {
    pub limits: Option<ResourceSpec>,
    pub reservations: Option<ResourceSpec>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResourceSpec {
    pub cpus: Option<String>,
    pub memory: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RestartPolicy {
    pub condition: Option<String>,
    pub delay: Option<String>,
    pub max_attempts: Option<i32>,
    pub window: Option<String>,
}

pub struct ComposeParser;

impl ComposeParser {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<ComposeFile, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&path)?;
        Self::parse_content(&content)
    }

    pub fn parse_content(content: &str) -> Result<ComposeFile, Box<dyn std::error::Error>> {
        let compose: ComposeFile = serde_yaml::from_str(content)?;
        Ok(compose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_compose() {
        let content = r#"
version: "3.8"
services:
  web:
    image: nginx:latest
    ports:
      - "80:80"
  db:
    image: postgres:13
    environment:
      POSTGRES_PASSWORD: secret
"#;
        let compose = ComposeParser::parse_content(content).unwrap();
        assert!(compose.services.is_some());
        let services = compose.services.unwrap();
        assert_eq!(services.len(), 2);
        assert!(services.contains_key("web"));
        assert!(services.contains_key("db"));
    }

    #[test]
    fn test_parse_compose_with_build() {
        let content = r#"
version: "3"
services:
  app:
    build:
      context: .
      dockerfile: Dockerfile.prod
"#;
        let compose = ComposeParser::parse_content(content).unwrap();
        let services = compose.services.unwrap();
        let app = services.get("app").unwrap();
        assert!(app.build.is_some());
    }
}
