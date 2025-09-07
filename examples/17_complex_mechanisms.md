# Example 17: Complex Mechanisms

**[← Mesh Constraint](16_mesh.md)** | **[Back to Introduction →](00_introduction.md)**

## The Story

Real-world mechanisms combine multiple constraint types to create functional assemblies. Let's bring together everything we've learned to understand how SLVSX handles complex mechanical systems.

## Common Mechanism Patterns

### Four-Bar Linkage
The classic mechanism for converting rotary to linear motion:
- 4 points forming a quadrilateral
- Distance constraints for rigid links
- One fixed link (frame)
- Input and output cranks

### Planetary Gear Train
Multiple gears orbiting a central sun:
- Sun gear (fixed or driven)
- Planet gears (mesh with sun)
- Ring gear (internal, meshes with planets)
- Carrier connecting planet centers

### Slider-Crank
Engine piston mechanism:
- Rotating crank
- Connecting rod
- Sliding piston (point-on-line)
- Fixed guides

## Constraint Combinations

Different constraints work together:

**Rigid Bodies**: 
- Use distance constraints between points
- Add angle constraints for fixed angles

**Sliding Motion**:
- Point-on-line for linear slides
- Point-on-circle for circular paths

**Gear Trains**:
- Mesh constraints for tooth engagement
- Distance for center spacing
- Fixed constraints for frames

**Symmetrical Designs**:
- Symmetric constraint for mirroring
- Equal length/radius for uniformity

## Degrees of Freedom

Understanding DOF is crucial:
- Each point starts with 2 DOF (X, Y)
- Each constraint removes DOF
- Fully constrained = 0 DOF
- Under-constrained = mechanism can move
- Over-constrained = conflicting requirements

## Design Workflow

1. **Sketch the mechanism**: Identify moving parts
2. **Place key points**: Joints, pivots, centers
3. **Add entities**: Lines for links, circles for paths
4. **Apply constraints**: Start with fixed references
5. **Test with parameters**: Make dimensions adjustable
6. **Validate solution**: Check for overlaps/conflicts

## Troubleshooting Tips

**"Inconsistent constraints"**:
- You have conflicting requirements
- Remove constraints one by one to find conflict

**"Under-constrained"**:
- Mechanism has freedom to move
- May be intentional for mechanisms
- Add constraints if full definition needed

**Phase validation failures** (gears):
- Teeth are colliding
- Solver found solution but it's not manufacturable
- Adjust positions or tooth counts

## Advanced Features

**Parameters**: Make designs adjustable
```json
"parameters": {
  "link_length": 100,
  "crank_angle": 45
}
```

**Expressions**: Use in constraints
```json
"value": "$link_length * 2"
```

**Multiple solutions**: Some constraints have multiple valid solutions. The solver picks one, but others may exist.

## Real-World Applications

SLVSX is used for:
- **3D Printing**: Ensuring parts fit and move
- **Robotics**: Joint and linkage design
- **Architecture**: Movable structures
- **Manufacturing**: Jigs and fixtures
- **Education**: Teaching mechanical principles

## Key Takeaways

1. **Start simple**: Build complexity gradually
2. **Fix references**: Always have fixed points
3. **Test incrementally**: Add constraints one at a time
4. **Use parameters**: Make designs flexible
5. **Validate physically**: Check for real-world feasibility

## What's Next?

You now have all the tools to create complex mechanisms:
- Combine different constraint types
- Use parameters for adjustable designs
- Validate solutions for manufacturing
- Export to CAD for further development

The constraint solver handles the math - you focus on the design!

---

**[Back to Introduction →](00_introduction.md)**

## Complete Example Index

1. [Introduction](00_introduction.md) - Overview and concepts
2. [Fixed Points](01_first_point.md) - Reference points
3. [Distance](02_distance_constraint.md) - Setting lengths
4. [Lines](03_lines_and_length.md) - Connecting points
5. [Triangles](04_triangle.md) - Rigid structures
6. [Parallel/Perpendicular](05_parallel_perpendicular.md) - Geometric relationships
7. [Circles](06_circles.md) - Curved geometry
8. [Point on Line](07_point_on_line.md) - Sliding constraints
9. [Angles](08_angles.md) - Angular constraints
10. [Coincident](09_coincident.md) - Points meeting
11. [Equal Length](10_equal_length.md) - Matching sizes
12. [Symmetric](11_symmetric.md) - Mirror constraints
13. [3D Basics](12_3d_basics.md) - Spatial coordinates
14. [Horizontal/Vertical](13_horizontal_vertical.md) - Axis alignment
15. [Point on Circle](14_point_on_circle.md) - Circular paths
16. [Equal Radius](15_equal_radius.md) - Matching circles
17. [Mesh (Gears)](16_mesh.md) - Gear engagement
18. [Complex Mechanisms](17_complex_mechanisms.md) - Putting it all together