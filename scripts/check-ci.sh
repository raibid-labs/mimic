#!/usr/bin/env bash
#
# Run all CI checks locally before pushing
# This mirrors what GitHub Actions will run
#

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track overall success
FAILED=0

print_header() {
    echo -e "${GREEN}==>${NC} $1"
}

print_error() {
    echo -e "${RED}ERROR:${NC} $1"
}

print_success() {
    echo -e "${GREEN}SUCCESS:${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}WARNING:${NC} $1"
}

run_check() {
    local name=$1
    shift
    print_header "Running: $name"
    if "$@"; then
        print_success "$name passed"
        return 0
    else
        print_error "$name failed"
        FAILED=1
        return 1
    fi
}

# Change to repo root
cd "$(git rev-parse --show-toplevel)"

echo "========================================"
echo "  Running CI Checks Locally"
echo "========================================"
echo ""

# 1. Format check
run_check "Format check" cargo fmt --all -- --check

# 2. Clippy
run_check "Clippy" cargo clippy --all-targets --all-features -- -D warnings

# 3. Build
run_check "Build (all features)" cargo build --all-features --verbose

# 4. Test - Library
run_check "Test (library)" cargo test --lib --all-features --verbose

# 5. Test - Integration
run_check "Test (integration)" cargo test --test '*' --all-features --verbose || true

# 6. Test - Doc tests
run_check "Test (doc)" cargo test --doc --all-features --verbose || true

# 7. Feature flag tests
print_header "Feature flag tests"
FEATURES=(
    "--no-default-features"
    "--all-features"
)

for feature in "${FEATURES[@]}"; do
    run_check "Test $feature" cargo test $feature --verbose || true
done

# 8. Build documentation
run_check "Documentation" cargo doc --all-features --no-deps

# 9. Check for common issues
print_header "Additional checks"

# Check for TODO/FIXME
if git grep -n "TODO\|FIXME" -- '*.rs' 2>/dev/null; then
    print_warning "Found TODO/FIXME comments (not a failure, just FYI)"
fi

# Check for println! in non-example code
if git grep -n "println!" -- 'src/**/*.rs' 2>/dev/null | grep -v "test" | grep -v "example"; then
    print_warning "Found println! in source code (consider using proper logging)"
fi

# Summary
echo ""
echo "========================================"
if [ $FAILED -eq 0 ]; then
    print_success "All CI checks passed!"
    echo ""
    echo "You can push your changes with confidence."
    exit 0
else
    print_error "Some CI checks failed!"
    echo ""
    echo "Please fix the errors above before pushing."
    exit 1
fi
