use super::Rule;
use super::dockerfile::*;
use super::compose::*;
use colored::Colorize;
use once_cell::sync::Lazy;

/// All Dockerfile rules
static DOCKERFILE_RULES: Lazy<Vec<Box<dyn Rule>>> = Lazy::new(|| {
    vec![
        Box::new(LatestTagRule),
        Box::new(RootUserRule),
        Box::new(NoDockerignoreRule),
        Box::new(LayerOrderRule),
        Box::new(HealthcheckRule),
        Box::new(SecretsInEnvRule),
        Box::new(VersionPinningRule),
        Box::new(MultistageRule),
        Box::new(LargeBaseImageRule),
        Box::new(CurlBashRule),
    ]
});

/// All Compose rules
static COMPOSE_RULES: Lazy<Vec<Box<dyn Rule>>> = Lazy::new(|| {
    vec![
        Box::new(RestartPolicyRule),
        Box::new(PrivilegedRule),
        Box::new(ResourceLimitsRule),
        Box::new(ComposeLatestTagRule),
        Box::new(HardcodedSecretsRule),
    ]
});

pub fn get_dockerfile_rules() -> &'static Vec<Box<dyn Rule>> {
    &DOCKERFILE_RULES
}

pub fn get_compose_rules() -> &'static Vec<Box<dyn Rule>> {
    &COMPOSE_RULES
}

pub fn get_all_rules() -> Vec<&'static dyn Rule> {
    let mut all: Vec<&dyn Rule> = Vec::new();
    for rule in DOCKERFILE_RULES.iter() {
        all.push(rule.as_ref());
    }
    for rule in COMPOSE_RULES.iter() {
        all.push(rule.as_ref());
    }
    all
}

pub fn get_rule_by_id(id: &str) -> Option<&'static dyn Rule> {
    let upper_id = id.to_uppercase();
    get_all_rules().into_iter().find(|r| r.id() == upper_id)
}

pub fn print_all_rules() {
    println!("{}", "Dockerfile Rules:".bold().underline());
    println!();
    
    for rule in DOCKERFILE_RULES.iter() {
        let severity_str = format!("[{}]", rule.severity());
        let colored_severity = match rule.severity() {
            super::Severity::Critical => severity_str.red(),
            super::Severity::Warning => severity_str.yellow(),
            super::Severity::Suggestion => severity_str.blue(),
        };
        println!(
            "  {} {} - {}",
            rule.id().bold(),
            colored_severity,
            rule.name()
        );
        println!("      {}", rule.description().dimmed());
        println!();
    }

    println!("{}", "Docker Compose Rules:".bold().underline());
    println!();
    
    for rule in COMPOSE_RULES.iter() {
        let severity_str = format!("[{}]", rule.severity());
        let colored_severity = match rule.severity() {
            super::Severity::Critical => severity_str.red(),
            super::Severity::Warning => severity_str.yellow(),
            super::Severity::Suggestion => severity_str.blue(),
        };
        println!(
            "  {} {} - {}",
            rule.id().bold(),
            colored_severity,
            rule.name()
        );
        println!("      {}", rule.description().dimmed());
        println!();
    }
}
