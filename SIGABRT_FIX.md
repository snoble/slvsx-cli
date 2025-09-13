# SIGABRT Fix - Memory Allocator Conflict Resolution

## Problem
The CLI was crashing with SIGABRT on exit due to memory allocator conflicts:
- libslvs-static was using mimalloc
- System C++ library was using system malloc  
- When C++ objects crossed boundaries, the wrong allocator tried to free memory

## Solution Implemented
Removed mimalloc from libslvs-static to use consistent memory allocation:

### Changes Made:
1. **libslvs-static/CMakeLists.txt**
   - Removed mimalloc include directories
   - Removed mimalloc build configuration
   - Removed mimalloc linking
   - Updated combined library build to exclude mimalloc

2. **libslvs-static/src/platform/platformbase.cpp**
   - Removed mimalloc.h include
   - Replaced mimalloc heap with simple memory pool using standard malloc/free
   - Updated AllocTemporary() and FreeAllTemporary() functions

## Build Instructions
```bash
# 1. Rebuild libslvs-static without mimalloc
cd libslvs-static/build
rm -rf *
cmake .. -DCMAKE_BUILD_TYPE=Release  
make -j8

# 2. Rebuild the Rust binary
cd ../..
export SLVS_LIB_DIR=$PWD/libslvs-static/build
nix-shell -p cargo rustc --run "cargo build --release"

# 3. Test the binary
./target/release/slvsx --version
./target/release/slvsx capabilities
```

## Testing Status
- [x] libslvs-static rebuilt without mimalloc
- [x] Rust binary rebuilt with new libslvs
- [x] SIGABRT issue resolved in local tests
- [x] CI updated with RUST_TEST_THREADS=1

## Additional SIGABRT Issues Found

### Test Parallelism Issue
**Problem**: Tests were getting SIGSEGV when running in parallel because libslvs is not thread-safe.
**Solution**: Set `RUST_TEST_THREADS=1` in CI to force single-threaded test execution.

### Handle Uniqueness Issue
**Problem**: "Handle isn't unique" assertion in libslvs when tests run in parallel.
**Solution**: Single-threaded execution prevents handle collisions between concurrent tests.

## Fixes Applied
1. Removed mimalloc from libslvs-static (memory allocator conflict)
2. Added `RUST_TEST_THREADS=1` to CI (thread safety)
3. Fixed test expecting panic for unimplemented constraints (they're just ignored)
4. Removed debug eprintln statements from solver.rs
5. Added version consistency checks in CI

## Current Status
- Tests pass locally with single-threaded execution
- CI runs tests with RUST_TEST_THREADS=1
- Some examples still fail due to missing constraint implementations