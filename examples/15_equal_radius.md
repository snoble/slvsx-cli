# Example 15: Equal Radius Circles

**[← Point on Circle](14_point_on_circle.md)** | **[Next: Mesh Constraint →](16_mesh.md)**

## The Story

In many designs, you need multiple circles of the same size - think of gear blanks before teeth are cut, bearing races, or decorative patterns. The equal radius constraint ensures circles maintain identical diameters without specifying the exact size.

Let's create a triangular arrangement of equal-sized circles.

## The Entities

We'll create:
1. Three circles at different positions
2. Constraints to make them all the same size

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "c1_center",
      "at": [30, 50, 0]
    },
    {
      "type": "point",
      "id": "c2_center",
      "at": [70, 50, 0]
    },
    {
      "type": "point",
      "id": "c3_center",
      "at": [50, 80, 0]
    },
    {
      "type": "circle",
      "id": "c1",
      "center": [30, 50, 0],
      "diameter": 30
    },
    {
      "type": "circle",
      "id": "c2",
      "center": [70, 50, 0],
      "diameter": 30
    },
    {
      "type": "circle",
      "id": "c3",
      "center": [50, 80, 0],
      "diameter": 30
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "c1_center"
    },
    {
      "type": "fixed",
      "entity": "c2_center"
    },
    {
      "type": "fixed",
      "entity": "c3_center"
    },
    {
      "type": "equal_radius",
      "a": "c1",
      "b": "c2"
    },
    {
      "type": "equal_radius",
      "a": "c2",
      "b": "c3"
    }
  ]
}
```

## Understanding the Code

- **`equal_radius`**: Forces two circles to have the same radius
- **Transitive property**: c1 = c2 and c2 = c3 means c1 = c3
- **No absolute size**: The solver finds a radius that satisfies all constraints

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

All three circles now have exactly the same radius!

## Design Applications

- **Gear trains**: Multiple gears of same pitch diameter
- **Bearing assemblies**: Matching bearing sizes
- **Hole patterns**: Uniform bolt holes
- **Decorative patterns**: Consistent circular elements

## Combining with Other Constraints

Equal radius works well with:
- **Distance constraints**: Set center-to-center spacing
- **Tangent constraints**: Create gear-like arrangements
- **Point on circle**: For planetary motion

## Key Takeaway

Equal radius constraints maintain size relationships between circles without fixing absolute dimensions. This is perfect for scalable designs where proportions matter more than exact measurements.

**[Next: Mesh Constraint →](16_mesh.md)**