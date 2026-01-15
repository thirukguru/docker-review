use std::path::Path;

/// Check if a .dockerignore file exists in the given directory
pub fn check_dockerignore<P: AsRef<Path>>(dir: P) -> bool {
    let dockerignore_path = dir.as_ref().join(".dockerignore");
    dockerignore_path.exists()
}

/// Parse .dockerignore patterns from a file
pub fn parse_dockerignore<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(parse_patterns(&content))
}

fn parse_patterns(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_patterns() {
        let content = r#"
# Comment
node_modules
*.log

.git
target/
"#;
        let patterns = parse_patterns(content);
        assert_eq!(patterns.len(), 4);
        assert!(patterns.contains(&"node_modules".to_string()));
        assert!(patterns.contains(&".git".to_string()));
    }
}
