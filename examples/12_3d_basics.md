# Example 12: 3D Basics

**[← Symmetric Constraints](https://github.com/snoble/slvsx-cli/blob/main/examples/11_symmetric.md)** | **[Next: Horizontal & Vertical →](https://github.com/snoble/slvsx-cli/blob/main/examples/13_horizontal_vertical.md)**

## The Story

Until now, we've been working in 2D, setting all Z coordinates to 0. But the real world is three-dimensional! SLVSX fully supports 3D constraints, opening up possibilities for spatial mechanisms, frameworks, and assemblies.

Let's create a simple 3D coordinate system with a diagonal in space.

## The Entities

We'll create:
1. Points along each axis (X, Y, Z)
2. A point floating in 3D space
3. Lines connecting them to visualize the 3D structure

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
      "id": "x_axis",
      "at": [100, 0, 0]
    },
    {
      "type": "point",
      "id": "y_axis",
      "at": [0, 100, 0]
    },
    {
      "type": "point",
      "id": "z_axis",
      "at": [0, 0, 100]
    },
    {
      "type": "point",
      "id": "space_point",
      "at": [50, 50, 50]
    },
    {
      "type": "line",
      "id": "x_line",
      "p1": "origin",
      "p2": "x_axis"
    },
    {
      "type": "line",
      "id": "y_line",
      "p1": "origin",
      "p2": "y_axis"
    },
    {
      "type": "line",
      "id": "z_line",
      "p1": "origin",
      "p2": "z_axis"
    },
    {
      "type": "line",
      "id": "diagonal",
      "p1": "origin",
      "p2": "space_point"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "origin"
    },
    {
      "type": "fixed",
      "entity": "x_axis"
    },
    {
      "type": "fixed",
      "entity": "y_axis"
    },
    {
      "type": "fixed",
      "entity": "z_axis"
    },
    {
      "type": "distance",
      "between": ["origin", "space_point"],
      "value": 86.6
    }
  ]
}
```

## Understanding the Code

- **3D coordinates**: Points now have non-zero Z values
- **Spatial distance**: The diagonal length is √(50² + 50² + 50²) ≈ 86.6mm
- **Visualization challenge**: 3D in 2D requires projection

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

The solver confirms our 3D structure is fully constrained!

## 3D Applications

- **Robotics**: Joint positions in space
- **Architecture**: Structural nodes and beams
- **Mechanical**: Shaft alignments, spatial linkages
- **Manufacturing**: CNC tool paths, 3D printing

## Working in 3D

Key concepts for 3D constraint solving:
1. **Degrees of freedom**: Points have 3 DOF (X, Y, Z)
2. **Rotations**: Objects can rotate around all three axes
3. **Workplanes**: Define 2D sketches in 3D space
4. **Distance**: Works in 3D just like 2D

## Visual Output

![3D Basics](https://raw.githubusercontent.com/snoble/slvsx-cli/main/examples/12_3d_basics.svg)

## Key Takeaway

3D constraints work exactly like 2D ones - just add Z coordinates! The solver handles the third dimension naturally, making it easy to create spatial mechanisms and structures.

**[Next: Horizontal & Vertical →](https://github.com/snoble/slvsx-cli/blob/main/examples/13_horizontal_vertical.md)**