{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Use rustup for managing Rust versions
    rustup
    
    # For code coverage and security
    cargo-tarpaulin
    cargo-audit
    
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

  # Set up environment variables
  RUST_BACKTRACE = "1";
  
  shellHook = ''
    echo "SLVSX Development Environment"
    echo "=============================="
    
    # Ensure stable Rust is installed
    if ! rustup toolchain list | grep -q "stable"; then
      echo "Installing stable Rust toolchain..."
      rustup toolchain install stable
    fi
    rustup default stable
    
    echo "Building libslvs..."
    
    # Build libslvs if not already built
    if [ ! -f libslvs/SolveSpaceLib/build/libslvs.a ]; then
      mkdir -p libslvs/SolveSpaceLib/build
      cd libslvs/SolveSpaceLib/build
      
      # Configure with proper paths for Nix environment
      cmake .. -DCMAKE_BUILD_TYPE=Release || echo "CMake configuration failed, continuing anyway"
      
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