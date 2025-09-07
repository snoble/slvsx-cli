#!/bin/bash
set -e

echo "Building static libslvs..."

cd libslvs/SolveSpaceLib

# Clean build directory
rm -rf build
mkdir -p build
cd build

# Configure with minimal dependencies - only build slvs library
cmake .. \
  -DCMAKE_BUILD_TYPE=Release \
  -DBUILD_SHARED_LIBS=OFF \
  -DENABLE_GUI=OFF \
  -DENABLE_TESTS=OFF \
  -DENABLE_OPENGL=OFF \
  -DCMAKE_POLICY_VERSION_MINIMUM=3.1 || {
    echo "CMake configuration failed. Trying alternative approach..."
    
    # If full cmake fails, try building just the slvs component directly
    cd ../src/slvs
    mkdir -p build
    cd build
    
    # Minimal build for just slvs
    cmake ../../../ \
      -DCMAKE_BUILD_TYPE=Release \
      -DBUILD_SHARED_LIBS=OFF \
      -DCMAKE_POLICY_VERSION_MINIMUM=3.1
}

# Build
make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu) slvs_static || make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu) slvs

# Find and report static libraries
echo "Static libraries built:"
find . -name "*.a" -ls

echo "Build complete!"