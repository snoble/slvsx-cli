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
- `05_parallel_perpendicular.json` - Parallel/perpendicular lines
- `07_point_on_line.json` - Point on line constraint
- `09_coincident.json` - Coincident points

### Examples With Missing Features

These examples require constraints that are not yet implemented:

- `08_angles.json` - Requires **Angle** constraint
- `10_equal_length.json` - Requires **EqualLength** constraint
- `11_symmetric.json` - Requires **Symmetric** constraint
- `13_horizontal_vertical.json` - Requires **Horizontal/Vertical** constraints
- `14_point_on_circle.json` - Requires **PointOnCircle** constraint
- `15_equal_radius.json` - Requires **EqualRadius** constraint
- `17_four_bar_linkage.json` - Requires **Angle** constraint
- `18_simple_rectangle.json` - Requires **Horizontal/Vertical** constraints
- `19_parametric_square.json` - Requires **Horizontal/Vertical/EqualLength** constraints
- `20_slider_crank.json` - Requires **Horizontal/Angle** constraints

These examples will fail validation with "not yet implemented" errors until the required constraints are implemented.

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

