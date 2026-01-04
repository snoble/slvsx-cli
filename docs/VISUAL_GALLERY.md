# Visual Gallery: SLVSX in Action 🎨

This gallery showcases SLVSX's capabilities through rendered visualizations. Each example demonstrates different constraint types and geometric relationships.

> **💡 Tip**: All renders are generated from actual constraint solutions - they're mathematically precise, not approximations!

## 🎬 Quick Navigation

- [3D Geometry](#-3d-geometry) - Tetrahedron, coordinate systems
- [Mechanisms & Linkages](#-mechanisms--linkages) - Hinges, symmetric designs
- [Geometric Shapes](#-geometric-shapes) - Rectangles, parallel/perpendicular
- [Creating Visualizations](#-creating-your-own-visualizations) - How to generate your own

## 🎨 3D Geometry

### Tetrahedron (Multiple Views)

A perfect tetrahedron with all edges equal length, shown from different viewing angles:

**XY View (Top-Down)**
![Tetrahedron XY View](examples/outputs/tetrahedron_xy.svg)

**XZ View (Front)**
![Tetrahedron XZ View](examples/outputs/tetrahedron_xz.svg)

**YZ View (Side)**
![Tetrahedron YZ View](examples/outputs/tetrahedron_yz.svg)

**What it demonstrates**:
- 3D constraint solving
- Equal-length constraints
- Multiple viewing angles
- Perfect geometric precision

### 3D Coordinate System

A 3D coordinate system showing X, Y, and Z axes with a point in space:

**XY View**
![3D Basics XY View](examples/outputs/3d_basics_xy.svg)

**XZ View**
![3D Basics XZ View](examples/outputs/3d_basics_xz.svg)

**What it demonstrates**:
- 3D point positioning
- Distance constraints in 3D space
- Multiple reference frames

## 🔧 Mechanisms & Linkages

### Angle-Controlled Hinge

A parametric hinge mechanism where two arms meet at a specific angle:

![Angle Hinge](examples/outputs/08_angles.svg)

**What it demonstrates**:
- Angle constraints
- Horizontal alignment
- Parametric design (change `hinge_angle` parameter)
- Real-world mechanism design

**Try it**:
```bash
slvsx solve examples/08_angles.json
slvsx export -f svg examples/08_angles.json -o hinge.svg
```

### Symmetric Arrowhead

A perfectly symmetric arrowhead design:

![Symmetric Arrow](examples/outputs/11_symmetric.svg)

**What it demonstrates**:
- Symmetry constraints
- Point-on-line constraints
- Complex geometric relationships
- Design precision

## 📐 Geometric Shapes

### Triangle from Distances

A classic triangle construction using distance constraints:

![Triangle](examples/outputs/02_triangle.svg)

**What it demonstrates**:
- Distance constraints
- Multiple solutions (two possible positions)
- Basic geometric construction
- Triangulation

**Try it**:
```bash
slvsx solve examples/02_triangle.json
slvsx export -f svg examples/02_triangle.json -o triangle.svg
```

## 🎯 Creating Your Own Visualizations

### Automated Render Generation

Use the provided script to generate all renders:

```bash
./scripts/generate_renders.sh
```

This will:
- Solve all examples
- Generate SVG renders for 2D examples
- Create multiple views (XY, XZ, YZ) for 3D examples
- Save everything to `examples/outputs/`

**CI Verification**: Renders are automatically generated and verified in CI. If examples change, CI will fail until renders are updated. This ensures renders always match the current examples.

**Local Verification**: Check if your renders are up-to-date:
```bash
./scripts/verify_renders.sh
```

### Export from Multiple Angles

For 3D objects, export from different viewing angles to see them from all sides:

```bash
# Top-down view (XY plane) - looking down from above
slvsx export -f svg -v xy examples/04_3d_tetrahedron.json -o top.svg

# Front view (XZ plane) - looking from the front
slvsx export -f svg -v xz examples/04_3d_tetrahedron.json -o front.svg

# Side view (YZ plane) - looking from the side
slvsx export -f svg -v yz examples/04_3d_tetrahedron.json -o side.svg
```

**Pro Tip**: Export all three views to create a comprehensive visualization showing your 3D design from every angle!

### Export to Different Formats

```bash
# SVG for web/documentation
slvsx export -f svg examples/08_angles.json -o output.svg

# DXF for CAD import
slvsx export -f dxf examples/08_angles.json -o output.dxf

# STL for 3D printing
slvsx export -f stl examples/04_3d_tetrahedron.json -o output.stl
```

## 🔍 What Makes These Visualizations Special?

1. **Mathematical Precision**: Every line, angle, and distance is exactly as specified
2. **Constraint Satisfaction**: All constraints are satisfied simultaneously
3. **Multiple Views**: 3D objects can be viewed from any angle
4. **Export Flexibility**: Same constraint definition, multiple output formats

## 📚 More Examples

Explore the [`examples/`](../examples/) directory for:
- More 3D geometries
- Complex mechanisms
- Parametric designs
- Real-world applications

Each example can be solved and exported to create your own visualizations!

## 🎬 Animation Potential

These static renders can be turned into animations by:
1. Varying parameters (e.g., `hinge_angle` from 0° to 180°)
2. Exporting each frame
3. Combining into animated GIF or video

Example workflow:
```bash
for angle in {0..180..10}; do
  # Modify JSON with new angle value
  slvsx solve modified.json > /dev/null
  slvsx export -f svg modified.json -o frame_${angle}.svg
done
# Combine frames into animation
```

---

**Pro Tip**: Use SVG exports in documentation, presentations, or web pages. The vector format scales perfectly and looks great at any size. For 3D objects, export multiple views to show depth and structure!

