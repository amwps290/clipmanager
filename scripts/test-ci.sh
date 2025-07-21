#!/bin/bash

# Local CI Test Script
# This script simulates the CI pipeline locally

set -e

echo "🧪 Running local CI tests..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to run a test step
run_step() {
    local step_name=$1
    local command=$2
    
    print_status "Running: $step_name"
    
    if eval "$command"; then
        print_success "$step_name passed"
        echo
    else
        print_error "$step_name failed"
        exit 1
    fi
}

# Check prerequisites
print_status "Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust."
    exit 1
fi

if ! command -v rustfmt &> /dev/null; then
    print_warning "rustfmt not found. Installing..."
    rustup component add rustfmt
fi

if ! command -v clippy &> /dev/null; then
    print_warning "clippy not found. Installing..."
    rustup component add clippy
fi

print_success "Prerequisites check passed"
echo

# Run CI steps
print_status "Starting CI pipeline simulation..."
echo

# Step 1: Format check
run_step "Format check" "cargo fmt --all -- --check"

# Step 2: Clippy check
run_step "Clippy check" "cargo clippy --all-targets --all-features -- -D warnings"

# Step 3: Run tests
run_step "Unit tests" "cargo test --verbose"

# Step 4: Build check
run_step "Build check" "cargo build --verbose"

# Step 5: Benchmark check (dry run)
run_step "Benchmark check" "cargo bench --no-run"

# Step 6: Documentation check
run_step "Documentation check" "cargo doc --no-deps"

# Optional: Cross-compilation test
if [[ "$1" == "--cross-compile" ]]; then
    print_status "Testing cross-compilation..."
    
    # Install targets if not present
    rustup target add x86_64-unknown-linux-gnu 2>/dev/null || true
    rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
    
    # Test Linux build
    run_step "Linux cross-compile" "cargo build --target x86_64-unknown-linux-gnu"
    
    # Test Windows build (if mingw is available)
    if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
        run_step "Windows cross-compile" "cargo build --target x86_64-pc-windows-gnu"
    else
        print_warning "mingw-w64 not found, skipping Windows cross-compilation"
        print_status "Install with: sudo apt-get install gcc-mingw-w64-x86-64"
    fi
fi

print_success "🎉 All CI tests passed!"

# Summary
echo
print_status "CI Test Summary:"
echo "  ✅ Format check"
echo "  ✅ Clippy check"
echo "  ✅ Unit tests"
echo "  ✅ Build check"
echo "  ✅ Benchmark check"
echo "  ✅ Documentation check"

if [[ "$1" == "--cross-compile" ]]; then
    echo "  ✅ Cross-compilation tests"
fi

echo
print_status "Your code is ready for CI! 🚀"
