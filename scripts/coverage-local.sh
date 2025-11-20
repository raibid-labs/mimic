#!/usr/bin/env bash
#
# Generate code coverage report locally
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
cd "$(git rev-parse --show-toplevel)"

print_header "Installing cargo-tarpaulin (if needed)"
if ! command -v cargo-tarpaulin &> /dev/null; then
    cargo install cargo-tarpaulin
fi

print_header "Generating code coverage"
cargo tarpaulin \
    --verbose \
    --all-features \
    --workspace \
    --timeout 300 \
    --out Html \
    --output-dir coverage

print_header "Coverage report generated!"
echo ""
echo "Open coverage/index.html in your browser to view the report."
echo ""

# Try to open in browser
if command -v xdg-open &> /dev/null; then
    print_header "Opening coverage report in browser..."
    xdg-open coverage/index.html 2>/dev/null || true
elif command -v open &> /dev/null; then
    print_header "Opening coverage report in browser..."
    open coverage/index.html 2>/dev/null || true
else
    print_warning "Could not automatically open browser. Please open coverage/index.html manually."
fi
