#!/usr/bin/env bash
# pkmgr Linux Installer
# Specialized installer for Linux distributions
# Copyright (c) 2025 CasjaysDev
# License: MIT

set -e

# Source the universal installer
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "${SCRIPT_DIR}/install.sh" ]; then
    exec "${SCRIPT_DIR}/install.sh" "$@"
else
    echo "Error: install.sh not found"
    exit 1
fi
