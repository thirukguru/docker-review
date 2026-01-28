#!/bin/bash
set -e

# Docker Review CLI Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/thirukguru/docker-review/main/install.sh | bash

VERSION="${VERSION:-latest}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
BINARY_NAME="docker-review"
REPO="thirukguru/docker-review"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════╗"
    echo "║       Docker Review CLI Installer         ║"
    echo "║  Analyze Dockerfiles like a Pro DevOps    ║"
    echo "╚═══════════════════════════════════════════╝"
    echo -e "${NC}"
}

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$OS" in
        linux)
            case "$ARCH" in
                x86_64|amd64)
                    PLATFORM="linux-amd64"
                    ;;
                aarch64|arm64)
                    PLATFORM="linux-arm64"
                    ;;
                *)
                    error "Unsupported architecture: $ARCH"
                    ;;
            esac
            ;;
        darwin)
            case "$ARCH" in
                x86_64|amd64)
                    PLATFORM="darwin-amd64"
                    ;;
                aarch64|arm64)
                    PLATFORM="darwin-arm64"
                    ;;
                *)
                    error "Unsupported architecture: $ARCH"
                    ;;
            esac
            ;;
        *)
            error "Unsupported OS: $OS"
            ;;
    esac

    info "Detected platform: $PLATFORM"
}

get_latest_version() {
    if [ "$VERSION" = "latest" ]; then
        info "Fetching latest version..."
        VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
        if [ -z "$VERSION" ]; then
            return 1
        fi
    fi
    info "Installing version: $VERSION"
    return 0
}

download_binary() {
    # Binary naming convention: docker-review-{version}-{platform}
    # Example: docker-review-v0.1.0-linux-amd64
    local DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY_NAME}-${VERSION}-${PLATFORM}"
    local TMP_DIR=$(mktemp -d)
    local TMP_FILE="${TMP_DIR}/${BINARY_NAME}"

    info "Downloading from: $DOWNLOAD_URL" >&2
    
    if curl -fsSL "$DOWNLOAD_URL" -o "$TMP_FILE" 2>/dev/null; then
        chmod +x "$TMP_FILE"
        echo "$TMP_FILE"
        return 0
    fi
    
    # Try with .tar.gz extension
    DOWNLOAD_URL="${DOWNLOAD_URL}.tar.gz"
    info "Trying: $DOWNLOAD_URL" >&2
    
    if curl -fsSL "$DOWNLOAD_URL" -o "${TMP_FILE}.tar.gz" 2>/dev/null; then
        tar -xzf "${TMP_FILE}.tar.gz" -C "$TMP_DIR"
        chmod +x "$TMP_FILE"
        echo "$TMP_FILE"
        return 0
    fi

    return 1
}

install_binary() {
    local BINARY_PATH="$1"

    info "Installing to $INSTALL_DIR..."

    # Check if we need sudo
    if [ -w "$INSTALL_DIR" ]; then
        cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
        chmod +x "$INSTALL_DIR/$BINARY_NAME"
    else
        warn "Need sudo to install to $INSTALL_DIR"
        sudo cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi
}

verify_installation() {
    if command -v "$BINARY_NAME" &> /dev/null; then
        success "Installation complete!"
        echo ""
        "$BINARY_NAME" --version
        echo ""
        echo -e "${GREEN}Run '${BINARY_NAME} --help' to get started${NC}"
    else
        warn "Binary installed but not in PATH. Add $INSTALL_DIR to your PATH."
        echo ""
        echo "Add this to your shell profile:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
}

install_from_source() {
    info "Installing from source..."
    
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo not found. Install from https://rustup.rs/"
    fi

    local TMP_DIR=$(mktemp -d)
    
    info "Cloning repository..."
    git clone --depth 1 "https://github.com/${REPO}.git" "$TMP_DIR/docker-review" || error "Failed to clone repository"
    
    info "Building release binary (this may take a minute)..."
    cd "$TMP_DIR/docker-review"
    cargo build --release || error "Build failed"
    
    install_binary "$TMP_DIR/docker-review/target/release/$BINARY_NAME"
}

main() {
    print_banner
    detect_platform
    
    # Try to download pre-built binary
    if get_latest_version; then
        if BINARY_PATH=$(download_binary); then
            install_binary "$BINARY_PATH"
            verify_installation
            return
        else
            warn "Pre-built binary not available for $PLATFORM"
        fi
    else
        warn "Could not fetch release info from GitHub"
    fi
    
    # Fall back to building from source
    echo ""
    read -p "Would you like to build from source? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        install_from_source
        verify_installation
    else
        error "Installation cancelled"
    fi
}

main "$@"
