#!/bin/bash
# Debug script for pkmgr development using Docker
# Provides interactive debugging environment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "🔍 Starting pkmgr debug environment..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Start development environment
echo "🚀 Starting development container..."
docker compose up -d pkmgr-dev

echo "🛠️  Development environment ready!"
echo ""
echo "Available commands:"
echo "  📋 Check compilation:    docker compose exec pkmgr-dev cargo check"
echo "  🔨 Build debug binary:   docker compose exec pkmgr-dev cargo build"
echo "  🔨 Build release binary: docker compose exec pkmgr-dev cargo build --release --target x86_64-unknown-linux-musl"
echo "  🧪 Run tests:           docker compose exec pkmgr-dev cargo test"
echo "  🔍 Format code:         docker compose exec pkmgr-dev cargo fmt"
echo "  🕵️  Lint code:           docker compose exec pkmgr-dev cargo clippy"
echo "  🐚 Interactive shell:   docker compose exec pkmgr-dev bash"
echo ""
echo "Environment variables:"
echo "  RUST_LOG=debug"
echo "  RUST_BACKTRACE=1"
echo ""
echo "📁 Working directory: /app (mounted from current directory)"
echo "📦 Cargo cache: Persistent volume"
echo "🎯 Target cache: Persistent volume"

# Check if user wants to enter interactive mode
if [ "${1:-}" = "-i" ] || [ "${1:-}" = "--interactive" ]; then
    echo ""
    echo "🐚 Entering interactive debug shell..."
    docker compose exec pkmgr-dev bash
else
    echo ""
    echo "💡 Run with -i or --interactive to enter debug shell"
    echo "💡 Use scripts/build.sh to build the project"
    echo "💡 Use scripts/test.sh to run tests"
    echo ""
    echo "🔍 To stop debug environment: docker compose down"
fi