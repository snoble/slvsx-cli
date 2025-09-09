# Claude.md - slvsx-cli Project Knowledge Base

## Project Overview

slvsx-cli is a command-line interface wrapper for the SolveSpace constraint solver library (libslvs). The primary goal is to create a **fully static binary** that includes libslvs, so users don't need to install libslvs separately.

### Critical Requirements
- **Static Binaries**: All artifacts MUST be statically linked. This is non-negotiable.
- **No Mocking**: The project depends on libslvs actually working. Never attempt to mock it.
- **Cross-Platform**: Must build on Ubuntu Linux and macOS
- **Fork Only**: We use only the libslvs-static fork, not the original SolveSpace submodule

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
├── libslvs-static/        # Fork with static build and mimalloc
│   ├── include/slvs.h
│   └── src/               # Pre-built static library
├── examples/              # Example .slvs files
└── tests/                # Integration tests
```

## Build System

### Local Build Process

1. **Build libslvs-static library**:
   ```bash
   cd libslvs-static
   mkdir build && cd build
   cmake .. -DCMAKE_BUILD_TYPE=Release
   make
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
  - `libslvs-static/build/` (default)
  - `$SLVS_LIB_DIR` (environment variable for CI)
- Links required system libraries (stdc++ on Linux, c++ on macOS)

#### libslvs-static Fork
The fork includes:
- Static library build by default
- mimalloc integration for better memory management
- Simplified CMake without GUI dependencies
- Pre-built `libslvs-combined.a` that includes all dependencies

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
   
3. **Build libslvs-static Library**
   - Navigate to: `libslvs-static`
   - Simple CMake build without GUI
   - Output: `libslvs-combined.a`
   
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

3. **Fork Usage**: We use the libslvs-static fork exclusively, which has all necessary modifications for static building.

4. **Build Order Matters**: 
   - First: Build libslvs static library
   - Second: Build slvsx with proper environment variables
   - Third: Verify static linking

5. **Platform Differences**:
   - Linux uses `stdc++`, macOS uses `c++`
   - Linux needs explicit static flags for gcc/g++
   - macOS handles some static linking differently

## Current Solution

We use the libslvs-static fork which:
- Removes GUI dependencies
- Focuses only on slvs library
- Has static build as default
- Includes mimalloc for memory management
- Provides `libslvs-combined.a` with all dependencies

This eliminates CI build issues and ensures consistent static builds.

## Commands Reference

```bash
# Build libslvs-static
cd libslvs-static && \
  mkdir -p build && cd build && \
  cmake .. -DCMAKE_BUILD_TYPE=Release && \
  make

# Build with static linking
export SLVS_LIB_DIR=$PWD/libslvs-static/build
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
*Last updated: 2024-09-09*
*This document should be updated whenever significant changes are made to the build system or CI configuration.*