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
CC="${CC:-cc}"
CXXFLAGS="-O3 -fPIC -std=c++11 -DLIBRARY -DSTATIC_LIB"
CFLAGS="-O3 -fPIC"
INCLUDES="-I../include -I../src -I../extlib/eigen -I../extlib/mimalloc/include"

echo "Building minimal mimalloc..."

# Build minimal mimalloc (just the core)
$CC -c $CFLAGS -I../extlib/mimalloc/include ../extlib/mimalloc/src/heap.c -o heap.o
$CC -c $CFLAGS -I../extlib/mimalloc/include ../extlib/mimalloc/src/alloc.c -o alloc.o
$CC -c $CFLAGS -I../extlib/mimalloc/include ../extlib/mimalloc/src/init.c -o init.o
$CC -c $CFLAGS -I../extlib/mimalloc/include ../extlib/mimalloc/src/os.c -o os.o
$CC -c $CFLAGS -I../extlib/mimalloc/include ../extlib/mimalloc/src/page.c -o page.o
$CC -c $CFLAGS -I../extlib/mimalloc/include ../extlib/mimalloc/src/segment.c -o segment.o
$CC -c $CFLAGS -I../extlib/mimalloc/include ../extlib/mimalloc/src/arena.c -o arena.o

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

# Create the static library including mimalloc
ar rcs libslvs.a \
    util.o entity.o expr.o constrainteq.o system.o platformbase.o lib.o \
    heap.o alloc.o init.o os.o page.o segment.o arena.o

# Create the expected directory structure
mkdir -p ../build/src/slvs
cp libslvs.a ../build/src/slvs/

echo "Static library created at: $(pwd)/libslvs.a"
echo "Also copied to: ../build/src/slvs/libslvs.a"

# Verify the library
echo "Library contents:"
ar -t libslvs.a

echo "Build complete!"