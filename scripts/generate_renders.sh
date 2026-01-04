#!/bin/bash
# Generate SVG renders for all examples
# This creates visualizations that can be used in documentation
#
# Usage:
#   ./scripts/generate_renders.sh [binary_path]
#
# If binary_path is not provided, will look for target/release/slvsx
# or target/x86_64-unknown-linux-gnu/release/slvsx

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_DIR="$PROJECT_ROOT/examples/outputs"

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# Set up environment
export SLVS_LIB_DIR="$PROJECT_ROOT/libslvs-static/build"

# Determine binary path
if [ -n "$1" ]; then
    BINARY="$1"
elif [ -f "$PROJECT_ROOT/target/x86_64-unknown-linux-gnu/release/slvsx" ]; then
    BINARY="$PROJECT_ROOT/target/x86_64-unknown-linux-gnu/release/slvsx"
elif [ -f "$PROJECT_ROOT/target/release/slvsx" ]; then
    BINARY="$PROJECT_ROOT/target/release/slvsx"
else
    echo "Error: slvsx binary not found. Please build it first:"
    echo "  cargo build --release"
    echo "Or provide the binary path as an argument:"
    echo "  $0 /path/to/slvsx"
    exit 1
fi

echo "Using binary: $BINARY"

echo "Generating SVG renders for examples..."

# Generate renders for 2D examples
for example in "$PROJECT_ROOT/examples"/*.json; do
    if [ -f "$example" ]; then
        name=$(basename "$example" .json)
        echo "  Rendering $name..."
        
        # Solve first (needed for export)
        "$BINARY" solve "$example" > /dev/null 2>&1 || true
        
        # Export to SVG
        "$BINARY" export -f svg "$example" -o "$OUTPUT_DIR/${name}.svg" 2>&1 || true
    fi
done

# Generate multiple views for 3D examples
echo "Generating 3D views..."

# Tetrahedron - all views including isometric
if [ -f "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" ]; then
    "$BINARY" solve "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" > /dev/null 2>&1
    "$BINARY" export -f svg -v xy "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" -o "$OUTPUT_DIR/tetrahedron_xy.svg" 2>&1
    "$BINARY" export -f svg -v xz "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" -o "$OUTPUT_DIR/tetrahedron_xz.svg" 2>&1
    "$BINARY" export -f svg -v yz "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" -o "$OUTPUT_DIR/tetrahedron_yz.svg" 2>&1
    "$BINARY" export -f svg -v isometric "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" -o "$OUTPUT_DIR/tetrahedron_isometric.svg" 2>&1
    echo "  Generated tetrahedron views (XY, XZ, YZ, Isometric)"
fi

# 3D basics - multiple views
if [ -f "$PROJECT_ROOT/examples/12_3d_basics.json" ]; then
    "$BINARY" solve "$PROJECT_ROOT/examples/12_3d_basics.json" > /dev/null 2>&1
    "$BINARY" export -f svg -v xy "$PROJECT_ROOT/examples/12_3d_basics.json" -o "$OUTPUT_DIR/3d_basics_xy.svg" 2>&1
    "$BINARY" export -f svg -v xz "$PROJECT_ROOT/examples/12_3d_basics.json" -o "$OUTPUT_DIR/3d_basics_xz.svg" 2>&1
    echo "  Generated 3D basics views"
fi

echo ""
echo "✓ Renders generated in $OUTPUT_DIR"
echo "  Total SVG files: $(ls -1 "$OUTPUT_DIR"/*.svg 2>/dev/null | wc -l | tr -d ' ')"
echo ""
echo "View the gallery: docs/VISUAL_GALLERY.md"

