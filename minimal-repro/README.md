# Minimal Reproduction of proc-macro Compilation Issue

This minimal project demonstrates the proc-macro compilation error that occurs when global rustflags include static linking options.

## The Problem

When `.cargo/config.toml` contains:
```toml
[build]
rustflags = ["-C", "target-feature=+crt-static"]
```

Building fails with:
```
error: cannot produce proc-macro for `clap_derive` as the target does not support these crate types
```

## Root Cause

Proc-macros MUST be built as dynamic libraries (.so/.dylib) because they're compiler plugins that run during compilation. The `+crt-static` flag forces static linking, which is incompatible with proc-macros.

## Solutions

1. **Remove global rustflags** - Don't set them in `[build]` section
2. **Use target-specific rustflags** - Only when using `--target` explicitly
3. **Use RUSTFLAGS environment variable** - Set it only for release builds

## Test

Run `./test.sh` to see the different scenarios (requires Linux or Docker).

## Key Insight

When you use `cargo build --target x86_64-unknown-linux-gnu`, Cargo knows to apply RUSTFLAGS only to the target being built, NOT to host tools like proc-macros. Without `--target`, RUSTFLAGS apply to everything.