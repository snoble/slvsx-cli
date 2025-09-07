# Build Guide for SLVSX

## Prerequisites

This project uses Nix for reproducible builds. All commands should be run through `nix-shell`.

## Step-by-Step Build Instructions

### 1. Clone and Initialize Submodules

```bash
git clone https://github.com/snoble/slvsx-cli.git
cd slvsx-cli
git submodule update --init --recursive
```

### 2. Build libslvs (C++ constraint solver)

```bash
cd libslvs/SolveSpaceLib
mkdir -p build
cd build
nix-shell ../../../build.nix --run "cmake .. && make slvs -j4"
cd ../../../
```

The library will be built at: `libslvs/SolveSpaceLib/build/bin/libslvs.dylib` (or `.so` on Linux)

### 3. Build the Rust CLI

```bash
nix-shell build.nix --run "DYLD_LIBRARY_PATH=./libslvs/SolveSpaceLib/build/bin cargo build --release"
```

### 4. Run the CLI

```bash
export DYLD_LIBRARY_PATH="./libslvs/SolveSpaceLib/build/bin:$DYLD_LIBRARY_PATH"
./target/release/slvsx --help
```

## Common Issues and Solutions

### Issue: "slvs.h not found"
- **Solution**: Ensure submodules are initialized with `git submodule update --init --recursive`

### Issue: "library not found for -lslvs"
- **Solution**: Build libslvs first (step 2) before building Rust code

### Issue: "dyld: Library not loaded"
- **Solution**: Set DYLD_LIBRARY_PATH (or LD_LIBRARY_PATH on Linux) to include the libslvs build directory

### Issue: CMake errors about missing submodules (pixman, cairo, etc)
- **Solution**: Run `git submodule update --init --recursive` to fetch all dependencies

## Quick Rebuild Commands

After initial setup, use these commands for rebuilding:

```bash
# Rebuild everything
nix-shell build.nix --run "cd libslvs/SolveSpaceLib/build && make slvs -j4 && cd ../../../ && DYLD_LIBRARY_PATH=./libslvs/SolveSpaceLib/build/bin cargo build --release"

# Just rebuild Rust code
nix-shell build.nix --run "DYLD_LIBRARY_PATH=./libslvs/SolveSpaceLib/build/bin cargo build --release"

# Run tests
nix-shell build.nix --run "DYLD_LIBRARY_PATH=./libslvs/SolveSpaceLib/build/bin cargo test"
```

## Running Examples

```bash
export DYLD_LIBRARY_PATH="./libslvs/SolveSpaceLib/build/bin:$DYLD_LIBRARY_PATH"
./target/release/slvsx solve examples/01_basic_distance.json
./target/release/slvsx export --format svg --output output.svg examples/02_triangle.json
```