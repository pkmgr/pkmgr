#!/usr/bin/env bash
# Compilation check for pkmgr - uses Docker (never host)
# Per project policy: ALWAYS use Docker for building

set -e

echo "🔍 pkmgr Compilation Check (Docker)"
echo "===================================="
echo ""

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found. Please install Docker."
    exit 1
fi

echo "✅ Docker found: $(docker --version)"
echo ""

# Check if docker-compose.yml exists
if [ ! -f "docker/docker-compose.yml" ]; then
    echo "❌ docker/docker-compose.yml not found"
    echo "Please run from project root"
    exit 1
fi

cd docker

echo "🔍 Building Docker development image..."
if docker-compose build pkmgr-dev 2>&1 | tail -5; then
    echo "✅ Docker image built successfully"
else
    echo "❌ Docker build failed"
    exit 1
fi

echo ""
echo "🔍 Checking code compilation (cargo check)..."
if docker-compose run --rm pkmgr-dev cargo check 2>&1 | tee /tmp/pkmgr-check.log | tail -20; then
    echo ""
    echo "✅ Code check passed!"
else
    echo ""
    echo "❌ Code check failed. See errors above."
    echo "📝 Full log saved to /tmp/pkmgr-check.log"
    exit 1
fi

echo ""
echo "🎉 All checks passed! Code is ready to build."
echo ""
echo "Next steps:"
echo "  ./scripts/build.sh       # Full release build"
echo "  ./tests/test-incus.sh    # Test on real distributions"

