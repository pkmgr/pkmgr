#!/usr/bin/env bash
# pkmgr Incus Testing Script
# Tests pkmgr binary across multiple full OS containers
# NEVER test binaries directly on host - always use Incus

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Emoji
SUCCESS="✅"
ERROR="❌"
INFO="ℹ️"
TEST="🧪"
CLEAN="🧹"

log_info() {
    echo -e "${BLUE}${INFO}${NC} $1"
}

log_success() {
    echo -e "${GREEN}${SUCCESS}${NC} $1"
}

log_error() {
    echo -e "${RED}${ERROR}${NC} $1" >&2
}

log_test() {
    echo -e "${YELLOW}${TEST}${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command -v incus >/dev/null 2>&1; then
        log_error "Incus not found. Please install Incus."
        log_info "Visit: https://linuxcontainers.org/incus/"
        exit 1
    fi
    
    if [ ! -f "target/x86_64-unknown-linux-musl/release/pkmgr" ]; then
        log_error "Binary not found. Build first with: ./scripts/build.sh"
        exit 1
    fi
    
    log_success "Prerequisites OK"
}

# Test distributions
DISTROS=(
    "debian/12:test-debian"
    "ubuntu/22.04:test-ubuntu"
    "fedora/39:test-fedora"
    "almalinux/9:test-alma"
)

# Launch test container
launch_container() {
    local image=$1
    local name=$2
    
    log_info "Launching ${name} from ${image}..."
    
    # Delete if exists
    incus delete -f "${name}" 2>/dev/null || true
    
    # Launch new container
    if incus launch "images:${image}" "${name}"; then
        # Wait for container to be ready
        sleep 2
        log_success "Container ${name} launched"
        return 0
    else
        log_error "Failed to launch ${name}"
        return 1
    fi
}

# Copy binary to container
copy_binary() {
    local name=$1
    
    log_info "Copying binary to ${name}..."
    
    if incus file push target/x86_64-unknown-linux-musl/release/pkmgr "${name}/tmp/pkmgr"; then
        incus exec "${name}" -- chmod +x /tmp/pkmgr
        log_success "Binary copied to ${name}"
        return 0
    else
        log_error "Failed to copy binary to ${name}"
        return 1
    fi
}

# Test basic commands
test_basic_commands() {
    local name=$1
    
    log_test "Testing basic commands on ${name}..."
    
    # Test --version
    if incus exec "${name}" -- /tmp/pkmgr --version >/dev/null 2>&1; then
        log_success "${name}: --version works"
    else
        log_error "${name}: --version failed"
        return 1
    fi
    
    # Test --help
    if incus exec "${name}" -- /tmp/pkmgr --help >/dev/null 2>&1; then
        log_success "${name}: --help works"
    else
        log_error "${name}: --help failed"
        return 1
    fi
    
    # Test search (read-only operation)
    if incus exec "${name}" -- /tmp/pkmgr search vim 2>&1 | grep -q "vim\|searching"; then
        log_success "${name}: search command works"
    else
        log_error "${name}: search command failed"
        return 1
    fi
    
    # Test list (read-only operation)
    if incus exec "${name}" -- /tmp/pkmgr list installed >/dev/null 2>&1; then
        log_success "${name}: list command works"
    else
        log_error "${name}: list command failed"
        return 1
    fi
    
    return 0
}

# Clean up container
cleanup_container() {
    local name=$1
    
    log_info "Cleaning up ${name}..."
    if incus delete -f "${name}" 2>/dev/null; then
        log_success "Cleaned up ${name}"
    fi
}

# Main test function
run_tests() {
    local failed=0
    
    echo ""
    echo "${TEST} pkmgr Incus Testing"
    echo "======================="
    echo ""
    
    for distro_info in "${DISTROS[@]}"; do
        IFS=':' read -r image name <<< "$distro_info"
        
        echo ""
        echo "Testing on ${name} (${image})"
        echo "----------------------------"
        
        if ! launch_container "${image}" "${name}"; then
            failed=$((failed + 1))
            continue
        fi
        
        if ! copy_binary "${name}"; then
            cleanup_container "${name}"
            failed=$((failed + 1))
            continue
        fi
        
        if ! test_basic_commands "${name}"; then
            cleanup_container "${name}"
            failed=$((failed + 1))
            continue
        fi
        
        cleanup_container "${name}"
        echo ""
        log_success "All tests passed on ${name}"
    done
    
    echo ""
    echo "===================="
    echo "Test Summary"
    echo "===================="
    
    if [ $failed -eq 0 ]; then
        log_success "All distributions tested successfully!"
        return 0
    else
        log_error "${failed} distribution(s) failed testing"
        return 1
    fi
}

# Cleanup on exit
cleanup_all() {
    echo ""
    log_info "${CLEAN} Cleaning up all test containers..."
    for distro_info in "${DISTROS[@]}"; do
        IFS=':' read -r _ name <<< "$distro_info"
        incus delete -f "${name}" 2>/dev/null || true
    done
}

trap cleanup_all EXIT

# Main
main() {
    check_prerequisites
    run_tests
}

main "$@"
