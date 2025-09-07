# Recovery Document - SLVSX Development Status

## Current Work Context

### What Was Being Done
Working on fixing the example JSON files to match the schema and generating missing SVGs. The examples are failing validation because the JSON constraint format in the examples doesn't match what the Rust structs expect.

### Key Issues Found

1. **Constraint Schema Mismatches**:
   - `parallel` constraint: Examples use `"entities": ["A", "B"]` but schema expects `"a": "A", "b": "B"`
   - `perpendicular` constraint: Same issue - needs `"a"` and `"b"` fields
   - `horizontal` constraint: Examples use `"entity": "AB"` but schema expects `"a": "AB"`
   - `vertical` constraint: Same as horizontal
   - `equal_length` constraint: Likely uses wrong format
   - `coincident` constraint: Missing `"at"` field
   - `symmetric` constraint: Not implemented in the schema at all

2. **Example File Issues**:
   - Many example markdown files contain multiple JSON blocks
   - The test script (`test-examples.sh`) only extracts the first JSON block
   - Some examples have narrative JSON (not actual valid JSON)
   - Introduction example (00_introduction.md) has invalid JSON

3. **Current Test Results**:
   - 21 total examples tested
   - Only 3 passing: 01_basic_distance, 02_triangle, 04_3d_tetrahedron
   - 18 failing due to various schema mismatches

### Files Already Modified
1. `crates/core/build.rs` - Changed to static linking of libslvs
2. `build.nix` - Updated to use rustup for newer Rust
3. `.github/workflows/release.yml` - Updated for static builds
4. `README.md` - Updated build instructions
5. `BUILD.md` - Created comprehensive build documentation
6. `examples/05_parallel_perpendicular.md` - Partially fixed (parallel, perpendicular, horizontal, vertical constraints)

### Static Linking Success
✅ Successfully implemented static linking for libslvs. The binary now works without DYLD_LIBRARY_PATH.

## Detailed TODO List

### 1. Fix Example JSON Schema Issues [IN PROGRESS]

#### a. Fix constraint field names in all examples
For each example file, need to update:
- `parallel`: Change `"entities": [a, b]` → `"a": a, "b": b`
- `perpendicular`: Change `"entities": [a, b]` → `"a": a, "b": b`  
- `horizontal`: Change `"entity": x` → `"a": x`
- `vertical`: Change `"entity": x` → `"a": x`
- `equal_length`: Check format and fix (likely needs `"a"` and `"b"`)
- `coincident`: Ensure has `"at"` and `"of"` fields
- `angle`: Check if uses correct `"between"` and `"value"` format

Files to fix:
- ✅ 05_parallel_perpendicular.md (partially done)
- ❌ 00_introduction.md (has invalid JSON)
- ❌ 01_first_point.md (multiple JSON blocks)
- ❌ 02_distance_constraint.md
- ❌ 03_constraints.md
- ❌ 03_lines_and_length.md
- ❌ 04_triangle.md
- ❌ 06_circles.md
- ❌ 07_point_on_line.md
- ❌ 08_angles.md
- ❌ 09_coincident.md
- ❌ 10_equal_length.md
- ❌ 11_symmetric.md (symmetric constraint not supported - need to remove or implement)
- ❌ 12_3d_basics.md
- ❌ 13_horizontal_vertical.md
- ❌ 14_point_on_circle.md
- ❌ 15_equal_radius.md
- ❌ 17_complex_mechanisms.md

#### b. Handle multiple JSON blocks
Some examples have multiple JSON snippets showing progression. Options:
1. Create separate .json files for each variant
2. Update test script to handle multiple blocks
3. Keep only the final/complete JSON in each example

#### c. Remove or implement unsupported constraints
- `symmetric` constraint appears in examples but not in schema
- Either implement it or remove from examples

### 2. Generate Missing SVGs [PENDING]
Once JSON is fixed, run:
```bash
./test-examples.sh  # This will auto-generate missing SVGs
```

Currently 12 SVGs exist, but we have 21 examples. Need to generate 9 more.

### 3. Update Examples README [PENDING]
The `examples/README.md` references old content including gear examples that were removed. Need to:
- Remove all gear-related sections
- Update to match actual examples 00-17
- Fix broken SVG paths (currently points to `outputs/` but SVGs are in same directory)
- Remove references to `testdata/` examples that don't exist

### 4. Add Example Testing to CI [PENDING]
- Add `test-examples.sh` to the CI workflow
- Ensure all examples pass before allowing commits
- Could be added to `run-ci-local.sh`

### 5. Commit and Push Fixed Examples [PENDING]
Once all examples are fixed:
```bash
git add examples/
git commit -m "Fix example JSON to match constraint schema"
git push
```

### 6. Run CI and Verify [PENDING]
```bash
./run-ci-local.sh
```

## Quick Commands to Resume

```bash
# Test current state of examples
./test-examples.sh

# Test a specific example
awk '/```json/{flag=1; next} /```/{flag=0} flag' examples/05_parallel_perpendicular.md | ./target/release/slvsx solve -

# Check constraint schema
grep -A5 "enum Constraint" crates/core/src/ir.rs

# Run CI
./run-ci-local.sh
```

## Known Working Examples
- 01_basic_distance.json ✅
- 02_triangle.json ✅  
- 04_3d_tetrahedron.json ✅

## Schema Reference (from ir.rs)
```rust
Constraint enum variants:
- Coincident { at: String, of: Vec<String> }
- Distance { between: Vec<String>, value: ExprOrNumber }
- Angle { between: Vec<String>, value: ExprOrNumber }
- Perpendicular { a: String, b: String }
- Parallel { a: String, b: String }
- Horizontal { a: String }
- Vertical { a: String }
- EqualLength { a: String, b: String }
- EqualRadius { a: String, b: String }
- Tangent { a: String, b: String }
- PointOnLine { point: String, line: String }
- PointOnCircle { point: String, circle: String }
- Fixed { entity: String }
```

## Next Steps
1. Continue fixing constraint field names in remaining examples
2. Test each fix with the solver
3. Generate SVGs for all examples
4. Update documentation
5. Commit and run CI