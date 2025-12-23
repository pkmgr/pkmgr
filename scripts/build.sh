#!/bin/bash
# Build script for pkmgr using Docker
# Builds single static Rust binary as specified in CLAUDE.md

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "🔨 Building pkmgr using Docker..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cd "$PROJECT_DIR"

# Clean up previous builds
echo "🧹 Cleaning up previous builds..."
docker compose down --remove-orphans 2>/dev/null || true

# Build the development container
echo "📦 Building development container..."
docker compose build pkmgr-dev

# Create and start development container
echo "🚀 Starting development environment..."
docker compose up -d pkmgr-dev

# Build the static binary inside container
echo "🛠️  Building static Rust binary..."
docker compose exec pkmgr-dev cargo build --release --target x86_64-unknown-linux-musl

# Check if binary exists
if docker compose exec pkmgr-dev test -f /app/target/x86_64-unknown-linux-musl/release/pkmgr; then
    echo "✅ Build successful!"
    echo "📍 Binary location: target/x86_64-unknown-linux-musl/release/pkmgr"

    # Show binary info
    echo "📊 Binary information:"
    docker compose exec pkmgr-dev file /app/target/x86_64-unknown-linux-musl/release/pkmgr
    docker compose exec pkmgr-dev ls -lh /app/target/x86_64-unknown-linux-musl/release/pkmgr

    # Test the binary
    echo "🧪 Testing binary..."
    docker compose exec pkmgr-dev /app/target/x86_64-unknown-linux-musl/release/pkmgr --version
else
    echo "❌ Build failed!"
    exit 1
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Build complete! Use scripts/test.sh to run tests."