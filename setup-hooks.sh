#!/usr/bin/env bash
set -e

# Configure git to use the custom hooks directory
git config core.hooksPath git-hooks

# Download Biome for HTML linting and formatting if not present
if ! command -v biome &> /dev/null && [ ! -f "./biome" ]; then
    echo "Downloading Biome standalone binary..."
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    if [ "$ARCH" = "x86_64" ]; then ARCH="x64"; fi
    if [ "$ARCH" = "aarch64" ]; then ARCH="arm64"; fi
    if [ "$OS" = "darwin" ] || [ "$OS" = "linux" ]; then
        curl -sLo biome "https://github.com/biomejs/biome/releases/latest/download/biome-${OS}-${ARCH}"
        chmod +x biome
    else
        echo "Please install Biome manually: https://biomejs.dev/guides/getting-started/#installation"
    fi
fi

echo "Git pre-commit hook configured successfully."
