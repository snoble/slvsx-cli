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
    
    echo "Building libslvs-static..."
    
    # Build libslvs-static if not already built
    if [ ! -f libslvs-static/build/libslvs-combined.a ]; then
      mkdir -p libslvs-static/build
      cd libslvs-static/build
      
      # Configure with CMake
      cmake .. -DCMAKE_BUILD_TYPE=Release || echo "CMake configuration failed, continuing anyway"
      
      # Build the combined static library
      make -j$(nproc) || echo "libslvs-static build failed, continuing anyway"
      cd ../..
    fi
    
    # Set environment variables for static build
    export SLVS_LIB_DIR="$PWD/libslvs-static/build"
    export SLVS_STATIC=1
    export SLVS_USE_FORK=1
    
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