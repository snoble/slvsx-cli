#!/bin/bash

echo "Setting up local CI for SLVSX..."

# Configure git to use our hooks
git config core.hooksPath .githooks

echo "✅ Git hooks configured"
echo ""
echo "Local CI is now set up!"
echo ""
echo "Available commands:"
echo "  ./run-ci-local.sh           - Run CI locally"
echo "  ./run-ci-local.sh --push    - Run CI and push results to GitHub"
echo ""
echo "The pre-push hook will remind you to run CI before pushing."
echo ""
echo "To disable the reminder:"
echo "  git config core.hooksPath .git/hooks"