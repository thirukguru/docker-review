use super::Severity;
use serde::{Deserialize, Serialize};

/// Represents a detected issue in a Docker configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub rule_id: String,
    pub rule_name: String,
    pub severity: Severity,
    pub line_number: Option<usize>,
    pub message: String,
    pub fix_suggestion: Option<String>,
    pub impact: Option<ImpactEstimate>,
}

/// Impact estimation for an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactEstimate {
    pub build_time_improvement: Option<String>,
    pub image_size_reduction: Option<String>,
    pub security_improvement: Option<String>,
    pub reliability_improvement: Option<String>,
}

/// Trait for implementing rules
pub trait Rule: Send + Sync {
    /// Unique identifier for the rule (e.g., "DF001")
    fn id(&self) -> &'static str;
    
    /// Human-readable name
    fn name(&self) -> &'static str;
    
    /// Severity level
    fn severity(&self) -> Severity;
    
    /// Short description
    fn description(&self) -> &'static str;
    
    /// Detailed rationale for why this rule exists
    fn rationale(&self) -> &'static str;
    
    /// Suggested fix (optional)
    fn fix_suggestion(&self) -> Option<&'static str> {
        None
    }
    
    /// Impact estimate
    fn impact(&self) -> Option<ImpactEstimate> {
        None
    }
    
    /// Generate a detailed explanation of the rule
    fn explain(&self) -> String {
        let mut explanation = String::new();
        explanation.push_str(&format!("Rule: {} ({})\n", self.name(), self.id()));
        explanation.push_str(&format!("Severity: {}\n\n", self.severity()));
        explanation.push_str(&format!("Description:\n  {}\n\n", self.description()));
        explanation.push_str(&format!("Rationale:\n  {}\n", self.rationale()));
        
        if let Some(fix) = self.fix_suggestion() {
            explanation.push_str(&format!("\nSuggested Fix:\n  {}\n", fix));
        }
        
        if let Some(impact) = self.impact() {
            explanation.push_str("\nImpact:\n");
            if let Some(build) = &impact.build_time_improvement {
                explanation.push_str(&format!("  Build time: {}\n", build));
            }
            if let Some(size) = &impact.image_size_reduction {
                explanation.push_str(&format!("  Image size: {}\n", size));
            }
            if let Some(security) = &impact.security_improvement {
                explanation.push_str(&format!("  Security: {}\n", security));
            }
            if let Some(reliability) = &impact.reliability_improvement {
                explanation.push_str(&format!("  Reliability: {}\n", reliability));
            }
        }
        
        explanation
    }
}
