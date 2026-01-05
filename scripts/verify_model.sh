#!/bin/bash
# Verify a constraint model: validate, solve, and render
# Usage: ./scripts/verify_model.sh <input.json>

set -e

INPUT_FILE="$1"

if [ -z "$INPUT_FILE" ]; then
    echo "Usage: $0 <input.json>"
    exit 1
fi

echo "🔍 Verifying $INPUT_FILE..."
echo ""

export SLVS_LIB_DIR="$PWD/libslvs-static/build"

# Step 1: Validate
echo "1. Validating JSON schema..."
if nix-shell -p cargo rustc cmake pkg-config --run "./target/release/slvsx validate $INPUT_FILE" 2>&1; then
    echo "   ✓ Schema valid"
else
    echo "   ❌ Schema validation failed"
    exit 1
fi

# Step 2: Solve
echo ""
echo "2. Solving constraints..."
if nix-shell -p cargo rustc cmake pkg-config --run "./target/release/slvsx solve $INPUT_FILE" 2>&1 | head -10; then
    echo "   ✓ Solve successful"
else
    echo "   ❌ Solve failed"
    exit 1
fi

# Step 3: Render
echo ""
echo "3. Generating renders..."
./scripts/solve_and_render.sh "$INPUT_FILE"

echo ""
echo "✅ Model verification complete!"

