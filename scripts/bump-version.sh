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
# Must match the regex in .github/workflows/build.yml
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?(\+[a-zA-Z0-9.-]+)?$'; then
  echo "Error: Invalid version format '$VERSION'"
  echo "Expected: X.Y.Z, X.Y.Z-prerelease, or X.Y.Z+build"
  echo "Examples: 0.2.3, 0.2.3-beta.1, 0.2.3-rc-1, 0.2.3+build.123"
  exit 1
fi

echo "Bumping version to $VERSION..."

# Detect sed type (BSD vs GNU)
# GNU sed uses -i, BSD sed uses -i ''
if sed --version 2>&1 | grep -q GNU; then
  SED_INPLACE="sed -i"
else
  SED_INPLACE="sed -i ''"
fi

# Update Cargo.toml (workspace version)
$SED_INPLACE "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Update package.json
$SED_INPLACE "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json

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

