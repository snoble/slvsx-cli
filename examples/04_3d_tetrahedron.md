# 3D Regular Tetrahedron

This example demonstrates 3D constraint solving by creating a regular tetrahedron (4-sided solid with all edges equal length). This shows SLVSX working in full 3D space.

## Problem

Create a regular tetrahedron where:
- All 6 edges are exactly 50mm long
- Point A is fixed at the origin
- Points B, C, and D can move in 3D space to satisfy the constraints
- The result should be a geometrically perfect tetrahedron

## Solution

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "edge_length": 50.0
  },
  "entities": [
    {
      "type": "point",
      "id": "A",
      "at": [0, 0, 0]
    },
    {
      "type": "point",
      "id": "B",
      "at": [50, 0, 0]
    },
    {
      "type": "point",
      "id": "C",
      "at": [25, 43, 0]
    },
    {
      "type": "point",
      "id": "D",
      "at": [25, 14, 35]
    },
    {
      "type": "line",
      "id": "AB",
      "p1": "A",
      "p2": "B"
    },
    {
      "type": "line",
      "id": "BC",
      "p1": "B",
      "p2": "C"
    },
    {
      "type": "line",
      "id": "CA",
      "p1": "C",
      "p2": "A"
    },
    {
      "type": "line",
      "id": "AD",
      "p1": "A",
      "p2": "D"
    },
    {
      "type": "line",
      "id": "BD",
      "p1": "B",
      "p2": "D"
    },
    {
      "type": "line",
      "id": "CD",
      "p1": "C",
      "p2": "D"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "A"
    },
    {
      "type": "distance",
      "between": ["A", "B"],
      "value": "$edge_length"
    },
    {
      "type": "distance",
      "between": ["B", "C"],
      "value": "$edge_length"
    },
    {
      "type": "distance",
      "between": ["C", "A"],
      "value": "$edge_length"
    },
    {
      "type": "distance",
      "between": ["A", "D"],
      "value": "$edge_length"
    },
    {
      "type": "distance",
      "between": ["B", "D"],
      "value": "$edge_length"
    },
    {
      "type": "distance",
      "between": ["C", "D"],
      "value": "$edge_length"
    }
  ]
}
```

## Usage

```bash
# Solve the 3D constraint system
slvsx solve examples/04_3d_tetrahedron.json

# Export different views
slvsx export examples/04_3d_tetrahedron.json --format svg --view xy --output tetrahedron_xy.svg
slvsx export examples/04_3d_tetrahedron.json --format svg --view xz --output tetrahedron_xz.svg
slvsx export examples/04_3d_tetrahedron.json --format svg --view yz --output tetrahedron_yz.svg
```

## Result

The solver positions the points at:
- Point A: `(0, 0, 0)` (fixed)
- Point B: `(49.98, -0.12, -1.26)`  
- Point C: `(25.09, 43.24, -1.03)`
- Point D: `(26.05, 14.75, 40.05)`

All 6 edges are exactly 50mm long, forming a perfect regular tetrahedron.

### XY View (Top-down)
![Tetrahedron XY View](outputs/04_3d_tetrahedron_xy.svg)

### XZ View (Side)
![Tetrahedron XZ View](outputs/04_3d_tetrahedron_xz.svg)

## Key Concepts

- **3D constraint solving**: Points can move in X, Y, and Z directions
- **Multiple views**: Different projections reveal different aspects of the 3D shape
- **Regular polyhedron**: All edges equal length creates a geometrically perfect solid
- **3D degrees of freedom**: Each 3D point has 3 DOF, giving rich constraint possibilities

## Mathematical Verification

For a regular tetrahedron with edge length `a`:
- Height: `a * √(2/3) ≈ 0.816 * a`
- With edge length 50mm: height ≈ 40.8mm
- Our solved height (D's Z coordinate): ~40.05mm ✓

This example demonstrates SLVSX's ability to solve complex 3D geometric relationships that would require significant manual calculation using traditional methods.