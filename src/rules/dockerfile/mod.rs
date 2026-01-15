mod latest_tag;
mod root_user;
mod no_dockerignore;
mod layer_order;
mod healthcheck;
mod secrets_in_env;
mod version_pinning;
mod multistage;
mod large_base_image;
mod curl_bash;

pub use latest_tag::LatestTagRule;
pub use root_user::RootUserRule;
pub use no_dockerignore::NoDockerignoreRule;
pub use layer_order::LayerOrderRule;
pub use healthcheck::HealthcheckRule;
pub use secrets_in_env::SecretsInEnvRule;
pub use version_pinning::VersionPinningRule;
pub use multistage::MultistageRule;
pub use large_base_image::LargeBaseImageRule;
pub use curl_bash::CurlBashRule;

use super::{Rule, Issue, Severity, ImpactEstimate};
use crate::parser::DockerfileParser;

/// Trait for Dockerfile-specific rules
pub trait DockerfileRule: Rule {
    fn check(&self, parser: &DockerfileParser, context_dir: Option<&std::path::Path>) -> Vec<Issue>;
}
