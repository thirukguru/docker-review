# Docker Review

A fast, offline-first CLI tool that reviews Docker configurations like a Senior DevOps Engineer. It detects performance issues, security vulnerabilities, and maintainability problems, providing actionable suggestions and impact estimates.

[![CI](https://github.com/YOUR_USERNAME/docker-review/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/docker-review/actions/workflows/ci.yml)
[![Release](https://github.com/YOUR_USERNAME/docker-review/actions/workflows/release.yml/badge.svg)](https://github.com/YOUR_USERNAME/docker-review/releases)

## Features

- **Dockerfile Analysis** - Detects 11 types of issues
- **Docker Compose Analysis** - Detects 5 types of issues  
- **Security Checks** - Root user, secrets in ENV, curl|bash patterns
- **Performance Checks** - Layer ordering, large images, caching issues
- **Maintainability Checks** - Health checks, restart policies, version pinning
- **Scoring System** - Security, Performance, Maintainability scores (0-10)
- **CI/CD Ready** - JSON output, exit codes, `--fail-on` flag
- **Single Binary** - No runtime dependencies, ~2.7MB

## Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/docker-review/main/install.sh | bash
```

### From Source

```bash
git clone https://github.com/YOUR_USERNAME/docker-review.git
cd docker-review
cargo build --release
sudo cp target/release/docker-review /usr/local/bin/
```

### From Releases

Download the binary for your platform from [Releases](https://github.com/YOUR_USERNAME/docker-review/releases).

## Usage

### Analyze a Dockerfile

```bash
docker-review analyze Dockerfile
docker-review analyze ./path/to/project
```

### Analyze a docker-compose file

```bash
docker-review analyze docker-compose.yml
```

### JSON Output (for CI)

```bash
docker-review analyze Dockerfile --json
```

### CI Mode with Failure Threshold

```bash
docker-review analyze Dockerfile --ci --fail-on critical
docker-review analyze Dockerfile --ci --fail-on warning
```

### List All Rules

```bash
docker-review rules
```

### Explain a Specific Rule

```bash
docker-review explain DF001
docker-review explain DC002
```

## Rules

### Dockerfile Rules (DF001-DF011)

| ID | Name | Severity |
|----|------|----------|
| DF001 | Using latest tag | Critical |
| DF002 | Running as root | Critical |
| DF003 | No .dockerignore | Warning |
| DF004 | Bad layer ordering | Warning |
| DF005 | No HEALTHCHECK | Warning |
| DF006 | Secrets in ENV | Critical |
| DF007 | No version pinning | Warning |
| DF008 | Missing multi-stage build | Suggestion |
| DF009 | Large base image | Suggestion |
| DF010 | Curl pipe to shell | Critical |
| DF011 | Inefficient layer usage | Warning |

### Docker Compose Rules (DC001-DC005)

| ID | Name | Severity |
|----|------|----------|
| DC001 | No restart policy | Warning |
| DC002 | Privileged container | Critical |
| DC003 | No resource limits | Warning |
| DC004 | Using latest tag | Critical |
| DC005 | Hardcoded secrets | Critical |

## Example Output

```
Docker Review Report
File: Dockerfile

📊 Scores
  Security:       ░░░░░░░░░░ 0/10
  Performance:    █░░░░░░░░░ 1/10
  Maintainability:████████░░ 8/10
  Overall:        ██░░░░░░░░ 2/10

📋 Issues Summary
  5 Critical, 5 Warnings, 1 Suggestions

✗ Critical Issues
  [DF001] Using latest tag :2
    Image 'ubuntu' has no tag (implicitly uses 'latest')
    Fix: Pin to a specific version tag (e.g., FROM node:18.17.0-alpine)
```

## CI/CD Integration

### GitHub Actions

```yaml
- name: Analyze Dockerfile
  run: |
    curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/docker-review/main/install.sh | bash
    docker-review analyze . --ci --fail-on critical
```

### GitLab CI

```yaml
docker-review:
  script:
    - curl -fsSL https://raw.githubusercontent.com/YOUR_USERNAME/docker-review/main/install.sh | bash
    - docker-review analyze . --ci --fail-on critical
```

## License

MIT
