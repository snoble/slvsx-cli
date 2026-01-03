#!/bin/bash
set -e

echo "=== Building slvsx ==="

# Determine number of parallel jobs
if command -v nproc &> /dev/null; then
    JOBS=$(nproc)
elif command -v sysctl &> /dev/null; then
    JOBS=$(sysctl -n hw.ncpu)
else
    JOBS=4
fi

# Build libslvs-static
echo ""
echo "=== Building libslvs-static ==="
cd libslvs-static
mkdir -p build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$JOBS
cd ../..

# Set environment for Rust build
export SLVS_LIB_DIR=$PWD/libslvs-static/build

# Build slvsx
echo ""
echo "=== Building slvsx (Rust) ==="
cargo build --release

# Show result
echo ""
echo "=== Build complete ==="
echo "Binary: $(pwd)/target/release/slvsx"
echo ""
./target/release/slvsx --version
