# Docker Review

A fast, offline-first CLI tool that reviews Docker configurations like a Senior DevOps Engineer. It detects performance issues, security vulnerabilities, and maintainability problems, providing actionable suggestions and impact estimates.

**Now in Go** - Easy cross-compilation for Linux, macOS (Intel + Apple Silicon), and Windows from a single machine!

## Features

- **Dockerfile Analysis** - Detects 12 types of issues (DF001-DF012)
- **Docker Compose Analysis** - Detects 5 types of issues (DC001-DC005)
- **Security Checks** - Root user, secrets in ENV, curl|bash patterns
- **Performance Checks** - Layer ordering, large images, caching issues
- **Maintainability Checks** - Health checks, restart policies, version pinning
- **Scoring System** - Security, Performance, Maintainability scores (0-10)
- **CI/CD Ready** - JSON output, exit codes, `--fail-on` flag
- **Auto-Fix** - Automatically optimize Dockerfiles with `--fix`
- **Single Binary** - No runtime dependencies, ~3MB

## Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/thirukguru/docker-review/main/install.sh | bash
```

### From Source

```bash
git clone https://github.com/thirukguru/docker-review.git
cd docker-review
make build
sudo cp docker-review /usr/local/bin/
```

### From Releases

Download the binary for your platform from [Releases](https://github.com/thirukguru/docker-review/releases).

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

### Auto-Fix Dockerfiles

```bash
# Generate optimized Dockerfile
docker-review analyze Dockerfile --fix

# Save to custom path
docker-review analyze Dockerfile --fix --fix-output Dockerfile.optimized

# Show diff of changes
docker-review analyze Dockerfile --fix --diff
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

### Dockerfile Rules (DF001-DF012)

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
| DF012 | ML stack optimization | Suggestion |

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
  Performance:    ██░░░░░░░░ 2/10
  Maintainability:███░░░░░░░ 3/10
  Overall:        █░░░░░░░░░ 1/10

📋 Issues Summary
  5 Critical, 5 Warnings, 2 Suggestions

✗ Critical Issues
  [DF001] Using latest tag :1
    Image 'ubuntu' has no tag (implicitly uses 'latest')
    Fix: Pin to a specific version tag (e.g., FROM node:18.17.0-alpine)
```

## Building for Multiple Platforms

One of the key advantages of the Go rewrite - build for all platforms from any machine:

```bash
make build-all
```

This creates binaries in `dist/`:
- `docker-review-linux-amd64`
- `docker-review-linux-arm64`
- `docker-review-darwin-amd64` (Intel Mac)
- `docker-review-darwin-arm64` (Apple Silicon)
- `docker-review-windows-amd64.exe`

## CI/CD Integration

### GitHub Actions

```yaml
- name: Analyze Dockerfile
  run: |
    curl -fsSL https://raw.githubusercontent.com/thirukguru/docker-review/main/install.sh | bash
    docker-review analyze . --ci --fail-on critical
```

### GitLab CI

```yaml
docker-review:
  script:
    - curl -fsSL https://raw.githubusercontent.com/thirukguru/docker-review/main/install.sh | bash
    - docker-review analyze . --ci --fail-on critical
```

## License

MIT
