# Releasing docker-review

This document describes how to release a new version of docker-review to GitHub.

## Quick Release (Manual)

### Step 1: Build Release Binary

```bash
cd /home/thiruk/Development/docker-review

# Build optimized release binary
cargo build --release

# Check binary size
ls -lh target/release/docker-review
```

### Step 2: Create GitHub Release

1. Go to your GitHub repository
2. Click **"Releases"** (right sidebar) or go to `https://github.com/thirukguru/docker-review/releases`
3. Click **"Create a new release"** or **"Draft a new release"**

### Step 3: Fill Release Details

- **Choose a tag**: Type `v0.1.0` and click "Create new tag on publish"
- **Release title**: `v0.1.0`
- **Description**: Write release notes or click "Generate release notes"

### Step 4: Upload Binary

1. Rename your binary with platform suffix:
   ```bash
   # For Linux x86_64
   cp target/release/docker-review docker-review-v0.1.0-linux-amd64
   
   # For Linux ARM64
   cp target/release/docker-review docker-review-v0.1.0-linux-arm64
   
   # For macOS Intel
   cp target/release/docker-review docker-review-v0.1.0-darwin-amd64
   
   # For macOS Apple Silicon
   cp target/release/docker-review docker-review-v0.1.0-darwin-arm64
   ```

2. In GitHub Release page, drag and drop the binary file to **"Attach binaries"** section

3. Click **"Publish release"**

### Step 5: Verify

Test the install script:
```bash
curl -fsSL https://raw.githubusercontent.com/thirukguru/docker-review/main/install.sh | bash
```

---

## Binary Naming Convention

The install script expects binaries named:
```
docker-review-{version}-{platform}
```

Where:
- `{version}` = `v0.1.0`, `v0.2.0`, etc.
- `{platform}` = one of:
  - `linux-amd64` (Linux x86_64)
  - `linux-arm64` (Linux ARM64)
  - `darwin-amd64` (macOS Intel)
  - `darwin-arm64` (macOS Apple Silicon)

---

## Complete Manual Release Script

Run this to prepare binaries for release:

```bash
#!/bin/bash
VERSION="v0.1.0"  # Change this

cd /home/thiruk/Development/docker-review

# Build release
cargo build --release

# Create release directory
mkdir -p releases

# Copy binary with platform name
cp target/release/docker-review "releases/docker-review-${VERSION}-linux-amd64"

# Show what to upload
echo "Upload this file to GitHub Release:"
ls -lh releases/

echo ""
echo "Go to: https://github.com/thirukguru/docker-review/releases/new"
echo "1. Tag: ${VERSION}"
echo "2. Title: ${VERSION}"
echo "3. Upload: releases/docker-review-${VERSION}-linux-amd64"
echo "4. Click 'Publish release'"
```

---

## First-Time Setup

### 1. Create GitHub Repository

Go to https://github.com/new and create `docker-review` repository.

### 2. Update thirukguru

Replace `thirukguru` with your actual GitHub username:

```bash
# Replace in all files
sed -i 's/thirukguru/your-github-username/g' install.sh README.md RELEASING.md
```

### 3. Push Code to GitHub

```bash
cd /home/thiruk/Development/docker-review

# Initialize git
git init
git add .
git commit -m "Initial commit: docker-review CLI v0.1.0"

# Add remote and push
git remote add origin https://github.com/thirukguru/docker-review.git
git branch -M main
git push -u origin main
```

### 4. Create First Release

Follow the "Quick Release (Manual)" steps above.

---

## Using GitHub CLI (Optional)

If you have `gh` CLI installed:

```bash
VERSION="v0.1.0"

# Build
cargo build --release
cp target/release/docker-review "docker-review-${VERSION}-linux-amd64"

# Create release with binary
gh release create $VERSION \
  --title "$VERSION" \
  --notes "Initial release" \
  "docker-review-${VERSION}-linux-amd64"
```

---

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

| Change Type | Example | When to use |
|-------------|---------|-------------|
| Patch | `v0.1.0` → `v0.1.1` | Bug fixes |
| Minor | `v0.1.0` → `v0.2.0` | New features |
| Major | `v0.1.0` → `v1.0.0` | Breaking changes |

---

## Checklist Before Release

- [ ] Update version in `Cargo.toml`
- [ ] Run `cargo test` - all tests pass
- [ ] Run `cargo clippy` - no warnings
- [ ] Build with `cargo build --release`
- [ ] Test binary works: `./target/release/docker-review --version`
- [ ] Commit all changes
- [ ] Create and upload release on GitHub
