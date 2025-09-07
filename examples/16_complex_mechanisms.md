# Example 16: Complex Mechanisms

**[← Equal Radius](15_equal_radius.md)** | **[Back to Introduction →](00_introduction.md)**

## The Story

Real-world mechanisms combine multiple constraint types to create functional assemblies. Let's bring together everything we've learned to understand how SLVSX handles complex mechanical systems.

## Common Mechanism Patterns

### Four-Bar Linkage
The classic mechanism for converting rotary to linear motion:
- 4 points forming a quadrilateral
- Distance constraints for rigid links
- One fixed link (frame)
- Input and output cranks

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

## Example: Simple Four-Bar Linkage

Here's a complete four-bar linkage mechanism:

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "link1": 40,
    "link2": 60,
    "link3": 50,
    "link4": 70
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
      "at": [40, 0, 0]
    },
    {
      "type": "point",
      "id": "C",
      "at": [60, 40, 0]
    },
    {
      "type": "point",
      "id": "D",
      "at": [20, 50, 0]
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "A"
    },
    {
      "type": "fixed",
      "entity": "B"
    },
    {
      "type": "distance",
      "between": ["A", "D"],
      "value": "$link1"
    },
    {
      "type": "distance",
      "between": ["D", "C"],
      "value": "$link2"
    },
    {
      "type": "distance",
      "between": ["C", "B"],
      "value": "$link3"
    }
  ]
}
```

## Advanced Features

**Parameters**: Make designs adjustable by defining variables that can be referenced throughout

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

## Visual Output

![Complex Mechanisms](16_complex_mechanisms.svg)

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
4. [3D Tetrahedron](04_3d_tetrahedron.md) - Working in 3D
5. [Parallel/Perpendicular](05_parallel_perpendicular.md) - Geometric relationships
6. [Circles](06_circles.md) - Curved geometry
7. [Point on Line](07_point_on_line.md) - Sliding constraints
8. [Angles](08_angles.md) - Angular constraints
9. [Coincident](09_coincident.md) - Points meeting
10. [Equal Length](10_equal_length.md) - Matching sizes
11. [Symmetric](11_symmetric.md) - Mirror constraints
12. [3D Basics](12_3d_basics.md) - Spatial coordinates
13. [Horizontal/Vertical](13_horizontal_vertical.md) - Axis alignment
14. [Point on Circle](14_point_on_circle.md) - Circular paths
15. [Equal Radius](15_equal_radius.md) - Matching circles
16. [Complex Mechanisms](16_complex_mechanisms.md) - Putting it all together