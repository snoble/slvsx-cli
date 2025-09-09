#!/bin/bash
set -e

echo "Preparing to commit static build changes..."

# Add modified files
git add .github/workflows/build.yml
git add crates/core/build.rs

# Add the new libslvs-static directory
git add libslvs-static/

# Check what will be committed
echo "Files to be committed:"
git status --short

echo ""
echo "Ready to commit. Run:"
echo "git commit -m 'Add libslvs-static for fully static builds'"
echo "git push"