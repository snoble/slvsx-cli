# Example Validation

All examples in this directory are automatically validated as part of the test suite.

## Running Validation Locally

To validate all examples:

```bash
export SLVS_LIB_DIR=$PWD/libslvs-static/build
cargo test --test example_validation_test
```

To validate a specific example:

```bash
export SLVS_LIB_DIR=$PWD/libslvs-static/build
cargo build --release
./target/release/slvsx validate examples/17_four_bar_linkage.json
./target/release/slvsx solve examples/17_four_bar_linkage.json
```

## Expected Results

### Examples That Should Solve

These examples use only implemented constraints and should solve successfully:

- `01_first_point.json` - Fixed point
- `01_basic_distance.json` - Distance constraint
- `02_distance_constraint.json` - Distance constraint
- `02_triangle.json` - Triangle with distances
- `03_correctly_constrained.json` - Properly constrained system
- `04_3d_tetrahedron.json` - 3D tetrahedron
- `05_circles.json` - Circles with constraints
- `06_circles.json` - More circle examples
- `07_point_on_line.json` - Point on line constraint
- `09_coincident.json` - Coincident points
- `12_3d_basics.json` - 3D basics
- `18_simple_rectangle.json` - Simple rectangle (fixed to use implemented constraints)
- `19_parametric_square.json` - Parametric square (fixed to use implemented constraints)

### Examples With Missing Features

These examples require constraints that are not yet implemented:

- `05_parallel_perpendicular.json` - Requires **Parallel** and **Perpendicular** constraints
- `11_symmetric.json` - Requires **Symmetric** constraint
- `13_arcs.json` - Requires arc-related constraints
- `13_midpoint.json` - Requires **Midpoint** constraint

These examples will fail validation with "not yet implemented" errors until the required constraints are implemented.

### Examples That Need More Work

These examples currently fail but could work with constraint adjustments:

- `03_overconstrained.json` - Intentionally overconstrained (demonstrates error handling)
- `10_equal_length.json` - Underconstrained, needs more constraints  
- `17_four_bar_linkage.json` - Underconstrained without angle constraint
- `20_slider_crank.json` - Underconstrained without angle/horizontal constraints
- `13_workplanes.json` - File path issue

### Examples Fixed to Work

These examples were updated to use only implemented constraints:

- `18_simple_rectangle.json` - Removed horizontal/vertical, uses perpendicular + distances
- `19_parametric_square.json` - Removed horizontal/vertical/equal_length, uses distances

## CI Integration

The `example_validation_test` runs automatically on every PR and push to main. It ensures:

1. All example JSON files are valid JSON
2. All examples can be validated (even if they fail due to missing features)
3. Examples with implemented constraints solve successfully
4. Tutorial examples document their missing features

## Adding New Examples

When adding a new example:

1. Create the JSON file with valid schema
2. If it requires missing constraints, create a corresponding `.md` file documenting which constraints are missing
3. Run `cargo test --test example_validation_test` to verify
4. If the example should solve with current constraints, add it to the `test_solvable_examples_solve` test

