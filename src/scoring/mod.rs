use crate::rules::{Issue, Severity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scores {
    pub performance: Score,
    pub security: Score,
    pub maintainability: Score,
    pub overall: Score,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub current: u8,
    pub potential: u8,
}

impl Score {
    pub fn display(&self) -> String {
        format!("{}/10 → {}/10", self.current, self.potential)
    }
}

pub fn calculate_scores(issues: &[Issue]) -> Scores {
    let critical_count = issues.iter().filter(|i| i.severity == Severity::Critical).count() as u8;
    let warning_count = issues.iter().filter(|i| i.severity == Severity::Warning).count() as u8;
    let suggestion_count = issues.iter().filter(|i| i.severity == Severity::Suggestion).count() as u8;

    // Calculate current scores (start at 10, deduct based on issues)
    let security_current = calculate_category_score(issues, &["DF001", "DF002", "DF006", "DF010", "DC002", "DC004", "DC005"]);
    let performance_current = calculate_category_score(issues, &["DF003", "DF004", "DF007", "DF008", "DF009"]);
    let maintainability_current = calculate_category_score(issues, &["DF005", "DC001", "DC003"]);

    // Overall is weighted average
    let overall_current = (security_current.saturating_mul(4) + performance_current.saturating_mul(3) + maintainability_current.saturating_mul(3)) / 10;

    Scores {
        performance: Score {
            current: performance_current,
            potential: 10,
        },
        security: Score {
            current: security_current,
            potential: 10,
        },
        maintainability: Score {
            current: maintainability_current,
            potential: 10,
        },
        overall: Score {
            current: overall_current,
            potential: 10,
        },
    }
}

fn calculate_category_score(issues: &[Issue], rule_ids: &[&str]) -> u8 {
    let mut score: i8 = 10;

    for issue in issues {
        if rule_ids.contains(&issue.rule_id.as_str()) {
            match issue.severity {
                Severity::Critical => score -= 3,
                Severity::Warning => score -= 2,
                Severity::Suggestion => score -= 1,
            }
        }
    }

    score.max(0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::ImpactEstimate;

    #[test]
    fn test_perfect_score_no_issues() {
        let issues: Vec<Issue> = vec![];
        let scores = calculate_scores(&issues);
        assert_eq!(scores.security.current, 10);
        assert_eq!(scores.performance.current, 10);
        assert_eq!(scores.maintainability.current, 10);
    }

    #[test]
    fn test_score_deduction() {
        let issues = vec![
            Issue {
                rule_id: "DF001".to_string(),
                rule_name: "Test".to_string(),
                severity: Severity::Critical,
                line_number: Some(1),
                message: "Test".to_string(),
                fix_suggestion: None,
                impact: None,
            },
        ];
        let scores = calculate_scores(&issues);
        assert_eq!(scores.security.current, 7); // 10 - 3 for critical
    }
}
