# Building SLVSX

This document provides detailed instructions for building SLVSX from source.

## Quick Start (with Nix)

The easiest way to build SLVSX is using Nix, which provides a reproducible build environment:

```bash
git clone --recursive https://github.com/snoble/slvsx-cli.git
cd slvsx-cli/slvsx/slvsx-cli
nix-shell build.nix
cargo build --release
```

## Manual Build

### Prerequisites

- **Rust**: 1.74+ stable (install via [rustup](https://rustup.rs/))
- **CMake**: 3.10+ for building libslvs
- **C++ Compiler**: GCC, Clang, or MSVC
- **Make**: GNU Make or Ninja
- **Git**: With submodule support

### Step 1: Clone Repository

```bash
git clone --recursive https://github.com/snoble/slvsx-cli.git
cd slvsx-cli/slvsx/slvsx-cli
```

If you already cloned without `--recursive`:
```bash
git submodule update --init --recursive
```

### Step 2: Build libslvs Static Library

SLVSX uses SolveSpace's constraint solver library. We build it as a static library to create standalone binaries:

```bash
cd libslvs/SolveSpaceLib
mkdir -p build
cd build

# Configure CMake
cmake .. -DCMAKE_BUILD_TYPE=Release

# Build static library
make slvs_static -j$(nproc)  # Linux/macOS
# or
make slvs_static -j%NUMBER_OF_PROCESSORS%  # Windows

cd ../../..
```

The static library will be created at `libslvs/SolveSpaceLib/build/bin/libslvs.a` (or `.lib` on Windows).

### Step 3: Build SLVSX

```bash
cargo build --release
```

The binary will be at `target/release/slvsx` (or `slvsx.exe` on Windows).

### Step 4: Verify Build

```bash
./target/release/slvsx --help
./target/release/slvsx solve examples/01_basic_distance.json
```

## Platform-Specific Notes

### macOS

On macOS, the default CMake from Homebrew works well:
```bash
brew install cmake
```

### Linux

Most distributions provide suitable packages:
```bash
# Ubuntu/Debian
sudo apt-get install cmake build-essential

# Fedora
sudo dnf install cmake gcc-c++ make

# Arch
sudo pacman -S cmake base-devel
```

### Windows

Options:
1. **Visual Studio**: Install Visual Studio with C++ workload
2. **MinGW**: Use MinGW-w64 for GCC on Windows
3. **WSL2**: Build in Windows Subsystem for Linux

## Cross-Compilation

### For Different Architectures

```bash
# Add target
rustup target add aarch64-unknown-linux-gnu

# Build (requires appropriate cross-compiler)
cargo build --release --target aarch64-unknown-linux-gnu
```

### For WASM

```bash
# Install dependencies
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# Build WASM module
cd crates/core
wasm-pack build --target web --features wasm
```

## Troubleshooting

### "Library not loaded" Error

If you get dynamic library errors when running the binary, ensure libslvs was built as a static library:
```bash
# Check if static library exists
ls libslvs/SolveSpaceLib/build/bin/libslvs.a

# Rebuild if needed
cd libslvs/SolveSpaceLib/build
cmake .. -DBUILD_STATIC=ON
make slvs_static
```

### CMake Can't Find Compiler

Specify the compiler explicitly:
```bash
cmake .. -DCMAKE_C_COMPILER=gcc -DCMAKE_CXX_COMPILER=g++
```

### Rust Version Too Old

Update Rust:
```bash
rustup update stable
rustup default stable
```

### Build Fails on M1/M2 Mac

Ensure you're using native ARM tools:
```bash
rustup default stable-aarch64-apple-darwin
```

## Development Builds

### Debug Build

```bash
cargo build  # No --release flag
./target/debug/slvsx --help
```

### Running Tests

```bash
cargo test
```

### Format Check

```bash
cargo fmt --check
```

## CI/CD

The project uses GitHub Actions for releases. See `.github/workflows/release.yml` for the automated build process.

Local CI can be run with:
```bash
./run-ci-local.sh
```

This updates the CI status badge in the README.

## Contributing

When contributing, please ensure:
1. Code passes `cargo fmt`
2. Tests pass with `cargo test`
3. Binary builds and runs examples successfully