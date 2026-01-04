# Preserve Flag Example

This example demonstrates the `preserve` flag feature, which allows you to mark entities as "preserved" so the solver minimizes changes to them during solving.

## What This Example Shows

- **Preserved points**: `base_corner` and `base_corner2` are marked with `"preserve": true`
- **Adjustable point**: `adjustable_point` is not preserved, so it can move freely
- **Iterative refinement**: When constraints change, preserved points try to stay fixed while other points adjust

## How It Works

The `preserve` flag marks an entity's parameters as "dragged" in SolveSpace. This tells the solver to minimize changes to those parameters when solving, preferring to adjust other entities instead.

This is useful for:
- **Iterative design**: Build up a design step-by-step, preserving established features
- **Constraint exploration**: Try different constraint values while keeping key aspects stable
- **Staged solving**: Lock earlier stages before moving to the next

## Usage

```bash
slvsx solve examples/23_preserve_flag.json
```

## Next Steps

- Try changing the distance constraints and see how preserved points stay more stable
- See `examples/24_where_dragged.json` for absolute locking with WHERE_DRAGGED constraint
- See `examples/25_iterative_refinement.json` for a more complex iterative design example

