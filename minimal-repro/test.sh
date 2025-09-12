#!/bin/bash

echo "Testing proc-macro compilation with different configurations..."
echo "================================================"

echo -e "\n1. Testing WITH global rustflags (should fail on Linux):"
echo "   Config: rustflags = [\"-C\", \"target-feature=+crt-static\"]"
cargo build 2>&1 | grep -E "(error:|Compiling)" || echo "Build succeeded"

echo -e "\n2. Testing WITHOUT global rustflags (should work):"
mv .cargo/config.toml .cargo/config.toml.bak
cargo clean
cargo build 2>&1 | grep -E "(error:|Compiling)" || echo "Build succeeded"

echo -e "\n3. Testing with --target and RUSTFLAGS env (should work):"
cargo clean
export RUSTFLAGS="-C target-feature=+crt-static"
cargo build --target x86_64-unknown-linux-gnu 2>&1 | grep -E "(error:|Compiling)" || echo "Build succeeded"

echo -e "\nConclusion:"
echo "- Global rustflags break proc-macro compilation"
echo "- Using --target with RUSTFLAGS only affects the target, not proc-macros"