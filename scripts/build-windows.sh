#!/bin/bash

# Windows 专用构建脚本
# 构建两个版本：带控制台的调试版本和不带控制台的发布版本

set -e

echo "🪟 Building ClipManager for Windows..."

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

# Check prerequisites
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust."
    exit 1
fi

# Check if Windows target is installed
if ! rustup target list --installed | grep -q "x86_64-pc-windows-gnu"; then
    print_warning "Windows target not installed. Installing..."
    rustup target add x86_64-pc-windows-gnu
fi

# Check for mingw-w64
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    print_error "mingw-w64 not found. Please install it:"
    if command -v apt-get >/dev/null 2>&1; then
        echo "  sudo apt-get install gcc-mingw-w64-x86-64"
    elif command -v yum >/dev/null 2>&1; then
        echo "  sudo yum install mingw64-gcc"
    elif command -v dnf >/dev/null 2>&1; then
        echo "  sudo dnf install mingw64-gcc"
    else
        echo "  Please install mingw-w64 for your system"
    fi
    exit 1
fi

# Create output directory
OUTPUT_DIR="windows-builds"
mkdir -p "$OUTPUT_DIR"

print_status "Output directory: $OUTPUT_DIR"

# Build debug version (with console)
print_status "Building debug version (with console)..."
if cargo build --target x86_64-pc-windows-gnu; then
    cp target/x86_64-pc-windows-gnu/debug/clipmanager.exe "$OUTPUT_DIR/clipmanager-debug-console.exe"
    DEBUG_SIZE=$(du -h "$OUTPUT_DIR/clipmanager-debug-console.exe" | cut -f1)
    print_success "Debug version built successfully ($DEBUG_SIZE)"
else
    print_error "Debug build failed"
    exit 1
fi

# Build release version (no console)
print_status "Building release version (no console)..."
if cargo build --release --target x86_64-pc-windows-gnu; then
    cp target/x86_64-pc-windows-gnu/release/clipmanager.exe "$OUTPUT_DIR/clipmanager-release.exe"
    RELEASE_SIZE=$(du -h "$OUTPUT_DIR/clipmanager-release.exe" | cut -f1)
    print_success "Release version built successfully ($RELEASE_SIZE)"
else
    print_error "Release build failed"
    exit 1
fi

# Create a console-enabled release version by temporarily modifying the source
print_status "Building release version with console (for debugging)..."

# Backup original main.rs
cp src/main.rs src/main.rs.backup

# Modify main.rs to always show console
sed 's/#!\[cfg_attr(not(debug_assertions), windows_subsystem = "windows")\]/\/\/ Console enabled for debugging/' src/main.rs.backup > src/main.rs

# Build console-enabled release
if cargo build --release --target x86_64-pc-windows-gnu; then
    cp target/x86_64-pc-windows-gnu/release/clipmanager.exe "$OUTPUT_DIR/clipmanager-release-console.exe"
    CONSOLE_SIZE=$(du -h "$OUTPUT_DIR/clipmanager-release-console.exe" | cut -f1)
    print_success "Console-enabled release version built successfully ($CONSOLE_SIZE)"
else
    print_warning "Console-enabled release build failed"
fi

# Restore original main.rs
mv src/main.rs.backup src/main.rs

# Create README for the builds
cat > "$OUTPUT_DIR/README.txt" << EOF
ClipManager Windows 构建版本
==========================

此目录包含 ClipManager 的不同 Windows 版本：

1. clipmanager-debug-console.exe ($DEBUG_SIZE)
   - 调试版本，显示控制台窗口
   - 包含调试信息，文件较大
   - 适用于开发和调试

2. clipmanager-release.exe ($RELEASE_SIZE)
   - 发布版本，无控制台窗口
   - 优化后的版本，文件较小
   - 适用于最终用户

3. clipmanager-release-console.exe ($CONSOLE_SIZE)
   - 发布版本，但显示控制台窗口
   - 优化后的版本，但可以看到日志输出
   - 适用于故障排除

使用建议：
- 普通用户：使用 clipmanager-release.exe
- 遇到问题时：使用 clipmanager-release-console.exe 查看错误信息
- 开发者：使用 clipmanager-debug-console.exe

注意事项：
- 如果程序无法启动，请尝试控制台版本查看错误信息
- 日志文件位置：%TEMP%\\clipmanager\\clipmanager.log
- 错误报告文件：%TEMP%\\clipmanager_error.txt

系统要求：
- Windows 10 或更高版本
- 支持 X11 的图形环境（如果在 WSL 中运行）
EOF

print_success "🎉 Windows builds completed successfully!"
echo
print_status "Built files:"
ls -lh "$OUTPUT_DIR"

echo
print_status "版本说明："
echo "  📁 clipmanager-debug-console.exe - 调试版本（显示控制台）"
echo "  📁 clipmanager-release.exe - 发布版本（无控制台）"
echo "  📁 clipmanager-release-console.exe - 发布版本（显示控制台）"
echo "  📄 README.txt - 使用说明"

echo
print_success "Windows 构建完成！建议用户先尝试 release 版本，如有问题再使用 console 版本。"
