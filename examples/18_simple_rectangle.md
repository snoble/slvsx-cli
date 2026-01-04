# Simple Rectangle

A basic SolveSpace tutorial example showing how to create a parametric rectangle.

## Story

This is often the first example in SolveSpace tutorials - creating a simple rectangle with:
- One fixed corner
- Horizontal and vertical sides
- Parametric width and height

## Constraints

1. Point A is fixed at origin
2. AB and CD are horizontal
3. BC and DA are vertical
4. Width and height are parametric
5. All corners are perpendicular

## Usage

```bash
# Solve with default parameters
slvsx solve examples/18_simple_rectangle.json

# Create a square (width = height)
slvsx solve examples/18_simple_rectangle.json \
  --param width=80 \
  --param height=80

# Export visualization
slvsx export --format svg examples/18_simple_rectangle.json -o rectangle.svg
```

## Missing Features

This example requires **Horizontal** and **Vertical** constraints, which are currently not implemented. Once implemented, this will be a perfect beginner tutorial example.

## Related Tutorials

- [SolveSpace Introductory Tutorial](https://solvespace.com/tutorial.pl)
- Basic geometric constraint solving concepts

