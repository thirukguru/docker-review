# Releasing docker-review

This document describes how to release a new version of docker-review to GitHub.

## Quick Release

### Step 1: Build All Binaries

```bash
cd /home/thiruk/Development/docker-review

# Build for all platforms
make build-all

# Check binaries
ls -lh bin/
```

### Step 2: Create GitHub Release

1. Go to `https://github.com/thirukguru/docker-review/releases`
2. Click **"Create a new release"**

### Step 3: Fill Release Details

- **Tag**: `v0.2.0` (creates new tag)
- **Title**: `v0.2.0`
- **Description**: Write release notes or click "Generate release notes"

### Step 4: Upload Binaries

Upload all files from the `bin/` folder:
- `docker-review-linux-amd64`
- `docker-review-linux-arm64`
- `docker-review-darwin-amd64`
- `docker-review-darwin-arm64`
- `docker-review-windows-amd64.exe`

Click **"Publish release"**

### Step 5: Verify

```bash
curl -fsSL https://raw.githubusercontent.com/thirukguru/docker-review/main/install.sh | bash
```

---

## Binary Naming Convention

The install script expects binaries named:
```
docker-review-{platform}
```

Where `{platform}` is one of:
- `linux-amd64` (Linux x86_64)
- `linux-arm64` (Linux ARM64)
- `darwin-amd64` (macOS Intel)
- `darwin-arm64` (macOS Apple Silicon)
- `windows-amd64.exe` (Windows)

---

## Using GitHub CLI

```bash
VERSION="v0.2.0"

# Build all platforms
make build-all

# Create release with all binaries
gh release create $VERSION \
  --title "$VERSION" \
  --notes "Release notes here" \
  bin/*
```

---

## Checklist Before Release

- [ ] Run `go test ./...` - all tests pass
- [ ] Run `go vet ./...` - no warnings
- [ ] Build with `make build-all`
- [ ] Test binary works: `./bin/docker-review-linux-amd64 --help`
- [ ] Commit all changes
- [ ] Create and upload release on GitHub

---

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

| Change Type | Example | When to use |
|-------------|---------|-------------|
| Patch | `v0.1.0` → `v0.1.1` | Bug fixes |
| Minor | `v0.1.0` → `v0.2.0` | New features |
| Major | `v0.1.0` → `v1.0.0` | Breaking changes |
