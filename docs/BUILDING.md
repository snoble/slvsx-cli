# Building SLVSX

## Quick Start

### Using Pre-built Binaries (Recommended)
Download static binaries from [GitHub Releases](https://github.com/snoble/slvsx-cli/releases).

### Building from Source

#### Prerequisites
- Rust toolchain (1.70+)
- CMake (3.10+)
- C++ compiler
- Git

#### Build Commands

```bash
# Clone with submodules
git clone --recursive https://github.com/snoble/slvsx-cli
cd slvsx-cli

# Build using nix (recommended for consistent environment)
nix-shell build.nix --run "cargo build --release"

# Or build directly
cd libslvs-static
mkdir -p build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j
cd ../..
export SLVS_LIB_DIR=$PWD/libslvs-static/build
export SLVS_USE_FORK=1
cargo build --release
```

The binary will be at `target/release/slvsx`.

## Build System Details

### Project Structure
- `libslvs-static/` - Fork of SolveSpace with static library build
- `crates/core/` - Core library with FFI bindings to libslvs
- `crates/cli/` - Command-line interface
- `crates/exporters/` - Export formats (SVG, DXF, etc.)

### Static Linking
SLVSX produces a fully static binary that includes:
- libslvs (SolveSpace constraint solver)
- mimalloc (memory allocator)
- All Rust dependencies

This means users don't need to install any libraries - the binary is self-contained.

### Platform-Specific Notes

#### macOS
- Links against system libc++ and libSystem
- Binary is mostly static except for system libraries

#### Linux
- Fully static binary when built with proper flags
- Uses musl libc for maximum portability

#### Windows
- Not yet tested (contributions welcome)

### CI/CD
GitHub Actions builds static binaries for:
- Ubuntu Linux (x86_64)
- macOS (arm64 and x86_64)

See `.github/workflows/build.yml` for the exact build process.

## Troubleshooting

### CMake Can't Find Dependencies
```bash
# Install required packages
# Ubuntu/Debian:
sudo apt-get install cmake build-essential libpng-dev zlib1g-dev

# macOS:
brew install cmake
```

### Linking Errors
If you see undefined symbols, ensure libslvs-static is built:
```bash
cd libslvs-static
mkdir -p build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make slvs-combined
```

### Test Failures
Tests may show SIGABRT on exit due to allocator cleanup issues. This doesn't affect functionality. See `docs/DEVELOPMENT.md` for details.

## Development

### Running Tests
```bash
cargo test
./test-examples.sh
```

### Building WASM Version
```bash
./build-wasm.sh
# Output in wasm-dist/
```

### Using Nix Shell
The `build.nix` file provides a consistent build environment:
```bash
nix-shell build.nix
# Now in a shell with all dependencies
cargo build --release
```

## Contributing
See `docs/DEVELOPMENT.md` for development setup and guidelines.