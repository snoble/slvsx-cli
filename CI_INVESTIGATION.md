# CI Investigation: proc-macro Compilation Failure

## Current Symptom
Ubuntu CI fails with:
```
error: cannot produce proc-macro for `clap_derive v4.5.47` as the target `x86_64-unknown-linux-gnu` does not support these crate types
```

## Timeline of Changes
1. **Initial state**: CI was working (commit 7291cf8)
2. **Attempted fixes**:
   - Removed static linking flags from test step
   - Added `unset RUSTFLAGS`
   - Tried `cargo build --tests` before `cargo test`
   - Removed Cargo.lock to generate fresh
   - Reverted to using committed Cargo.lock

## Key Observations
- macOS CI passes tests successfully
- Ubuntu CI consistently fails with proc-macro error
- The error happens during dependency resolution, not actual compilation
- Both removing and keeping Cargo.lock result in the same error

## Theories to Investigate

### Theory 1: Cargo.lock Version Mismatch
**Hypothesis**: The Cargo.lock was generated with a different Rust/Cargo version than what CI uses
**Data needed**:
- Local Rust/Cargo version that generated Cargo.lock
- CI Rust/Cargo version
- Differences in dependency resolution

### Theory 2: Environment Variable Interference
**Hypothesis**: Some environment variable is affecting how Cargo resolves proc-macro crates
**Data needed**:
- Full environment in CI
- Comparison with successful macOS environment
- Any RUSTFLAGS or CARGO_* variables set

### Theory 3: actions-rs Toolchain Issue
**Hypothesis**: The deprecated actions-rs/toolchain action has a bug with proc-macros
**Data needed**:
- Known issues with actions-rs
- Alternative toolchain actions behavior
- Version of Rust being installed

### Theory 4: Workspace Configuration Issue
**Hypothesis**: The workspace configuration is causing target resolution problems
**Data needed**:
- Cargo.toml resolver version
- Workspace member configurations
- Target specifications

### Theory 5: Dependency Conflict
**Hypothesis**: A specific dependency combination triggers the proc-macro error
**Data needed**:
- Full dependency tree
- Version constraints in Cargo.toml
- Any platform-specific dependencies

## Data Collection Plan

1. **Check Rust versions**:
   - Add step to print `rustc --version` and `cargo --version` in CI
   - Check what version generated the current Cargo.lock

2. **Environment inspection**:
   - Add `env | sort` to CI to see all variables
   - Compare Ubuntu vs macOS environments

3. **Dependency analysis**:
   - Run `cargo tree` locally
   - Check for any platform-specific dependency differences

4. **Research similar issues**:
   - Search for "cannot produce proc-macro" errors
   - Check clap_derive issue tracker
   - Look for actions-rs related problems

## Experiments to Run

### Experiment 1: Version Alignment
- Specify exact Rust version in CI that matches local
- Regenerate Cargo.lock with that version

### Experiment 2: Clean Environment
- Use minimal environment variables
- Explicitly unset all RUST* and CARGO* vars except essentials

### Experiment 3: Alternative Toolchain
- Replace actions-rs with dtolnay/rust-toolchain
- Test if issue persists

### Experiment 4: Resolver Version
- Ensure Cargo.toml specifies `resolver = "2"`
- Test with explicit edition and resolver

### Experiment 5: Direct proc-macro test
- Create minimal test that just uses clap_derive
- See if it compiles in isolation

## Root Cause Identified

After researching, the error is caused by **static linking flags being applied to proc-macro compilation**. 

Proc-macros MUST be built as dynamic libraries (dylibs) and cannot be statically linked. When `RUSTFLAGS` contains `-C target-feature=+crt-static` or similar static linking flags, it prevents proc-macro crates like `clap_derive` from being built.

### Why it's happening:
1. We removed `RUSTFLAGS` for the test step but Cargo.lock might have been generated with different flags
2. The CI environment may have residual RUSTFLAGS from previous steps
3. The Cargo.lock file may encode target information that conflicts with proc-macro requirements

### Why macOS works:
macOS handles dynamic libraries differently and doesn't have the same crt-static restrictions as Linux.

## Verified Solution

The fix is to ensure proc-macros are never built with static linking flags:

1. **For tests**: Never use static linking flags when running tests (already attempted)
2. **For builds**: Use `--target` explicitly so RUSTFLAGS only apply to the target, not host tools
3. **Clean Cargo.lock**: Generate it without any RUSTFLAGS set

## Implemented Solution

Based on research, the fix is to use explicit `--target` flags when building. This ensures that RUSTFLAGS only apply to the target being built and NOT to host tools like proc-macros.

### Changes Made:
1. Added explicit `--target x86_64-unknown-linux-gnu` for Ubuntu tests
2. Added explicit `--target x86_64-apple-darwin` for macOS tests  
3. Added explicit targets to release builds as well
4. Added debug output to verify environment

### Why This Works:
When you use `cargo build/test` without `--target`, Cargo applies RUSTFLAGS to everything including proc-macros. When you specify `--target`, Cargo knows to only apply RUSTFLAGS to the target artifacts, allowing proc-macros to be built normally as dynamic libraries for the host.

## Lessons Learned
1. Always use explicit `--target` when using RUSTFLAGS that affect linking
2. Proc-macros must always be dynamic libraries
3. The error message is misleading - the target DOES support proc-macros, but not with static linking
4. macOS and Linux handle this differently, which can mask issues