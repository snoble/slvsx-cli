# Parametric Square

A SolveSpace tutorial example demonstrating equal length constraints.

## Story

This example shows how to create a square using equal length constraints. All four sides are constrained to be equal, and with perpendicular corners, this guarantees a square.

## Constraints

1. Point A is fixed at origin
2. AB and CD are horizontal
3. BC and DA are vertical
4. All four sides have equal length
5. Corners are perpendicular

## Usage

```bash
# Solve with default side length
slvsx solve examples/19_parametric_square.json

# Create a larger square
slvsx solve examples/19_parametric_square.json \
  --param side_length=120

# Export visualization
slvsx export --format svg examples/19_parametric_square.json -o square.svg
```

## Missing Features

This example requires:
- **Horizontal** and **Vertical** constraints (not implemented)
- **EqualLength** constraint (not implemented)

Once these are implemented, this will be a perfect example of parametric design.

## Related Tutorials

- [SolveSpace Constraints Tutorial](https://solvespace.com/tutorial.pl)
- Parametric design principles

