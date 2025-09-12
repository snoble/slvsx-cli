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
    BINARY_NAME="slvsx-macos"
else
    echo "❌ Unsupported OS: $OS"
    exit 1
fi

# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Download latest release
echo "📦 Downloading latest release..."
LATEST_URL="https://github.com/snoble/slvsx-cli/releases/latest/download/${BINARY_NAME}"

if command -v curl &> /dev/null; then
    curl -fsSL "$LATEST_URL" -o ~/.local/bin/slvsx
elif command -v wget &> /dev/null; then
    wget -q "$LATEST_URL" -O ~/.local/bin/slvsx
else
    echo "❌ Neither curl nor wget found. Please install one."
    exit 1
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