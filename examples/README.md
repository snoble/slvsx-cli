# SLVSX Examples

A comprehensive tutorial series for learning geometric constraint solving with SLVSX.

## Getting Started

These examples are designed to be read in order, building from simple concepts to complex mechanisms. Each example includes:

- **Story & Motivation**: Why this constraint matters
- **Complete JSON**: Copy-paste ready examples
- **Actual Output**: Real results from the SLVSX solver
- **Visualizations**: SVG outputs showing the solution 
- Generated SVG visualization
- Verified solver output

## Tutorial Sequence

### Fundamentals
1. **[Introduction](00_introduction.md)** - Overview and concepts
2. **[Your First Point](01_first_point.md)** - Fixed constraints and reference points
3. **[Distance Constraints](02_distance_constraint.md)** - Setting distances with parameters
4. **[Understanding Constraints](03_constraints.md)** - Over-constrained vs properly constrained
5. **[3D Tetrahedron](04_3d_tetrahedron.md)** - Working in three dimensions

### Geometric Relationships
6. **[Parallel and Perpendicular](05_parallel_perpendicular.md)** - Angular relationships
7. **[Working with Circles](06_circles.md)** - Circles and tangent constraints
8. **[Point on Line](07_point_on_line.md)** - Sliding motion along paths
9. **[Angle Constraints](08_angles.md)** - Precise angular control

### Advanced Constraints
10. **[Coincident Points](09_coincident.md)** - Points meeting at junctions
11. **[Equal Length](10_equal_length.md)** - Maintaining equal sizes
12. **[Symmetric Constraints](11_symmetric.md)** - Mirror symmetry
13. **[3D Basics](12_3d_basics.md)** - Working in three dimensions
14. **[Horizontal & Vertical](13_horizontal_vertical.md)** - Axis alignment shortcuts
15. **[Point on Circle](14_point_on_circle.md)** - Circular motion paths
16. **[Equal Radius](15_equal_radius.md)** - Matching circle sizes
16. **[Complex Mechanisms](16_complex_mechanisms.md)** - Putting it all together


## 📐 Parameter Studies

Most examples include parameters that can be varied:

```bash
# Explore design space by varying parameters
slvsx solve examples/16_complex_mechanisms.json \
  --param link1=45 \
  --param link2=65

# Export visualization
slvsx export --format svg examples/16_complex_mechanisms.json -o mechanism.svg
```

## 🎯 AI Agent Examples

These examples are perfect for AI agents because they:

1. **Accept programmatic input** - JSON specification instead of GUI interaction
2. **Support parameter variation** - Easy to explore design spaces
3. **Provide mathematical validation** - Constraint satisfaction guarantees feasibility
4. **Export multiple formats** - SVG for visualization, STL for manufacturing

### Agent Workflow Example

```python
import subprocess
import json

# Agent generates constraint specification
linkage_spec = {
    "schema": "slvs-json/1",
    "units": "mm", 
    "parameters": {"input_angle": 45, "link_length": 80},
    # ... constraint definition
}

# Solve constraints
result = subprocess.run([
    "slvsx", "solve", "-"], 
    input=json.dumps(linkage_spec),
    capture_output=True, text=True
)

# Parse solution
solution = json.loads(result.stdout)
if solution["status"] == "ok":
    print("✓ Valid mechanism found!")
    # Export for visualization/manufacturing
    subprocess.run([
        "slvsx", "export", "--format", "svg", 
        "-", "-o", "result.svg"
    ], input=json.dumps(linkage_spec), text=True)
```

## 🔍 Example Categories

- **📏 Geometric**: Basic shapes, distances, angles
- **🔗 Kinematic**: Linkages, motion constraints  
- **⚙️ Mechanical**: Linkages, assemblies, interference checking
- **📐 Parametric**: Design optimization, parameter studies
- **🏭 Manufacturing**: 3D printing, assembly validation

Each example demonstrates SLVSX's ability to turn mechanical design from manual sketching into mathematical constraint solving - perfect for AI agents doing systematic mechanical problem solving.