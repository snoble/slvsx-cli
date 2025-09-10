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
- [ ] Rust binary rebuilt with new libslvs
- [ ] SIGABRT issue resolved
- [ ] CI updated and passing

## Next Steps
1. Complete Rust binary rebuild with nix-shell
2. Test that SIGABRT is fixed
3. Push changes to GitHub
4. Verify CI passes with the fix