#!/usr/bin/env bash
set -euo pipefail

# Create a new release
# Usage: ./scripts/create-release.sh v0.1.0

if [ $# -ne 1 ]; then
    echo "Usage: $0 <version-tag>"
    echo "Example: $0 v0.1.0"
    exit 1
fi

VERSION="$1"

# Validate version format
if [[ ! "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in format v0.0.0"
    exit 1
fi

echo "Creating release $VERSION..."

# Update version in Cargo.toml
SEMVER="${VERSION#v}"  # Remove 'v' prefix
sed -i.bak "s/^version = .*/version = \"$SEMVER\"/" Cargo.toml
rm Cargo.toml.bak

# Update version in all crates
find crates -name Cargo.toml -exec sed -i.bak "s/^version = .*/version = \"$SEMVER\"/" {} \;
find crates -name "*.bak" -delete

echo "Updated version to $SEMVER in Cargo.toml files"

# Commit version update
git add .
git commit -m "Bump version to $VERSION"

# Create and push tag
git tag -a "$VERSION" -m "Release $VERSION

Features:
- JSON-based constraint specification with schema validation
- Support for gears, circles, points, lines, and various constraint types
- Two-phase solving: geometric positioning + gear phase alignment  
- SVG/DXF/STL export capabilities
- WASM bindings for browser integration
- Comprehensive test suite with coverage reporting
- Static binaries for multiple platforms
- Nix build system with reproducible builds

Built with love and Rust 🦀"

git push origin main
git push origin "$VERSION"

echo "Release $VERSION created and pushed!"
echo "GitHub Actions will automatically build and publish binaries."
echo "Visit: https://github.com/snoble/slvsx-cli/releases"