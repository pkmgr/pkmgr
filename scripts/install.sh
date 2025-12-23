#!/usr/bin/env bash
# pkmgr Universal Installer Script
# Supports: Linux, macOS, BSD
# Copyright (c) 2025 CasjaysDev
# License: MIT

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Emoji (with fallbacks)
if [[ "${TERM}" =~ "xterm" ]] || [[ "${TERM}" =~ "screen" ]]; then
    SUCCESS="✅"
    ERROR="❌"
    INFO="ℹ️"
    WARN="⚠️"
    PACKAGE="📦"
else
    SUCCESS="[OK]"
    ERROR="[ERROR]"
    INFO="[INFO]"
    WARN="[WARN]"
    PACKAGE="[PKG]"
fi

# Configuration
BINARY_NAME="pkmgr"
GITHUB_REPO="pkmgr/pkmgr"
INSTALL_DIR="/usr/local/bin"
USER_INSTALL_DIR="$HOME/.local/bin"
VERSION="latest"

# Functions
log_info() {
    echo -e "${BLUE}${INFO}${NC} $1"
}

log_success() {
    echo -e "${GREEN}${SUCCESS}${NC} $1"
}

log_error() {
    echo -e "${RED}${ERROR}${NC} $1" >&2
}

log_warn() {
    echo -e "${YELLOW}${WARN}${NC} $1"
}

detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "darwin";;
        FreeBSD*)   echo "freebsd";;
        OpenBSD*)   echo "openbsd";;
        NetBSD*)    echo "netbsd";;
        *)          echo "unknown";;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x86_64";;
        aarch64|arm64)  echo "aarch64";;
        armv7l)         echo "armv7";;
        i686|i386)      echo "i686";;
        *)              echo "unknown";;
    esac
}

check_sudo() {
    if [ "$EUID" -ne 0 ]; then
        if command -v sudo >/dev/null 2>&1; then
            if sudo -n true 2>/dev/null; then
                return 0
            fi
        fi
        return 1
    fi
    return 0
}

download_binary() {
    local os=$1
    local arch=$2
    local url="https://github.com/${GITHUB_REPO}/releases/${VERSION}/download/${BINARY_NAME}-${os}-${arch}"
    
    log_info "Downloading ${PACKAGE} ${BINARY_NAME} for ${os}-${arch}..."
    
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "/tmp/${BINARY_NAME}" "${url}"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "/tmp/${BINARY_NAME}" "${url}"
    else
        log_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi
    
    chmod +x "/tmp/${BINARY_NAME}"
}

install_system() {
    log_info "Installing to ${INSTALL_DIR}..."
    
    if check_sudo; then
        sudo mv "/tmp/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
        sudo chmod 755 "${INSTALL_DIR}/${BINARY_NAME}"
        log_success "Installed to ${INSTALL_DIR}/${BINARY_NAME}"
    else
        log_warn "No sudo access. Installing to user directory..."
        install_user
    fi
}

install_user() {
    log_info "Installing to ${USER_INSTALL_DIR}..."
    
    mkdir -p "${USER_INSTALL_DIR}"
    mv "/tmp/${BINARY_NAME}" "${USER_INSTALL_DIR}/${BINARY_NAME}"
    chmod 755 "${USER_INSTALL_DIR}/${BINARY_NAME}"
    
    log_success "Installed to ${USER_INSTALL_DIR}/${BINARY_NAME}"
    
    # Check if directory is in PATH
    if [[ ":$PATH:" != *":${USER_INSTALL_DIR}:"* ]]; then
        log_warn "${USER_INSTALL_DIR} is not in your PATH"
        log_info "Add this to your shell profile:"
        echo ""
        echo "    export PATH=\"\${HOME}/.local/bin:\${PATH}\""
        echo ""
    fi
}

verify_installation() {
    if command -v ${BINARY_NAME} >/dev/null 2>&1; then
        local version=$(${BINARY_NAME} --version 2>/dev/null || echo "unknown")
        log_success "${BINARY_NAME} installed successfully!"
        log_info "Version: ${version}"
        return 0
    else
        log_error "Installation verification failed"
        return 1
    fi
}

main() {
    echo ""
    echo "${PACKAGE} pkmgr Universal Installer"
    echo "================================"
    echo ""
    
    # Detect platform
    local os=$(detect_os)
    local arch=$(detect_arch)
    
    if [ "${os}" = "unknown" ] || [ "${arch}" = "unknown" ]; then
        log_error "Unsupported platform: ${os}-${arch}"
        exit 1
    fi
    
    log_info "Detected platform: ${os}-${arch}"
    
    # Download
    if ! download_binary "${os}" "${arch}"; then
        log_error "Failed to download ${BINARY_NAME}"
        exit 1
    fi
    
    # Install
    if [ "$1" = "--user" ]; then
        install_user
    else
        install_system
    fi
    
    # Verify
    if verify_installation; then
        echo ""
        log_success "Installation complete!"
        echo ""
        log_info "Try it out:"
        echo "    ${BINARY_NAME} --help"
        echo "    ${BINARY_NAME} search vim"
        echo ""
    else
        exit 1
    fi
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --user)
            USER_INSTALL=1
            shift
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --user           Install to user directory (~/.local/bin)"
            echo "  --version VER    Install specific version (default: latest)"
            echo "  --help           Show this help message"
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

main "$@"
