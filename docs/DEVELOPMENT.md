# Development Guide

This guide covers setting up a development environment and contributing to slvsx.

## Prerequisites

- **Rust** 1.75 or later (`rustup` recommended)
- **CMake** 3.10 or later
- **C++ compiler** (GCC 7+ or Clang 8+)
- **Git** with submodule support

### Platform-specific

**Ubuntu/Debian:**
```bash
sudo apt-get install cmake build-essential libpng-dev zlib1g-dev
```

**macOS:**
```bash
# CMake is usually pre-installed, or:
brew install cmake
```

## Quick Start

```bash
# Clone with submodules
git clone --recursive https://github.com/snoble/slvsx-cli.git
cd slvsx-cli

# Build everything
./build.sh

# Run tests
cargo test

# Run the CLI
./target/release/slvsx --version
```

## Project Structure

```
slvsx-cli/
├── crates/
│   ├── core/          # Core library with FFI bindings to libslvs
│   ├── cli/           # CLI application
│   └── exporters/     # Export functionality (SVG, DXF, STL)
├── ffi/               # C wrapper for libslvs
├── libslvs-static/    # Fork of SolveSpace's constraint solver
├── examples/          # Example constraint problems
├── schema/            # JSON schema definitions
└── docs/              # Documentation
```

## Building

### Using build.sh (Recommended)

```bash
./build.sh
```

This handles building libslvs-static and the Rust project.

### Manual Build

```bash
# 1. Build libslvs-static
cd libslvs-static
mkdir -p build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
cd ../..

# 2. Build slvsx
export SLVS_LIB_DIR=$PWD/libslvs-static/build
cargo build --release
```

## Testing

### Run All Tests

```bash
# Tests must run single-threaded (libslvs is not thread-safe)
RUST_TEST_THREADS=1 cargo test
```

### Run with Coverage

```bash
# Install cargo-llvm-cov if needed
cargo install cargo-llvm-cov

# Run tests with coverage
RUST_TEST_THREADS=1 cargo llvm-cov --all-features --workspace

# Generate HTML report
RUST_TEST_THREADS=1 cargo llvm-cov --all-features --workspace --html
open target/llvm-cov/html/index.html
```

See [COVERAGE.md](../COVERAGE.md) for our coverage policy.

## Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Run clippy before committing (`cargo clippy`)
- Write tests for new functionality (TDD preferred)
- Document public APIs

## Pull Request Process

1. **Create a branch** from `main`
2. **Write tests first** (TDD style)
3. **Implement the feature/fix**
4. **Ensure all tests pass**: `RUST_TEST_THREADS=1 cargo test`
5. **Check formatting**: `cargo fmt --check`
6. **Run clippy**: `cargo clippy`
7. **Push and create PR**
8. **Wait for CI** - both Ubuntu and macOS builds must pass
9. **Get review** - 1 approval required

## Common Tasks

### Adding a New Constraint Type

1. Add the constraint to `crates/core/src/ir.rs`
2. Implement FFI mapping in `crates/core/src/solver.rs`
3. Add tests in `crates/core/src/tests/`
4. Update JSON schema if needed
5. Add an example in `examples/`

### Adding an Export Format

1. Create a new module in `crates/exporters/src/`
2. Implement the `Exporter` trait
3. Register in `crates/exporters/src/lib.rs`
4. Add CLI support in `crates/cli/src/main.rs`
5. Add feature flag if dependencies are heavy

### Updating libslvs-static

The libslvs-static submodule is a fork with modifications for static building. To update:

1. Make changes in the fork repo
2. Update the submodule reference
3. Test that static builds still work

## Troubleshooting

### "Cannot find libslvs-combined.a"

Ensure libslvs-static is built:
```bash
cd libslvs-static/build
cmake .. -DCMAKE_BUILD_TYPE=Release
make
```

### Tests crash with SIGSEGV

libslvs is not thread-safe. Run tests single-threaded:
```bash
RUST_TEST_THREADS=1 cargo test
```

### Proc-macro compilation errors

Don't use global rustflags. The build.sh script handles this correctly.

## Resources

- [SolveSpace Documentation](http://solvespace.com/ref.pl)
- [libslvs API](https://github.com/solvespace/solvespace/blob/master/include/slvs.h)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
