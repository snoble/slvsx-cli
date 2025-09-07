{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain with WASM support
    rustup
    cargo-tarpaulin  # For code coverage
    cargo-audit      # Security auditing
    
    # WASM tools
    wasm-pack
    wasm-bindgen-cli
    nodejs_20
    emscripten  # For compiling libslvs to WASM
    
    # Build tools
    cmake
    gnumake
    pkg-config
    
    # C++ compiler
    gcc
    
    # Required libraries for building libslvs
    zlib
    
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
    
    # Setup Rust toolchain with WASM target
    export RUSTUP_HOME="$PWD/.rustup"
    export CARGO_HOME="$PWD/.cargo"
    export PATH="$CARGO_HOME/bin:$PATH"
    
    if [ ! -f "$CARGO_HOME/bin/rustc" ]; then
      echo "Installing Rust toolchain..."
      rustup default stable
      rustup target add wasm32-unknown-unknown
    fi
    
    echo "Building libslvs..."
    
    # Build libslvs if not already built
    if [ ! -f libslvs/SolveSpaceLib/build/libslvs.a ]; then
      mkdir -p libslvs/SolveSpaceLib/build
      cd libslvs/SolveSpaceLib/build
      
      # Configure with proper paths for Nix environment
      cmake .. -DCMAKE_BUILD_TYPE=Release
      
      make -j$(nproc) || echo "libslvs build failed, continuing anyway"
      cd ../../..
    fi
    
    export LIBSLVS_DIR="$PWD/libslvs/SolveSpaceLib/build"
    export LD_LIBRARY_PATH="$LIBSLVS_DIR:$LD_LIBRARY_PATH"
    export DYLD_LIBRARY_PATH="$LIBSLVS_DIR:$DYLD_LIBRARY_PATH"
    
    echo "Environment ready!"
    echo ""
    echo "Quick commands:"
    echo "  cargo build --release           # Build the CLI"
    echo "  cargo test                      # Run tests"
    echo "  cargo tarpaulin                 # Generate coverage report"
    echo "  wasm-pack build                 # Build WASM module"
    echo "  ./target/release/slvsx          # Run the CLI"
  '';
}