{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    cargo-tarpaulin  # For code coverage
    cargo-audit      # Security auditing
    
    # Cross-compilation support
    pkgsCross.aarch64-multiplatform.stdenv.cc
    
    # Build tools
    cmake
    gnumake
    pkg-config
    
    # C++ compiler
    gcc
    
    # Development tools
    git
    jq
    
    # Python for test scripts
    python3
    python3Packages.numpy
    
    # Documentation
    mdbook
  ];

  shellHook = ''
    echo "SLVSX Development Environment"
    echo "=============================="
    echo "Building libslvs..."
    
    # Build libslvs if not already built
    if [ ! -f libslvs/SolveSpaceLib/build/libslvs.a ]; then
      mkdir -p libslvs/SolveSpaceLib/build
      cd libslvs/SolveSpaceLib/build
      cmake .. -DCMAKE_BUILD_TYPE=Release
      make -j$(nproc)
      cd ../../..
    fi
    
    export LIBSLVS_DIR="$PWD/libslvs/SolveSpaceLib/build"
    export LD_LIBRARY_PATH="$LIBSLVS_DIR:$LD_LIBRARY_PATH"
    export DYLD_LIBRARY_PATH="$LIBSLVS_DIR:$DYLD_LIBRARY_PATH"
    
    echo "Environment ready!"
    echo ""
    echo "Quick commands:"
    echo "  cargo build --release    # Build the CLI"
    echo "  cargo test              # Run tests"
    echo "  cargo tarpaulin         # Generate coverage report"
    echo "  ./target/release/slvsx  # Run the CLI"
  '';
}