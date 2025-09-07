#!/bin/bash
set -e

# Build libslvs with Emscripten
echo "Building libslvs with Emscripten..."
cd libslvs/SolveSpaceLib

# Create build directory for Emscripten
mkdir -p build-wasm
cd build-wasm

# Configure with Emscripten
emcmake cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DENABLE_GUI=OFF \
    -DENABLE_TESTS=OFF

# Build libslvs WASM
emmake make slvs-wasm -j$(nproc)

# Copy the generated JS file
cp src/slvs/slvs.js ../../../wasm-dist/libslvs.js

cd ../../../

echo "libslvs WASM build complete!"

# Now build our Rust WASM wrapper that will use libslvs
echo "Building Rust WASM wrapper..."
cd crates/core
wasm-pack build --target web --features wasm --out-dir ../../wasm-dist

echo "WASM build complete!"