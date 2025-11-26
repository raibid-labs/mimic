#!/usr/bin/env bash
#
# Install git pre-commit hooks for mimic
#

set -euo pipefail

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_header() {
    echo -e "${GREEN}==>${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}WARNING:${NC} $1"
}

# Change to repo root
REPO_ROOT=$(git rev-parse --show-toplevel)
cd "$REPO_ROOT"

HOOKS_DIR="$REPO_ROOT/.git/hooks"
PRE_COMMIT_HOOK="$HOOKS_DIR/pre-commit"

print_header "Installing pre-commit hooks for mimic"

# Ensure hooks directory exists
mkdir -p "$HOOKS_DIR"

# Create pre-commit hook
cat > "$PRE_COMMIT_HOOK" << 'EOF'
#!/usr/bin/env bash
#
# Pre-commit hook for mimic
# Runs fast checks before allowing commit
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_error() {
    echo -e "${RED}ERROR:${NC} $1"
}

print_success() {
    echo -e "${GREEN}SUCCESS:${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}WARNING:${NC} $1"
}

echo "Running pre-commit checks..."
echo ""

# 1. Check formatting
echo "Checking code formatting..."
if ! cargo fmt --all -- --check; then
    print_error "Code is not formatted correctly"
    echo ""
    echo "Run: cargo fmt --all"
    echo "Then stage the changes and try again."
    exit 1
fi
print_success "Code formatting OK"

# 2. Run clippy on changed files
echo ""
echo "Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    print_error "Clippy found issues"
    echo ""
    echo "Fix the clippy warnings above and try again."
    exit 1
fi
print_success "Clippy OK"

# 3. Run fast tests (library only)
echo ""
echo "Running library tests..."
if ! cargo test --lib --quiet; then
    print_error "Library tests failed"
    echo ""
    echo "Fix the failing tests and try again."
    exit 1
fi
print_success "Library tests OK"

# 4. Check for common issues
echo ""
echo "Checking for common issues..."

# Check for debug prints in staged files
if git diff --cached --name-only | grep '\.rs$' | xargs grep -n "dbg!\|println!" 2>/dev/null | grep -v "test\|example"; then
    print_warning "Found debug prints (dbg! or println!) in staged files"
    echo "Consider removing them or using proper logging."
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo ""
print_success "All pre-commit checks passed!"
echo ""
echo "You can skip these checks with: git commit --no-verify"
EOF

# Make hook executable
chmod +x "$PRE_COMMIT_HOOK"

print_header "Pre-commit hook installed successfully!"
echo ""
echo "The hook will run before each commit and check:"
echo "  - Code formatting (cargo fmt)"
echo "  - Clippy warnings"
echo "  - Library tests"
echo "  - Common issues (debug prints, etc.)"
echo ""
echo "To skip the hook for a single commit, use:"
echo "  git commit --no-verify"
echo ""
print_warning "Note: The hook only runs fast checks. Full CI runs all tests."
