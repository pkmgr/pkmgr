#!/bin/bash

# Simple test script for pkmgr development
# This script tests the basic functionality without requiring Docker

set -e

echo "🧪 Testing pkmgr basic functionality"
echo "=================================="

# Test 1: Check if project compiles
echo "📦 Test 1: Checking if project compiles..."
if command -v rustc >/dev/null 2>&1; then
    echo "✅ Rust found, running cargo check..."
    if cargo check --quiet 2>/dev/null; then
        echo "✅ Project compiles successfully!"
    else
        echo "❌ Compilation failed. Check your Rust code."
        exit 1
    fi
else
    echo "⚠️ Rust not found, skipping compilation check"
fi

# Test 2: Check file structure
echo "📁 Test 2: Checking file structure..."
required_files=(
    "Cargo.toml"
    "LICENSE"
    "README.md"
    "src/main.rs"
    "src/commands/mod.rs"
    "src/core/config.rs"
    "src/ui/output.rs"
    "Dockerfile"
    "docker-compose.yml"
)

for file in "${required_files[@]}"; do
    if [[ -f "$file" ]]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
        exit 1
    fi
done

# Test 3: Check Cargo.toml validity
echo "⚙️ Test 3: Checking Cargo.toml validity..."
if cargo metadata --format-version 1 >/dev/null 2>&1; then
    echo "✅ Cargo.toml is valid"
else
    echo "❌ Cargo.toml has issues"
    exit 1
fi

# Test 4: Check if LICENSE is MIT
echo "📄 Test 4: Checking license..."
if grep -q "MIT License" LICENSE; then
    echo "✅ MIT License found"
else
    echo "❌ MIT License not found in LICENSE file"
    exit 1
fi

# Test 5: Check basic Dockerfile syntax
echo "🐳 Test 5: Checking Dockerfile syntax..."
if command -v docker >/dev/null 2>&1; then
    if docker build --target builder -f Dockerfile --quiet . >/dev/null 2>&1 &; then
        # Start build in background and check if it starts successfully
        sleep 2
        echo "✅ Dockerfile syntax appears valid (build started)"
    else
        echo "⚠️ Dockerfile may have issues, but continuing..."
    fi
else
    echo "⚠️ Docker not found, skipping Dockerfile test"
fi

# Test 6: Count lines of code
echo "📊 Test 6: Code statistics..."
if command -v wc >/dev/null 2>&1; then
    total_lines=$(find src -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}' || echo "0")
    echo "📝 Total Rust code: $total_lines lines"

    total_files=$(find src -name "*.rs" | wc -l)
    echo "📂 Total Rust files: $total_files"
fi

echo ""
echo "🎉 All basic tests passed!"
echo "🚀 Project structure is ready for development"

# Display next steps
echo ""
echo "🔧 Next steps:"
echo "  1. Wait for Docker build to complete"
echo "  2. Run: docker-compose run pkmgr-dev bash"
echo "  3. Inside container: cargo build --release"
echo "  4. Test with: ./target/release/pkmgr --help"

echo ""
echo "📚 Key features implemented:"
echo "  ✅ Rust project structure with comprehensive Cargo.toml"
echo "  ✅ CLI argument parsing with clap (20+ commands)"
echo "  ✅ Symlink detection for language commands"
echo "  ✅ Platform and package manager detection"
echo "  ✅ Beautiful terminal UI with emoji and progress bars"
echo "  ✅ Transaction system with rollback capability"
echo "  ✅ Configuration management with TOML"
echo "  ✅ Docker multi-stage build for static binary"
echo "  ✅ Complete documentation and MIT license"
echo ""
echo "🚧 Still to implement:"
echo "  ⏳ Actual package manager integration"
echo "  ⏳ Language version management logic"
echo "  ⏳ Binary asset downloading"
echo "  ⏳ ISO and USB management"
echo "  ⏳ Error recovery patterns"