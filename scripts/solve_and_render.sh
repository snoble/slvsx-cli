#!/bin/bash
# Solve and render a constraint model
# Usage: ./scripts/solve_and_render.sh <input.json> [output_dir]

set -e

INPUT_FILE="$1"
OUTPUT_DIR="${2:-examples/outputs}"

if [ -z "$INPUT_FILE" ]; then
    echo "Usage: $0 <input.json> [output_dir]"
    exit 1
fi

BASENAME=$(basename "$INPUT_FILE" .json)

echo "Solving $INPUT_FILE..."
export SLVS_LIB_DIR="$PWD/libslvs-static/build"

# Solve
if ! nix-shell -p cargo rustc cmake pkg-config --run "./target/release/slvsx solve $INPUT_FILE" > /tmp/solve_output.json 2>&1; then
    echo "❌ Solve failed!"
    cat /tmp/solve_output.json
    exit 1
fi

echo "✓ Solve successful"

# Generate renders for all views
for view in xy xz yz isometric; do
    OUTPUT="$OUTPUT_DIR/${BASENAME}_${view}.svg"
    echo "  Generating ${view} view..."
    nix-shell -p cargo rustc cmake pkg-config --run "./target/release/slvsx export $INPUT_FILE --format svg --view $view --output $OUTPUT" > /dev/null 2>&1
    echo "    → $OUTPUT"
done

echo "✓ All renders generated in $OUTPUT_DIR"

