#!/bin/bash

# AISP Z3 Environment Setup for macOS Homebrew
# This script sets up the environment variables needed for Z3 integration

echo "🔧 Setting up Z3 environment for AISP..."

# Export environment variables
export LIBCLANG_PATH="/opt/homebrew/opt/llvm/lib"
export Z3_SYS_Z3_HEADER="/opt/homebrew/include/z3.h"
export C_INCLUDE_PATH="/opt/homebrew/include"
export LIBRARY_PATH="/opt/homebrew/lib"
export PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:$PKG_CONFIG_PATH"

echo "✅ Environment variables set:"
echo "   LIBCLANG_PATH=$LIBCLANG_PATH"
echo "   Z3_SYS_Z3_HEADER=$Z3_SYS_Z3_HEADER"
echo "   C_INCLUDE_PATH=$C_INCLUDE_PATH"
echo "   LIBRARY_PATH=$LIBRARY_PATH"
echo "   PKG_CONFIG_PATH=$PKG_CONFIG_PATH"

# Verify Z3 installation
echo ""
echo "🧪 Verifying Z3 installation..."
if command -v z3 &> /dev/null; then
    echo "   ✅ Z3 binary found: $(z3 --version)"
else
    echo "   ❌ Z3 binary not found"
    exit 1
fi

if pkg-config --exists z3; then
    echo "   ✅ Z3 pkg-config found"
else
    echo "   ❌ Z3 pkg-config not found"
    exit 1
fi

if [ -f "/opt/homebrew/include/z3.h" ]; then
    echo "   ✅ Z3 headers found"
else
    echo "   ❌ Z3 headers not found"
    exit 1
fi

if [ -f "/opt/homebrew/lib/libz3.dylib" ]; then
    echo "   ✅ Z3 library found"
else
    echo "   ❌ Z3 library not found"
    exit 1
fi

if [ -f "/opt/homebrew/opt/llvm/lib/libclang.dylib" ]; then
    echo "   ✅ libclang found"
else
    echo "   ❌ libclang not found"
    exit 1
fi

echo ""
echo "🎉 Z3 environment setup complete!"
echo ""
echo "To make this permanent, add these lines to your ~/.zshrc (or ~/.bashrc):"
echo ""
echo "export LIBCLANG_PATH=\"/opt/homebrew/opt/llvm/lib\""
echo "export Z3_SYS_Z3_HEADER=\"/opt/homebrew/include/z3.h\""
echo "export C_INCLUDE_PATH=\"/opt/homebrew/include\""
echo "export LIBRARY_PATH=\"/opt/homebrew/lib\""
echo "export PKG_CONFIG_PATH=\"/opt/homebrew/lib/pkgconfig:\$PKG_CONFIG_PATH\""
echo ""
echo "Now you can run:"
echo "cd aisp-formal-verification && cargo build --features z3-verification"