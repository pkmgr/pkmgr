#!/bin/bash
# Cleanup script for pkmgr Docker environment
# Cleans up all temporary files and containers as requested by user

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "🧹 Cleaning up pkmgr Docker environment..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Stop and remove all containers
echo "🛑 Stopping all pkmgr containers..."
docker compose down --remove-orphans

# Remove all pkmgr images
echo "🗑️  Removing pkmgr images..."
docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.ID}}" | grep "pkmgr" | awk '{print $3}' | xargs -r docker rmi -f

# Clean up Docker system (optional, with confirmation)
if [ "${1:-}" = "--deep" ]; then
    echo "🔥 Performing deep clean..."
    echo "⚠️  This will remove ALL unused Docker resources!"
    read -p "Are you sure? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        docker system prune -af --volumes
        echo "✅ Deep clean completed"
    else
        echo "❌ Deep clean cancelled"
    fi
else
    # Clean only pkmgr-related volumes and networks
    echo "🗄️  Removing pkmgr volumes..."
    docker volume ls -q | grep -E "(pkmgr|cargo-cache|target-cache)" | xargs -r docker volume rm

    echo "🌐 Removing pkmgr networks..."
    docker network ls -q --filter name=pkmgr | xargs -r docker network rm
fi

# Clean up build artifacts
echo "🧽 Cleaning Rust build artifacts..."
if [ -d "target" ]; then
    rm -rf target/
    echo "  ✅ Removed target/ directory"
fi

if [ -f "Cargo.lock" ]; then
    rm -f Cargo.lock
    echo "  ✅ Removed Cargo.lock"
fi

# Clean up temporary files
echo "🗂️  Cleaning temporary files..."
find . -name "*.tmp" -delete 2>/dev/null || true
find . -name "*.temp" -delete 2>/dev/null || true
find . -name ".DS_Store" -delete 2>/dev/null || true
find . -name "Thumbs.db" -delete 2>/dev/null || true

# Clean up log files
if [ -d "logs" ]; then
    rm -rf logs/
    echo "  ✅ Removed logs/ directory"
fi

# Clean up test artifacts
echo "🧪 Cleaning test artifacts..."
find . -name "test-results*" -delete 2>/dev/null || true
find . -name "*.test" -delete 2>/dev/null || true

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Cleanup completed!"
echo ""
echo "📊 Docker system status:"
docker system df
echo ""
echo "💡 Use scripts/clean.sh --deep for complete Docker system cleanup"
echo "🚀 Ready for fresh build with scripts/build.sh"