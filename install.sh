#!/bin/bash
# One-line installer for slvsx
# Usage: curl -fsSL https://raw.githubusercontent.com/snoble/slvsx-cli/main/install.sh | bash

set -e

echo "🚀 Installing slvsx geometry solver..."

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map to our binary names
if [ "$OS" = "linux" ]; then
    BINARY_NAME="slvsx-linux"
elif [ "$OS" = "darwin" ]; then
    # Detect macOS architecture
    if [ "$ARCH" = "x86_64" ]; then
        BINARY_NAME="slvsx-macos-x86_64"
    elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        BINARY_NAME="slvsx-macos-arm64"
    else
        echo "❌ Unsupported macOS architecture: $ARCH"
        exit 1
    fi
else
    echo "❌ Unsupported OS: $OS"
    exit 1
fi

# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Check if we're in development (local binary exists)
if [ -f "./target/release/slvsx" ]; then
    echo "📦 Using local development binary..."
    cp ./target/release/slvsx ~/.local/bin/slvsx
elif [ -f "./target/x86_64-unknown-linux-gnu/release/slvsx" ]; then
    echo "📦 Using local Linux target binary..."
    cp ./target/x86_64-unknown-linux-gnu/release/slvsx ~/.local/bin/slvsx
else
    # Download latest release
    echo "📦 Downloading latest release..."
    LATEST_URL="https://github.com/snoble/slvsx-cli/releases/latest/download/${BINARY_NAME}"
    
    if command -v curl &> /dev/null; then
        if ! curl -fsSL "$LATEST_URL" -o ~/.local/bin/slvsx 2>/dev/null; then
            echo "⚠️  No release found yet. For now, build from source:"
            echo "    git clone https://github.com/snoble/slvsx-cli.git"
            echo "    cd slvsx-cli && cargo build --release"
            exit 1
        fi
    elif command -v wget &> /dev/null; then
        if ! wget -q "$LATEST_URL" -O ~/.local/bin/slvsx 2>/dev/null; then
            echo "⚠️  No release found yet. Build from source."
            exit 1
        fi
    else
        echo "❌ Neither curl nor wget found. Please install one."
        exit 1
    fi
fi

# Make executable
chmod +x ~/.local/bin/slvsx

# Add to PATH if not already there
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "📝 Add this to your shell config (.bashrc, .zshrc, etc.):"
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

# Test installation
if ~/.local/bin/slvsx --version &> /dev/null; then
    echo "✅ Installation successful!"
    echo "📍 Installed to: ~/.local/bin/slvsx"
    echo ""
    echo "🎯 Quick test:"
    echo "   echo '{\"entities\":[{\"type\":\"point\",\"id\":\"p1\",\"at\":[0,0,0]}],\"constraints\":[],\"units\":\"mm\"}' | ~/.local/bin/slvsx solve -"
else
    echo "⚠️  Installation completed but test failed. Check your PATH."
fi