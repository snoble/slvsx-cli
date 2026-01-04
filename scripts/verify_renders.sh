#!/bin/bash
# Verify that renders are up-to-date
# This is a helper script for local development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Generate renders
echo "Generating renders..."
"$SCRIPT_DIR/generate_renders.sh"

# Check for differences
echo ""
echo "Checking for differences..."
if git diff --quiet examples/outputs/*.svg 2>/dev/null; then
    echo "✅ All renders are up-to-date!"
    exit 0
else
    echo "❌ Renders differ from committed versions:"
    git diff --stat examples/outputs/*.svg
    echo ""
    echo "To update renders, run:"
    echo "  ./scripts/generate_renders.sh"
    echo "  git add examples/outputs/*.svg"
    echo "  git commit -m 'chore: Update rendered SVGs'"
    exit 1
fi
