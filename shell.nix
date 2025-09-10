{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    cargo
    rustc
    
    # Build tools
    cmake
    gnumake
    pkg-config
    
    # C++ compiler
    gcc
  ];

  shellHook = ''
    export SLVS_LIB_DIR="$PWD/libslvs-static/build"
    export SLVS_STATIC=1
  '';
}