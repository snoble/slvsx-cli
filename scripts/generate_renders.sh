#!/bin/bash
# Generate SVG renders for all examples
# This creates visualizations that can be used in documentation

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_DIR="$PROJECT_ROOT/examples/outputs"

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# Set up environment
export SLVS_LIB_DIR="$PROJECT_ROOT/libslvs-static/build"

# Check if binary exists
if [ ! -f "$PROJECT_ROOT/target/release/slvsx" ]; then
    echo "Building slvsx..."
    cd "$PROJECT_ROOT"
    cargo build --release
fi

BINARY="$PROJECT_ROOT/target/release/slvsx"

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

# Tetrahedron - all three views
if [ -f "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" ]; then
    "$BINARY" solve "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" > /dev/null 2>&1
    "$BINARY" export -f svg -v xy "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" -o "$OUTPUT_DIR/tetrahedron_xy.svg" 2>&1
    "$BINARY" export -f svg -v xz "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" -o "$OUTPUT_DIR/tetrahedron_xz.svg" 2>&1
    "$BINARY" export -f svg -v yz "$PROJECT_ROOT/examples/04_3d_tetrahedron.json" -o "$OUTPUT_DIR/tetrahedron_yz.svg" 2>&1
    echo "  Generated tetrahedron views (XY, XZ, YZ)"
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

