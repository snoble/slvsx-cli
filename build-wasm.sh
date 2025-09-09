#!/bin/bash
set -e

echo "Building SLVSX WASM Module"
echo "=========================="

# Build libslvs-static first if needed
if [ ! -f libslvs-static/build/libslvs-combined.a ]; then
    echo "Building libslvs-static..."
    mkdir -p libslvs-static/build
    cd libslvs-static/build
    cmake .. -DCMAKE_BUILD_TYPE=Release
    make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu)
    cd ../..
fi

# Set environment for WASM build
export SLVS_LIB_DIR="$PWD/libslvs-static/build"
export SLVS_STATIC=1
export SLVS_USE_FORK=1

# Build WASM with wasm-pack
echo "Building WASM module..."
cd crates/core
wasm-pack build --target web --out-dir ../../examples/web-demo/pkg

echo "WASM build complete!"
echo "Open examples/web-demo/index.html in a browser to test"
