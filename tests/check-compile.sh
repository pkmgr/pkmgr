#!/usr/bin/env bash
# Quick compilation verification for pkmgr - uses Docker (never host)
# Per project policy: ALWAYS use Docker for building

set -e

echo "🔍 pkmgr Compilation Check"
echo "=========================="
echo ""

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found. Please install Docker."
    echo "Per project policy: NEVER build on host - always use Docker"
    exit 1
fi

echo "✅ Docker available"
echo ""

# Check if in project root
if [ ! -f "Cargo.toml" ] || [ ! -d "docker" ]; then
    echo "❌ Error: Run from project root"
    exit 1
fi

cd docker

echo "🔍 Running cargo check in Docker..."
if docker-compose run --rm pkmgr-dev cargo check 2>&1 | tail -10; then
    echo ""
    echo "✅ Compilation check passed!"
else
    echo ""
    echo "❌ Compilation check failed!"
    exit 1
fi

echo ""
echo "🎉 Ready to build! Next steps:"
echo "  ./scripts/build.sh       # Full release build"
echo "  ./tests/test-incus.sh    # Test in containers"

