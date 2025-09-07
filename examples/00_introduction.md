# SLVSX Examples: Introduction

Welcome to the SLVSX constraint solver examples! These tutorials will guide you through geometric constraint solving, from simple 2D sketches to complex 3D assemblies.

## What is Constraint Solving?

Imagine you're designing a mechanical linkage, laying out a floor plan, or creating a parametric design. You know certain relationships must hold:
- This distance should be exactly 100mm
- These two lines should be perpendicular
- This point should be at the midpoint of that line

A constraint solver takes these relationships and figures out the actual positions that satisfy all constraints simultaneously.

## How These Examples Work

Each example includes:
1. **The Story** - What we're building and why
2. **The Constraints** - The geometric relationships we're defining
3. **The JSON** - The actual constraint specification
4. **The Solution** - What the solver calculated
5. **Visual Output** - An SVG showing the result

## Example Structure

Every SLVSX document has this structure:

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {},
  "entities": [],
  "constraints": []
}
```

## Navigation

Start with the first example:

**[Next: Example 01 - Your First Point →](01_first_point.md)**

---

## Quick Reference

- **Entities**: point, line, circle, arc, gear, plane
- **Constraints**: fixed, distance, angle, parallel, perpendicular, coincident, equal_length, equal_radius, symmetric, horizontal, vertical, point_on_line, point_on_circle, tangent, mesh
- **Parameters**: Use `$name` to reference a parameter value

## Complete Example Index

### Fundamentals
1. [Your First Point](01_first_point.md) - Fixed constraints and reference points
2. [Distance Constraints](02_distance_constraint.md) - Setting distances with parameters
3. [Lines and Length](03_lines_and_length.md) - Line entities and their behavior
4. [Building a Triangle](04_triangle.md) - Combining multiple constraints

### Geometric Relationships
5. [Parallel and Perpendicular](05_parallel_perpendicular.md) - Angular relationships
6. [Working with Circles](06_circles.md) - Circles and tangent constraints
7. [Point on Line](07_point_on_line.md) - Sliding motion along paths
8. [Angle Constraints](08_angles.md) - Precise angular control

### Advanced Constraints
9. [Coincident Points](09_coincident.md) - Points meeting at junctions
10. [Equal Length](10_equal_length.md) - Maintaining equal sizes
11. [Symmetric Constraints](11_symmetric.md) - Mirror symmetry
12. [3D Basics](12_3d_basics.md) - Working in three dimensions
13. [Horizontal & Vertical](13_horizontal_vertical.md) - Axis alignment shortcuts
14. [Point on Circle](14_point_on_circle.md) - Circular motion paths
15. [Equal Radius](15_equal_radius.md) - Matching circle sizes
16. [Mesh Constraint (Gears)](16_mesh.md) - Gear tooth engagement
17. [Complex Mechanisms](17_complex_mechanisms.md) - Putting it all together

Let's begin!