use regex::Regex;
use once_cell::sync::Lazy;
use std::fs;
use std::path::Path;

static INSTRUCTION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^(FROM|RUN|CMD|LABEL|EXPOSE|ENV|ADD|COPY|ENTRYPOINT|VOLUME|USER|WORKDIR|ARG|ONBUILD|STOPSIGNAL|HEALTHCHECK|SHELL|MAINTAINER)\s+(.*)$").unwrap()
});

static CONTINUATION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\\\s*$").unwrap()
});

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub name: String,
    pub arguments: String,
    pub line_number: usize,
    pub raw_line: String,
}

#[derive(Debug)]
pub struct DockerfileParser {
    pub instructions: Vec<Instruction>,
    pub raw_content: String,
}

impl DockerfileParser {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(&path)?;
        Ok(Self::parse_content(&content))
    }

    pub fn parse_content(content: &str) -> Self {
        let mut instructions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                i += 1;
                continue;
            }

            // Check if this is an instruction
            if let Some(captures) = INSTRUCTION_RE.captures(line) {
                let instruction_name = captures.get(1).unwrap().as_str().to_uppercase();
                let mut arguments = captures.get(2).unwrap().as_str().to_string();
                let start_line = i + 1; // 1-indexed
                let mut raw_line = line.to_string();

                // Handle line continuations
                let mut current_line = line;
                while CONTINUATION_RE.is_match(current_line) && i + 1 < lines.len() {
                    i += 1;
                    current_line = lines[i].trim();
                    raw_line.push_str("\n");
                    raw_line.push_str(current_line);
                    
                    // Remove the backslash and append the continuation
                    arguments = CONTINUATION_RE.replace(&arguments, "").to_string();
                    arguments.push(' ');
                    arguments.push_str(current_line);
                }

                instructions.push(Instruction {
                    name: instruction_name,
                    arguments: arguments.trim().to_string(),
                    line_number: start_line,
                    raw_line,
                });
            }

            i += 1;
        }

        Self {
            instructions,
            raw_content: content.to_string(),
        }
    }

    pub fn get_instructions(&self, name: &str) -> Vec<&Instruction> {
        self.instructions
            .iter()
            .filter(|i| i.name.eq_ignore_ascii_case(name))
            .collect()
    }

    pub fn has_instruction(&self, name: &str) -> bool {
        self.instructions.iter().any(|i| i.name.eq_ignore_ascii_case(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_dockerfile() {
        let content = r#"
FROM ubuntu:20.04
RUN apt-get update
COPY . /app
CMD ["./app"]
"#;
        let parser = DockerfileParser::parse_content(content);
        assert_eq!(parser.instructions.len(), 4);
        assert_eq!(parser.instructions[0].name, "FROM");
        assert_eq!(parser.instructions[0].arguments, "ubuntu:20.04");
    }

    #[test]
    fn test_parse_multiline_run() {
        let content = r#"
FROM alpine
RUN apk add --no-cache \
    curl \
    git
"#;
        let parser = DockerfileParser::parse_content(content);
        assert_eq!(parser.instructions.len(), 2);
        assert!(parser.instructions[1].arguments.contains("curl"));
        assert!(parser.instructions[1].arguments.contains("git"));
    }

    #[test]
    fn test_skip_comments() {
        let content = r#"
# This is a comment
FROM alpine
# Another comment
RUN echo hello
"#;
        let parser = DockerfileParser::parse_content(content);
        assert_eq!(parser.instructions.len(), 2);
    }
}
