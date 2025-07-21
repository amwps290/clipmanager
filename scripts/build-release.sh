#!/bin/bash

# ClipManager Release Build Script
# This script builds release binaries for multiple platforms

set -e

echo "🚀 Building ClipManager release binaries..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust."
    exit 1
fi

# Create output directory
OUTPUT_DIR="release-builds"
mkdir -p "$OUTPUT_DIR"

print_status "Output directory: $OUTPUT_DIR"

# Platforms to build for
PLATFORMS=(
    "x86_64-unknown-linux-gnu:clipmanager:clipmanager-linux"
    "x86_64-pc-windows-gnu:clipmanager.exe:clipmanager-windows.exe"
)

# Function to build for a specific platform
build_platform() {
    local target=$1
    local binary_name=$2
    local output_name=$3
    
    print_status "Building for $target..."
    
    # Check if target is installed
    if ! rustup target list --installed | grep -q "$target"; then
        print_status "Installing target $target..."
        rustup target add "$target"
    fi
    
    # Build the binary
    if cargo build --release --target "$target"; then
        # Copy binary to output directory
        local source_path="target/$target/release/$binary_name"
        local dest_path="$OUTPUT_DIR/$output_name"
        
        if [ -f "$source_path" ]; then
            cp "$source_path" "$dest_path"
            
            # Strip binary for Linux
            if [[ "$target" == *"linux"* ]]; then
                strip "$dest_path" 2>/dev/null || print_warning "Could not strip binary"
            fi
            
            # Get file size
            local size=$(du -h "$dest_path" | cut -f1)
            print_success "Built $output_name ($size)"
        else
            print_error "Binary not found at $source_path"
            return 1
        fi
    else
        print_error "Build failed for $target"
        return 1
    fi
}

# Install cross-compilation dependencies on Linux
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    print_status "Checking cross-compilation dependencies..."
    
    # Check for mingw-w64 for Windows cross-compilation
    if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
        print_warning "mingw-w64 not found. Windows cross-compilation may fail."
        print_status "Install with: sudo apt-get install gcc-mingw-w64-x86-64"
    fi
fi

# Build for each platform
for platform_info in "${PLATFORMS[@]}"; do
    IFS=':' read -r target binary_name output_name <<< "$platform_info"
    
    if build_platform "$target" "$binary_name" "$output_name"; then
        echo
    else
        print_error "Failed to build for $target"
        exit 1
    fi
done

print_success "All builds completed successfully!"
print_status "Binaries are available in: $OUTPUT_DIR"

# List built files
echo
print_status "Built files:"
ls -lh "$OUTPUT_DIR"

echo
print_success "🎉 Release build complete!"
