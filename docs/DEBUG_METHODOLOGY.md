# Debug Methodology for SLVSX

This document captures debugging methodologies and techniques that have proven effective when working with the SLVSX codebase, particularly when dealing with FFI issues and SolveSpace integration.

## Quick Reference

| Issue Type | First Steps |
|------------|-------------|
| SIGABRT/Assertion failure | Add debug output to C wrapper, check entity types |
| "Handle isn't unique" | Check for ID collisions, run with `RUST_TEST_THREADS=1` |
| Constraint not working | Verify entity types match constraint requirements |
| CI failures | Reproduce locally first, then investigate |
| Tests pass locally but fail in CI | Check for C library rebuild issues |

## Core Principles

### 1. Reproduce Locally First

Never attempt to fix CI failures without local reproduction:

```bash
# Run the specific failing test
nix-shell --run "cargo test -p slvsx-core test_name -- --nocapture"

# Use single-threaded to avoid race conditions
nix-shell --run "export RUST_TEST_THREADS=1 && cargo test --workspace"
```

### 2. The C Library Rebuild Problem

**Critical**: Changes to `ffi/real_slvs_wrapper.c` may not be picked up automatically.

```bash
# Force rebuild of the C library
cd /path/to/slvsx-cli
touch ffi/real_slvs_wrapper.c
cargo clean -p slvsx-core
cargo build -p slvsx-core
```

This is necessary because:
- The Rust build system doesn't always detect C file changes
- The static library may be cached
- Incremental compilation may skip the C compilation step

### 3. Add Debug Output to C Wrapper

When debugging FFI issues, add fprintf statements to understand what's happening:

```c
// In real_slvs_solve():
fprintf(stderr, "DEBUG: Solving with %d constraints\n", s->sys.constraints);
for (int i = 0; i < s->sys.constraints; i++) {
    fprintf(stderr, "DEBUG: Constraint %d: type=%d, wrkpl=%u, ptA=%u, ptB=%u, entA=%u, entB=%u\n",
        i, s->sys.constraint[i].type, s->sys.constraint[i].wrkpl,
        s->sys.constraint[i].ptA, s->sys.constraint[i].ptB,
        s->sys.constraint[i].entityA, s->sys.constraint[i].entityB);
}
fprintf(stderr, "DEBUG: Entities: %d\n", s->sys.entities);
for (int i = 0; i < s->sys.entities; i++) {
    fprintf(stderr, "DEBUG: Entity %d: h=%u, type=%d, wrkpl=%u\n",
        i, s->sys.entity[i].h, s->sys.entity[i].type, s->sys.entity[i].wrkpl);
}
fflush(stderr);  // Important! Ensure output before potential crash
```

**Remember to remove debug output after fixing the issue.**

## Debugging Specific Issues

### SIGABRT / Assertion Failures

SolveSpace uses assertions to validate preconditions. Common assertions:

| Assertion | Meaning | Solution |
|-----------|---------|----------|
| `workplane != Entity::FREE_IN_3D` | 2D constraint needs a workplane | Pass valid workplane ID |
| `Unexpected entity types for X` | Wrong entity type for constraint | Check SolveSpace docs for expected types |
| `Handle isn't unique` | Duplicate entity IDs | Check ID assignment logic |
| `Cannot find handle` | Referenced entity doesn't exist | Verify entity creation order |

**To find what the assertion expects**, look at SolveSpace source:

```bash
# Find the constraint implementation
grep -n "case Type::CONSTRAINT_NAME" libslvs-static/src/constrainteq.cpp
sed -n 'START,ENDp' libslvs-static/src/constrainteq.cpp
```

### Entity ID Mapping

Our ID mapping scheme:

| Entity Type | ID Formula | Example |
|-------------|------------|---------|
| Points | `1000 + id` | ID 3 → 1003 |
| Normals | `2000 + id` (for arcs) or `3000 + id` (for arcs) | |
| Distance entities | `4000 + id` | |
| Workplanes | `1000 + id` | ID 2 → 1002 |
| Circle entities | `1000 + id` with support entities at offsets | |
| Constraints | `10000 + id` | ID 100 → 10100 |

**Trace through entity creation:**

```rust
// In solver.rs, next_id starts at 1
let mut next_id = 1;

// Plane creates: origin point (next_id), workplane (next_id + 1)
// entity_id_map stores workplane ID, not origin point ID

// Point2D uses workplane_id from entity_id_map
```

### Constraint Entity Type Requirements

From SolveSpace source, here are key constraints and their entity requirements:

```
CURVE_CURVE_TANGENT:
  - entityA, entityB must be: ARC_OF_CIRCLE or CUBIC (NOT CIRCLE, NOT LINE)

ARC_LINE_TANGENT:
  - entityA = arc, entityB = line

CUBIC_LINE_TANGENT:
  - entityA = cubic, entityB = line

HORIZONTAL / VERTICAL:
  - Requires workplane != FREE_IN_3D
  - If entityA set: extracts point[0] and point[1] from line
  - Otherwise uses ptA and ptB directly

SYMMETRIC_HORIZ / SYMMETRIC_VERT:
  - Requires workplane != FREE_IN_3D
  - Uses ptA and ptB (2D points in workplane)
  - SYMMETRIC_HORIZ: equal Y, opposite X
  - SYMMETRIC_VERT: equal X, opposite Y
```

### Understanding Test Failures

**When a constraint "doesn't work":**

1. **Check if entities exist:**
   - Print entity IDs and types
   - Verify references resolve correctly

2. **Check constraint parameters:**
   - Some constraints use `ptA/ptB` (points)
   - Some use `entityA/entityB` (lines, arcs, etc.)
   - Some use both

3. **Check workplane:**
   - 2D constraints need a valid workplane
   - 3D constraints should use `SLVS_FREE_IN_3D` (0)

4. **Check solve result:**
   - `SLVS_RESULT_OKAY` = 0
   - `SLVS_RESULT_INCONSISTENT` = 1 (overconstrained)
   - `SLVS_RESULT_DIDNT_CONVERGE` = 2
   - `SLVS_RESULT_TOO_MANY_UNKNOWNS` = 3

### Race Conditions

If tests pass individually but fail together:

```bash
# Run single-threaded
export RUST_TEST_THREADS=1
cargo test --workspace

# Or for a specific test file
cargo test -p slvsx-core --test constraint_tests -- --test-threads=1
```

## Workflow: Fixing a CI Failure

1. **Get the exact error** from CI logs
2. **Reproduce locally:**
   ```bash
   nix-shell --run "cargo test test_name -- --nocapture"
   ```
3. **If it passes locally, force C rebuild:**
   ```bash
   touch ffi/real_slvs_wrapper.c
   cargo clean -p slvsx-core
   cargo test test_name -- --nocapture
   ```
4. **Add debug output** if needed
5. **Check SolveSpace source** for constraint requirements
6. **Fix and verify:**
   ```bash
   # Run the specific test
   cargo test test_name -- --nocapture
   
   # Run all tests in the file
   cargo test -p slvsx-core --test test_file
   
   # Run all tests
   export RUST_TEST_THREADS=1 && cargo test --workspace
   ```
7. **Remove debug output**
8. **Commit and push**

## Reading SolveSpace Source

Key files in `libslvs-static/src/`:

| File | Contents |
|------|----------|
| `constrainteq.cpp` | Constraint equation generation |
| `entity.cpp` | Entity type definitions |
| `system.cpp` | Solver implementation |
| `sketch.h` | Data structure definitions |

Key files in `libslvs-static/include/`:

| File | Contents |
|------|----------|
| `slvs.h` | Public API, entity/constraint types, Slvs_Make* functions |

**Example: Understanding a constraint**

```bash
# Find constraint type constant
grep "SLVS_C_HORIZONTAL" libslvs-static/include/slvs.h

# Find constraint implementation
grep -n "case Type::HORIZONTAL" libslvs-static/src/constrainteq.cpp

# Read the implementation
sed -n '837,860p' libslvs-static/src/constrainteq.cpp
```

## Common Pitfalls

1. **Forgetting to flush stderr** - Debug output may not appear before crash
2. **Not rebuilding C library** - Changes not picked up
3. **Running tests in parallel** - Race conditions in FFI
4. **Using wrong entity type** - e.g., Circle vs Arc for tangent
5. **Missing workplane** - 2D constraints crash without workplane
6. **Wrong parameter order** - Check Slvs_MakeConstraint signature
7. **Entity ID offset mismatch** - Rust ID vs internal SolveSpace ID

## Tips

- **Start with FFI binding tests** - They isolate the C wrapper
- **Check entity types with debug output** - Verify before blaming constraint logic
- **Read the assertion message** - SolveSpace's messages are descriptive
- **Use `sed -n` for targeted reading** - Don't scroll through huge files
- **Commit working state often** - Easy to bisect if something breaks

