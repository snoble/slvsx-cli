#!/bin/bash
set -e

echo "Building minimal static libslvs..."

cd libslvs/SolveSpaceLib

# Clean any previous builds
rm -rf build-minimal
mkdir -p build-minimal
cd build-minimal

# Compiler flags
CXX="${CXX:-c++}"
CXXFLAGS="-O3 -fPIC -std=c++11 -DLIBRARY -DSTATIC_LIB"
INCLUDES="-I../include -I../src -I../extlib/eigen -I../extlib/mimalloc/include"

echo "Compiling solver core files..."

# Compile the core solver files
$CXX -c $CXXFLAGS $INCLUDES ../src/util.cpp -o util.o
$CXX -c $CXXFLAGS $INCLUDES ../src/entity.cpp -o entity.o
$CXX -c $CXXFLAGS $INCLUDES ../src/expr.cpp -o expr.o
$CXX -c $CXXFLAGS $INCLUDES ../src/constrainteq.cpp -o constrainteq.o
$CXX -c $CXXFLAGS $INCLUDES ../src/system.cpp -o system.o
$CXX -c $CXXFLAGS $INCLUDES ../src/platform/platformbase.cpp -o platformbase.o

# Compile the slvs library interface
$CXX -c $CXXFLAGS $INCLUDES ../src/slvs/lib.cpp -o lib.o

echo "Creating static library..."

# Create the static library
ar rcs libslvs.a \
    util.o \
    entity.o \
    expr.o \
    constrainteq.o \
    system.o \
    platformbase.o \
    lib.o

# Create the expected directory structure
mkdir -p ../build/src/slvs
cp libslvs.a ../build/src/slvs/

echo "Static library created at: $(pwd)/libslvs.a"
echo "Also copied to: ../build/src/slvs/libslvs.a"

# Verify the library
echo "Library contents:"
ar -t libslvs.a

echo "Build complete!"