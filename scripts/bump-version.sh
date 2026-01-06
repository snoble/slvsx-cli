#!/bin/bash
# Bump version across all files and create a git tag
# Usage: ./scripts/bump-version.sh 0.2.3

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
  echo "Usage: ./scripts/bump-version.sh <version>"
  echo "Example: ./scripts/bump-version.sh 0.2.3"
  echo ""
  echo "Current versions:"
  echo "  Cargo.toml:   $(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')"
  echo "  package.json: $(jq -r .version package.json)"
  exit 1
fi

# Validate version format (semver)
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
  echo "Error: Invalid version format '$VERSION'"
  echo "Expected: X.Y.Z or X.Y.Z-suffix (e.g., 0.2.3 or 0.2.3-rc1)"
  exit 1
fi

echo "Bumping version to $VERSION..."

# Update Cargo.toml (workspace version)
if [[ "$OSTYPE" == "darwin"* ]]; then
  sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
else
  sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
fi

# Update package.json
if [[ "$OSTYPE" == "darwin"* ]]; then
  sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
else
  sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
fi

# Update Cargo.lock
echo "Updating Cargo.lock..."
cargo update --workspace

# Verify versions match
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
NPM_VERSION=$(jq -r .version package.json)

if [ "$CARGO_VERSION" != "$VERSION" ]; then
  echo "Error: Cargo.toml version ($CARGO_VERSION) doesn't match target ($VERSION)"
  exit 1
fi

if [ "$NPM_VERSION" != "$VERSION" ]; then
  echo "Error: package.json version ($NPM_VERSION) doesn't match target ($VERSION)"
  exit 1
fi

echo "✓ Cargo.toml:   $CARGO_VERSION"
echo "✓ package.json: $NPM_VERSION"

# Stage changes
git add Cargo.toml Cargo.lock package.json

# Commit
git commit -m "chore: bump version to $VERSION"

# Create tag
git tag "v$VERSION"

echo ""
echo "✓ Version bumped to $VERSION"
echo "✓ Tag v$VERSION created"
echo ""
echo "To release, run:"
echo "  git push && git push origin v$VERSION"

