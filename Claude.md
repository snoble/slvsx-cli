# Claude.md - slvsx-cli Project Knowledge Base

## Project Overview

slvsx-cli is a command-line interface wrapper for the SolveSpace constraint solver library (libslvs). The primary goal is to create a **fully static binary** that includes libslvs, so users don't need to install libslvs separately.

### Critical Requirements
- **Static Binaries**: All artifacts MUST be statically linked. This is non-negotiable.
- **No Mocking**: The project depends on libslvs actually working. Never attempt to mock it.
- **Cross-Platform**: Must build on Ubuntu Linux and macOS

## Repository Structure

```
slvsx-cli/
├── .github/workflows/
│   ├── build.yml           # Main CI workflow - builds static binaries
│   ├── ci-status.yml       # CI status reporting
│   └── release.yml         # Release workflow
├── crates/
│   ├── core/              # Core library with FFI bindings to libslvs
│   │   ├── build.rs       # Build script that links libslvs statically
│   │   └── src/
│   ├── cli/               # CLI application
│   └── exporters/         # Export functionality
├── ffi/
│   ├── real_slvs_wrapper.c  # C wrapper for libslvs
│   └── slvs_wrapper.h
├── libslvs/
│   └── SolveSpaceLib/     # Git submodule of SolveSpace
│       ├── include/slvs.h
│       └── src/slvs/      # Modified to build static library
├── examples/              # Example .slvs files
└── tests/                # Integration tests
```

## Build System

### Local Build Process

1. **Build libslvs static library**:
   ```bash
   cd libslvs/SolveSpaceLib
   mkdir build && cd build
   cmake .. -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF
   make slvs_static
   ```

2. **Build slvsx with static linking**:
   ```bash
   cargo build --release
   ```

### Key Build Files

#### crates/core/build.rs
- Links libslvs statically
- Compiles real_slvs_wrapper.c
- Searches for libraries in:
  - `libslvs/SolveSpaceLib/build/src/slvs/` (default)
  - `$SLVS_LIB_DIR` (environment variable for CI)
- Links required system libraries (stdc++ on Linux, c++ on macOS)

#### libslvs/SolveSpaceLib Modifications
The submodule has been modified in `src/slvs/CMakeLists.txt` to add:
```cmake
# Also build static library
add_library(slvs_static STATIC)
target_compile_definitions(slvs_static PRIVATE -DSTATIC_LIB)
target_link_libraries(slvs_static PRIVATE slvs-interface)
set_target_properties(slvs_static PROPERTIES OUTPUT_NAME slvs)
```

## GitHub Actions CI

### Current Issues (as of last run)

1. **CMake Build Complexity**: SolveSpace's full CMake tries to build GUI components we don't need
2. **Missing Dependencies**: Ubuntu needs libpng-dev and zlib1g-dev
3. **Path Issues**: Submodule is at `libslvs/SolveSpaceLib`, not `SolveSpaceLib`

### Required CI Steps

Each step in `.github/workflows/build.yml` must:

1. **Checkout with Submodules**
   - Must use `submodules: recursive`
   
2. **Install Dependencies**
   - Ubuntu: `cmake build-essential libpng-dev zlib1g-dev`
   - macOS: cmake is pre-installed
   
3. **Build libslvs Static Library**
   - Navigate to correct path: `libslvs/SolveSpaceLib`
   - Use minimal CMake flags to avoid GUI dependencies
   - Target: `slvs_static` (our custom target)
   
4. **Build slvsx Binary**
   - Set `SLVS_LIB_DIR` environment variable
   - Use `RUSTFLAGS` for static linking:
     - Linux: `-C target-feature=+crt-static -C link-arg=-static-libgcc -C link-arg=-static-libstdc++`
     - macOS: `-C target-feature=+crt-static`
   
5. **Verify Static Linking**
   - Linux: `ldd` should show "not a dynamic executable"
   - macOS: `otool -L` should show minimal dependencies
   
6. **Upload Artifacts**
   - Must upload the static binary from correct path

## Known Issues & Solutions

### Issue: CMake fails with GUI dependencies
**Solution**: Use targeted build of just slvs library, or create fork without GUI

### Issue: Missing libslvs.a during Rust build
**Solution**: Ensure CMake build completes and check library paths in build.rs

### Issue: Dynamic dependencies in final binary
**Solution**: Use proper RUSTFLAGS and ensure all dependencies are statically linked

## Testing

### Local Testing Commands
```bash
# Build everything
./build-libslvs-static.sh
cargo build --release

# Verify static linking
ldd target/release/slvsx  # Linux
otool -L target/release/slvsx  # macOS

# Run tests
cargo test
./test-examples.sh
```

### CI Testing
```bash
# Run CI locally
nix-shell -p gh
gh run list --workflow=build.yml
gh run view <run-id> --log-failed
```

## Important Notes for Future Agents

1. **Never Mock libslvs**: The solver must actually work. Don't create mock implementations.

2. **Static Linking is Critical**: The entire purpose is to create a standalone binary. Always verify with `ldd` or `otool`.

3. **Submodule Modifications**: The libslvs/SolveSpaceLib submodule has local modifications for static building. These need to be preserved or moved to a fork.

4. **Build Order Matters**: 
   - First: Build libslvs static library
   - Second: Build slvsx with proper environment variables
   - Third: Verify static linking

5. **Platform Differences**:
   - Linux uses `stdc++`, macOS uses `c++`
   - Linux needs explicit static flags for gcc/g++
   - macOS handles some static linking differently

## Recommended Long-term Solution

1. **Fork SolveSpace/solvespace** repository
2. **Create a minimal branch** that:
   - Removes GUI dependencies
   - Focuses only on slvs library
   - Has static build as default
3. **Update submodule** to point to the fork
4. **Simplify build process** with the fork

This will eliminate most CI build issues and ensure consistent static builds.

## Commands Reference

```bash
# Update submodules
git submodule update --init --recursive

# Build libslvs manually
cd libslvs/SolveSpaceLib && \
  mkdir -p build && cd build && \
  cmake .. -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF && \
  make slvs_static

# Build with static linking
export SLVS_LIB_DIR=$PWD/libslvs/SolveSpaceLib/build/src/slvs
export RUSTFLAGS="-C target-feature=+crt-static"
cargo build --release

# Check static linking
file target/release/slvsx
ldd target/release/slvsx 2>/dev/null || echo "Static binary!"
```

## Contact & Resources

- Main repository: https://github.com/snoble/slvsx-cli
- SolveSpace upstream: https://github.com/solvespace/solvespace
- Issue tracker: Use GitHub Issues for problems

---
*Last updated: 2024-09-07*
*This document should be updated whenever significant changes are made to the build system or CI configuration.*