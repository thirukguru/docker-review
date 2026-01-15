mod restart_policy;
mod privileged;
mod resource_limits;
mod latest_tag;
mod hardcoded_secrets;

pub use restart_policy::RestartPolicyRule;
pub use privileged::PrivilegedRule;
pub use resource_limits::ResourceLimitsRule;
pub use latest_tag::ComposeLatestTagRule;
pub use hardcoded_secrets::HardcodedSecretsRule;

use super::{Rule, Issue, Severity, ImpactEstimate};
use crate::parser::ComposeFile;

/// Trait for Compose-specific rules
pub trait ComposeRule: Rule {
    fn check(&self, compose: &ComposeFile) -> Vec<Issue>;
}
