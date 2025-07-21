#!/bin/bash

# Test Cross-Compilation Script
# This script tests cross-compilation for Windows on Linux

set -e

echo "🧪 Testing cross-compilation setup..."

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

# Check if we're on Linux
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    print_error "This script is designed to run on Linux for cross-compilation testing"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust."
    exit 1
fi

print_status "Checking Rust installation..."
rustc --version
cargo --version

# Check if Windows target is installed
print_status "Checking Windows target installation..."
if rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
    print_success "Windows target (x86_64-pc-windows-gnu) is installed"
else
    print_warning "Windows target not installed. Installing..."
    rustup target add x86_64-pc-windows-gnu
fi

# Check if mingw-w64 is installed
print_status "Checking mingw-w64 installation..."
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    print_success "mingw-w64 is installed"
    x86_64-w64-mingw32-gcc --version | head -1
else
    print_error "mingw-w64 not found. Please install it:"
    echo "  sudo apt-get update"
    echo "  sudo apt-get install gcc-mingw-w64-x86-64"
    exit 1
fi

# Test Linux build
print_status "Testing Linux build..."
if cargo build --target x86_64-unknown-linux-gnu; then
    print_success "Linux build successful"
    ls -la target/x86_64-unknown-linux-gnu/debug/clipmanager
else
    print_error "Linux build failed"
    exit 1
fi

# Test Windows build
print_status "Testing Windows cross-compilation..."
if cargo build --target x86_64-pc-windows-gnu; then
    print_success "Windows cross-compilation successful"
    ls -la target/x86_64-pc-windows-gnu/debug/clipmanager.exe
    
    # Check if the binary is actually a Windows executable
    if file target/x86_64-pc-windows-gnu/debug/clipmanager.exe | grep -q "PE32+"; then
        print_success "Generated binary is a valid Windows PE executable"
    else
        print_warning "Generated binary might not be a valid Windows executable"
    fi
else
    print_error "Windows cross-compilation failed"
    exit 1
fi

# Test release builds
print_status "Testing release builds..."

print_status "Building Linux release..."
if cargo build --release --target x86_64-unknown-linux-gnu; then
    print_success "Linux release build successful"
    LINUX_SIZE=$(du -h target/x86_64-unknown-linux-gnu/release/clipmanager | cut -f1)
    print_status "Linux binary size: $LINUX_SIZE"
else
    print_error "Linux release build failed"
    exit 1
fi

print_status "Building Windows release..."
if cargo build --release --target x86_64-pc-windows-gnu; then
    print_success "Windows release build successful"
    WINDOWS_SIZE=$(du -h target/x86_64-pc-windows-gnu/release/clipmanager.exe | cut -f1)
    print_status "Windows binary size: $WINDOWS_SIZE"
else
    print_error "Windows release build failed"
    exit 1
fi

# Summary
echo
print_success "🎉 Cross-compilation test completed successfully!"
echo
print_status "Build Summary:"
echo "  ✅ Linux debug build"
echo "  ✅ Linux release build ($LINUX_SIZE)"
echo "  ✅ Windows debug build"
echo "  ✅ Windows release build ($WINDOWS_SIZE)"
echo
print_status "Generated files:"
echo "  📁 target/x86_64-unknown-linux-gnu/debug/clipmanager"
echo "  📁 target/x86_64-unknown-linux-gnu/release/clipmanager"
echo "  📁 target/x86_64-pc-windows-gnu/debug/clipmanager.exe"
echo "  📁 target/x86_64-pc-windows-gnu/release/clipmanager.exe"
echo
print_status "Your cross-compilation setup is working correctly! 🚀"
