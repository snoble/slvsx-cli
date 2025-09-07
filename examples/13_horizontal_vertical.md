# Example 13: Horizontal & Vertical Constraints

**[← 3D Basics](12_3d_basics.md)** | **[Next: Point on Circle →](14_point_on_circle.md)**

## The Story

Sometimes you need lines to be perfectly horizontal or vertical - think of building frames, grid layouts, or coordinate systems. These constraints are shortcuts for common angles (0° and 90°) that simplify your constraint definitions.

Let's create a coordinate system with guaranteed horizontal and vertical axes.

## The Entities

We'll create:
1. Lines that should be horizontal or vertical
2. A diagonal line at 45° for reference

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "origin",
      "at": [0, 0, 0]
    },
    {
      "type": "point",
      "id": "top",
      "at": [10, 100, 0]
    },
    {
      "type": "point",
      "id": "right",
      "at": [100, 10, 0]
    },
    {
      "type": "point",
      "id": "corner",
      "at": [100, 100, 0]
    },
    {
      "type": "line",
      "id": "vertical_line",
      "p1": "origin",
      "p2": "top"
    },
    {
      "type": "line",
      "id": "horizontal_line",
      "p1": "origin",
      "p2": "right"
    },
    {
      "type": "line",
      "id": "diagonal",
      "p1": "origin",
      "p2": "corner"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "origin"
    },
    {
      "type": "vertical",
      "a": "vertical_line"
    },
    {
      "type": "horizontal",
      "a": "horizontal_line"
    },
    {
      "type": "distance",
      "between": ["origin", "top"],
      "value": 100
    },
    {
      "type": "distance",
      "between": ["origin", "right"],
      "value": 100
    },
    {
      "type": "angle",
      "between": ["horizontal_line", "diagonal"],
      "a": "horizontal_line",
      "b": "diagonal",
      "value": 45
    }
  ]
}
```

## Understanding the Code

- **`horizontal`**: Forces a line to be perfectly horizontal (parallel to X-axis)
- **`vertical`**: Forces a line to be perfectly vertical (parallel to Y-axis)
- **Simpler than angles**: No need to specify 0° or 90° angles

## The Solution

```json
{
  "status": "ok",
  "diagnostics": {
    "iters": 1,
    "residual": 0.0,
    "dof": 0
  }
}
```

The solver snaps the lines to perfect horizontal and vertical orientations!

## Design Applications

- **Architectural drawings**: Wall alignments
- **Grid systems**: UI layouts, graph paper
- **Mechanical parts**: Shaft alignments
- **PCB design**: Trace routing

## Comparison with Angle Constraints

These are equivalent:
- `horizontal` = angle of 0° with X-axis
- `vertical` = angle of 90° with X-axis

But horizontal/vertical constraints are:
- More readable
- Computationally simpler
- Less error-prone

## Visual Output

![Horizontal and Vertical](13_horizontal_vertical.svg)

## Key Takeaway

Horizontal and vertical constraints are convenience shortcuts that make your constraint definitions clearer and more maintainable. Use them whenever you need axis-aligned geometry.

**[Next: Point on Circle →](14_point_on_circle.md)**