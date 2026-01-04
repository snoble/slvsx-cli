# Visual Gallery: SLVSX in Action

This gallery showcases SLVSX's capabilities through rendered visualizations. Each example demonstrates different constraint types and geometric relationships.

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

### Parallel and Perpendicular Lines

A rectangle constructed using parallel and perpendicular constraints:

![Parallel Perpendicular](examples/outputs/05_parallel_perpendicular.svg)

**What it demonstrates**:
- Parallel constraints
- Perpendicular relationships
- Distance constraints
- Geometric construction

## 🎯 Creating Your Own Visualizations

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

