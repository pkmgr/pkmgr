#!/bin/bash
# Comprehensive functionality test for pkmgr universal package manager
# Tests all major features according to CLAUDE.md specification

set -e

echo "🧪 Starting pkmgr functionality tests..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_TOTAL=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    local description="$3"

    ((TESTS_TOTAL++))
    echo -e "\n${BLUE}Test $TESTS_TOTAL: $test_name${NC}"
    echo "Description: $description"
    echo "Command: $test_command"
    echo -n "Result: "

    if eval "$test_command" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS${NC}"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}"
        echo "Error output:"
        eval "$test_command" 2>&1 | head -5 || true
    fi
}

# Test basic functionality
echo -e "\n${YELLOW}=== BASIC FUNCTIONALITY TESTS ===${NC}"

run_test "Version Check" "pkmgr --version" "Check if pkmgr binary runs and shows version"
run_test "Help Command" "pkmgr --help" "Verify help system works"
run_test "Binary Detection" "which pkmgr" "Confirm pkmgr is in PATH"

# Test shell integration
echo -e "\n${YELLOW}=== SHELL INTEGRATION TESTS ===${NC}"

run_test "Shell Environment" "pkmgr shell env" "Check shell integration status"
run_test "Shell Load Script" "pkmgr shell load bash" "Generate bash integration script"
run_test "PATH Management" "pkmgr shell add" "Generate PATH modification script"

# Test cache management
echo -e "\n${YELLOW}=== CACHE MANAGEMENT TESTS ===${NC}"

run_test "Cache Info" "pkmgr cache info" "Display cache information"
run_test "Cache List" "pkmgr cache list" "List cache contents"
run_test "Cache Clean Dry Run" "pkmgr cache clean --dry-run" "Test cache cleanup dry run"

# Test doctor/health checks
echo -e "\n${YELLOW}=== SYSTEM HEALTH TESTS ===${NC}"

run_test "Basic Doctor Check" "pkmgr doctor" "Run basic system health check"
run_test "Package Health" "pkmgr doctor --packages" "Check package management health"
run_test "Security Check" "pkmgr doctor --security" "Run security status check"

# Test package operations (dry run to avoid actual installs)
echo -e "\n${YELLOW}=== PACKAGE OPERATIONS TESTS ===${NC}"

run_test "Package Search" "pkmgr search curl" "Search for packages"
run_test "Package Info" "pkmgr info curl" "Get package information"
run_test "Package List" "pkmgr list installed" "List installed packages"
run_test "Dry Run Install" "pkmgr install curl --dry-run" "Test package installation (dry run)"

# Test language version management
echo -e "\n${YELLOW}=== LANGUAGE VERSION TESTS ===${NC}"

run_test "Python Version List" "pkmgr python list" "List Python versions"
run_test "Node Version List" "pkmgr node list" "List Node.js versions"
run_test "Language Current" "pkmgr python current" "Show current Python version"

# Test binary management
echo -e "\n${YELLOW}=== BINARY MANAGEMENT TESTS ===${NC}"

run_test "Binary Search" "pkmgr binary search lazydocker" "Search for binary releases"
run_test "Binary List" "pkmgr binary list" "List installed binaries"

# Test repository management
echo -e "\n${YELLOW}=== REPOSITORY MANAGEMENT TESTS ===${NC}"

run_test "Repository List" "pkmgr repos list" "List configured repositories"
run_test "Repository Info" "pkmgr repos info" "Show repository information"

# Test profile management
echo -e "\n${YELLOW}=== PROFILE MANAGEMENT TESTS ===${NC}"

run_test "Profile List" "pkmgr profile list" "List available profiles"
run_test "Profile Create" "pkmgr profile create test-profile --dry-run" "Create test profile (dry run)"

# Test configuration management
echo -e "\n${YELLOW}=== CONFIGURATION TESTS ===${NC}"

run_test "Config List" "pkmgr config list" "List configuration settings"
run_test "Config Get" "pkmgr config get verbosity" "Get configuration value"

# Test symlink functionality (language command wrapper)
echo -e "\n${YELLOW}=== SYMLINK/WRAPPER TESTS ===${NC}"

if [ -L ~/.local/bin/python ]; then
    run_test "Python Symlink" "python --version 2>&1 | grep -i python" "Test Python symlink wrapper"
fi

if [ -L ~/.local/bin/node ]; then
    run_test "Node Symlink" "node --version 2>&1 | grep -i node || node --version 2>&1 | grep -E 'v[0-9]+'" "Test Node.js symlink wrapper"
fi

# Test error recovery system
echo -e "\n${YELLOW}=== ERROR RECOVERY TESTS ===${NC}"

run_test "Fix Command" "pkmgr fix --dry-run" "Test error recovery system"

# Test universal package name normalization
echo -e "\n${YELLOW}=== PACKAGE NORMALIZATION TESTS ===${NC}"

run_test "Python Package" "pkmgr search python --dry-run" "Test Python package normalization"
run_test "Docker Package" "pkmgr search docker --dry-run" "Test Docker package normalization"

# Summary
echo -e "\n${YELLOW}=== TEST SUMMARY ===${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $TESTS_PASSED -eq $TESTS_TOTAL ]; then
    echo -e "🎉 ${GREEN}ALL TESTS PASSED!${NC} ($TESTS_PASSED/$TESTS_TOTAL)"
    echo -e "✅ pkmgr is working correctly according to CLAUDE.md specifications"
    exit 0
else
    echo -e "⚠️  ${YELLOW}$TESTS_PASSED/$TESTS_TOTAL tests passed${NC}"
    echo -e "❌ Some functionality may need attention"
    exit 1
fi