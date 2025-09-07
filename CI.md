# Continuous Integration

This project uses local CI to ensure quality. GitHub Actions CI is disabled due to complex build requirements.

## Setup

First time setup:
```bash
./setup-local-ci.sh
```

This configures git hooks to remind you about CI before pushing.

## Running CI

### Basic CI Run
```bash
./run-ci-local.sh
```

This will:
- Check code formatting (cargo fmt)
- Run linting (cargo clippy)
- Run all tests
- Build the release binary
- Validate all examples

Results are saved to `ci-results/` with timestamps.

### CI with GitHub Push
```bash
./run-ci-local.sh --push
```

This runs CI and then:
- Pushes results to GitHub
- Updates `ci-latest-results.md`
- Commits and pushes the results

## CI Status

The latest CI results are always available in `ci-latest-results.md`.

A GitHub workflow (`ci-status.yml`) will display the results when they're pushed.

## Running CI Before Push

The pre-push git hook will remind you to run CI. It shows:
- When CI was last run
- Whether it passed or failed
- Commands to run CI

## Disabling Reminders

If you want to disable the pre-push reminder:
```bash
git config core.hooksPath .git/hooks
```

To re-enable:
```bash
git config core.hooksPath .githooks
```

## CI Requirements

- Rust toolchain (cargo, rustc, rustfmt, clippy)
- CMake (for building libslvs)
- C++ compiler (gcc/clang)
- zlib development headers

## Troubleshooting

If CI fails locally:

1. **libslvs build issues**: 
   - Check CMake is installed
   - Check you have a C++ compiler
   - Try manually building: `cd libslvs/SolveSpaceLib/build && cmake .. && make`

2. **Rust issues**:
   - Update Rust: `rustup update`
   - Check formatting: `cargo fmt`
   - Fix clippy warnings: `cargo clippy --fix`

3. **Example validation issues**:
   - Run specific example: `./target/release/slvsx solve examples/01_first_point.json`
   - Check for JSON syntax errors

## CI Best Practices

1. Run CI before every push
2. Fix any failures immediately
3. Keep CI passing on main branch
4. Update CI script if new checks are needed