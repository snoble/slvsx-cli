# Test Coverage Policy

## Philosophy

We practice **Test-Driven Development (TDD)**. All production code should be motivated by a failing test first:

1. **Red** - Write a failing test that defines what you want
2. **Green** - Write the minimum code to make it pass
3. **Refactor** - Clean up while keeping tests green

This means 100% coverage isn't just a metric—it's a natural outcome of how we write code.

## Goal

**100% test coverage** for all production code.

Every line of code should either be tested or have a documented, provable reason for exclusion. If you're writing code without a test driving it, ask yourself why.

## Exceptions

Coverage exceptions are allowed only in these cases:

### 1. Provably Unreachable Code

Code that cannot be executed under any circumstances. This requires **proof**, not just a claim.

**Valid examples:**
- Match arms required by the compiler but logically impossible given type constraints
- Error handling for conditions prevented by type system guarantees
- Default branches in exhaustive matches over private enums

**How to mark:**
```rust
// COVERAGE: unreachable - this arm handles FooVariant but we only construct
// BarVariant in this module (see foo.rs:42). The enum is private so external
// code cannot create FooVariant.
_ => unreachable!()
```

**Not acceptable:**
- "This shouldn't happen"
- "I don't think this code path is used"
- "This is just defensive coding"

If you can't prove it's unreachable, write a test for it.

### 2. Generated Code

Files that are auto-generated and not manually maintained:

- FFI bindings (`ffi/**/*`)
- Schema files (`schema/**/*`)
- Build scripts (`**/build.rs`)

These are excluded in `codecov.yml`.

### 3. Platform-Specific Code

Code that only runs on specific platforms may be excluded from coverage on other platforms, but must be tested on its target platform.

## Marking Exclusions

For inline exclusions, use comments that explain **why** the code is unreachable:

```rust
// COVERAGE: unreachable - <explanation with proof>
```

Do **not** use generic ignore comments without explanation.

## Running Coverage Locally

### Prerequisites

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov
```

### Commands

```bash
# Run tests with coverage report
cargo llvm-cov --all-features --workspace

# Generate HTML report
cargo llvm-cov --all-features --workspace --html
open target/llvm-cov/html/index.html

# Generate lcov format (for IDE integration)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

### Single-Threaded Execution

Due to libslvs not being thread-safe, tests must run single-threaded:

```bash
RUST_TEST_THREADS=1 cargo llvm-cov --all-features --workspace
```

## CI Integration

Coverage is collected on every PR and push to main:

1. Tests run with `cargo llvm-cov` on both Ubuntu and macOS
2. Results upload to [Codecov](https://codecov.io/gh/snoble/slvsx-cli) using `CODECOV_TOKEN` secret
3. PRs automatically show coverage diff in comments (always)
4. PRs that decrease coverage below 100% will fail CI checks
5. Coverage status is shown in PR checks and on the Codecov dashboard
6. Each OS (Ubuntu/macOS) has separate coverage flags for comparison

## Adding Tests for Uncovered Code

When you find uncovered code:

1. **Determine if it's reachable** - Can you write a test that executes it?
2. **If reachable** - Write the test
3. **If provably unreachable** - Add a `COVERAGE: unreachable` comment with proof
4. **If uncertain** - It's reachable. Write the test.

## Questions?

If you're unsure whether code qualifies for an exception, err on the side of writing a test. Tests are always valuable; exceptions require justification.
