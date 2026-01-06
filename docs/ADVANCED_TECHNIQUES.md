# SLVSX Advanced Modeling Techniques

This guide covers advanced techniques for building complex geometry with SLVSX, including programmatic generation, reusable patterns, and interactive refinement.

## Table of Contents
- [Programmatic Geometry Generation](#programmatic-geometry-generation)
- [Walls with Thickness (Hollow Shells)](#walls-with-thickness-hollow-shells)
- [Repeated Patterns](#repeated-patterns)
- [Kinematic Mechanisms](#kinematic-mechanisms)
- [Interactive Refinement with Preserve/Dragged](#interactive-refinement-with-preservedragged)
- [Iterative Design Workflow](#iterative-design-workflow)

---

## Programmatic Geometry Generation

For complex models, writing JSON by hand is tedious and error-prone. Use a script to generate the JSON programmatically.

### Python Generator Example

```python
#!/usr/bin/env python3
"""Generate geometry programmatically"""
import json
import math

class GeometryBuilder:
    def __init__(self):
        self.entities = []
        self.constraints = []
        self.parameters = {}
    
    def add_point(self, id, x, y, z=0, fixed=False):
        self.entities.append({
            "type": "point", "id": id, "at": [x, y, z]
        })
        if fixed:
            self.constraints.append({"type": "fixed", "entity": id})
        return id
    
    def add_line(self, id, p1, p2):
        self.entities.append({
            "type": "line", "id": id, "p1": p1, "p2": p2
        })
        return id
    
    def add_circle(self, id, cx, cy, cz, diameter, normal=(0, 0, 1)):
        self.entities.append({
            "type": "circle", "id": id, 
            "center": [cx, cy, cz], 
            "diameter": diameter,
            "normal": list(normal)
        })
        return id
    
    def add_distance(self, p1, p2, value):
        self.constraints.append({
            "type": "distance", "between": [p1, p2], "value": value
        })
    
    def to_json(self):
        return json.dumps({
            "schema": "slvs-json/1",
            "units": "mm",
            "parameters": self.parameters,
            "entities": self.entities,
            "constraints": self.constraints
        }, indent=2)

# Usage
g = GeometryBuilder()
g.add_point("origin", 0, 0, 0, fixed=True)
g.add_point("p1", 100, 0, 0)
g.add_distance("origin", "p1", 100)
print(g.to_json())
```

Run it: `python generate.py | slvsx solve -`

---

## Walls with Thickness (Hollow Shells)

SLVSX is a wireframe solver - it doesn't create solid geometry. To represent walls with thickness, create **inner and outer shells**.

### Hollow Box Pattern

```python
def add_hollow_box(self, prefix, origin, width, depth, height, wall_thickness, fixed=False):
    """Create inner and outer box shells for walls with thickness"""
    
    # Outer shell (8 corners)
    outer = []
    for z_off in [0, height]:
        for name, dx, dy in [("fl", 0, 0), ("fr", width, 0), 
                              ("br", width, depth), ("bl", 0, depth)]:
            pt_id = f"{prefix}_outer_{name}_{z_off}"
            self.add_point(pt_id, origin[0] + dx, origin[1] + dy, origin[2] + z_off, fixed)
            outer.append(pt_id)
    
    # Inner shell (offset by wall_thickness)
    wt = wall_thickness
    inner = []
    inner_width = width - 2*wt
    inner_depth = depth - 2*wt
    for z_off in [0, height]:
        for name, dx, dy in [("fl", 0, 0), ("fr", inner_width, 0),
                              ("br", inner_width, inner_depth), ("bl", 0, inner_depth)]:
            pt_id = f"{prefix}_inner_{name}_{z_off}"
            self.add_point(pt_id, origin[0] + wt + dx, origin[1] + wt + dy, origin[2] + z_off)
            inner.append(pt_id)
    
    # Add lines for edges, connect corners to visualize walls
    # The gap between outer and inner shells IS the wall
    
    return {"outer": outer, "inner": inner}
```

### Workflow for CAD Export
1. Solve the geometry in SLVSX
2. Export as DXF or SLVS
3. Import into SolveSpace/FreeCAD
4. Loft or extrude between inner and outer shells to create solid walls

---

## Repeated Patterns

Use loops to create repeated geometry like bolt circles, grids, or arrays.

### Bolt Circle Pattern

```python
def add_bolt_circle(self, prefix, center, radius, num_holes, hole_diameter):
    """Add holes arranged in a circle"""
    for i in range(num_holes):
        angle = 2 * math.pi * i / num_holes
        x = center[0] + radius * math.cos(angle)
        y = center[1] + radius * math.sin(angle)
        self.add_circle(f"{prefix}_hole_{i}", x, y, center[2], hole_diameter)
    return [f"{prefix}_hole_{i}" for i in range(num_holes)]
```

### Linear Array

```python
def add_linear_array(self, prefix, start, direction, spacing, count):
    """Add points in a line with equal spacing"""
    points = []
    for i in range(count):
        offset = i * spacing
        x = start[0] + direction[0] * offset
        y = start[1] + direction[1] * offset
        z = start[2] + direction[2] * offset
        pt_id = f"{prefix}_{i}"
        self.add_point(pt_id, x, y, z)
        points.append(pt_id)
    
    # Add equal distance constraints
    for i in range(count - 1):
        self.add_distance(points[i], points[i+1], spacing)
    
    return points
```

### Grid Pattern

```python
def add_grid(self, prefix, origin, cols, rows, col_spacing, row_spacing):
    """Add points in a grid"""
    points = []
    for row in range(rows):
        for col in range(cols):
            pt_id = f"{prefix}_{row}_{col}"
            self.add_point(pt_id, 
                          origin[0] + col * col_spacing,
                          origin[1] + row * row_spacing,
                          origin[2])
            points.append(pt_id)
    return points
```

---

## Kinematic Mechanisms

SLVSX excels at kinematic mechanisms where parts move relative to each other.

### Four-Bar Linkage

```python
def add_four_bar_linkage(self, ground1, ground2, link1_len, link2_len, link3_len, input_angle):
    """Classic four-bar linkage mechanism"""
    # Fixed ground pivots
    self.add_point("ground1", ground1[0], ground1[1], 0, fixed=True)
    self.add_point("ground2", ground2[0], ground2[1], 0, fixed=True)
    
    # Moving joints
    x1 = ground1[0] + link1_len * math.cos(math.radians(input_angle))
    y1 = ground1[1] + link1_len * math.sin(math.radians(input_angle))
    self.add_point("joint1", x1, y1, 0)
    self.add_point("joint2", ground2[0], ground2[1] + 50, 0)  # Initial guess
    
    # Links
    self.add_line("link1", "ground1", "joint1")
    self.add_line("link2", "joint1", "joint2")
    self.add_line("link3", "joint2", "ground2")
    
    # Constraints
    self.add_distance("ground1", "joint1", link1_len)
    self.add_distance("joint1", "joint2", link2_len)
    self.add_distance("joint2", "ground2", link3_len)
    
    # Input angle constraint
    self.constraints.append({
        "type": "angle", 
        "between": ["link1", "ground_line"],
        "value": input_angle
    })
```

### Animate Mechanisms

Create multiple solutions at different input angles:

```bash
for angle in 0 30 60 90 120 150 180; do
    python linkage.py --angle $angle > frame_$angle.json
    slvsx solve frame_$angle.json | slvsx export -f svg -o frame_$angle.svg
done
```

---

## Interactive Refinement with Preserve/Dragged

The `preserve` flag and `Dragged` constraint enable interactive refinement workflows.

### What is Preserve?

When `preserve: true` is set on an entity, the solver tries to minimize changes to that entity's position during solving. This is useful when:

- **Refining a design**: Keep base geometry fixed while adjusting other parts
- **Incremental changes**: Make small tweaks without the whole model jumping
- **Stable reference points**: Ensure key dimensions don't shift unexpectedly

### Example: Preserve Base Points

```json
{
  "entities": [
    {"type": "point", "id": "origin", "at": [0, 0, 0], "preserve": true},
    {"type": "point", "id": "corner", "at": [100, 0, 0], "preserve": true},
    {"type": "point", "id": "top", "at": [50, 80, 0]}
  ],
  "constraints": [
    {"type": "distance", "between": ["origin", "top"], "value": 100}
  ]
}
```

The solver will adjust `top` to satisfy the distance, keeping `origin` and `corner` stable.

### What is Dragged?

The `Dragged` constraint absolutely locks a point to its current position. Unlike `Fixed` which can be part of under-constrained systems, `Dragged` means "don't move this, period."

Use cases:
- **AI-guided exploration**: Lock parts of a design while exploring alternatives
- **Multi-stage solving**: Solve part of a model, lock results, add more constraints
- **What-if analysis**: "What if this point was here?" - drag it and re-solve

### Example: Dragged Constraint

```json
{
  "entities": [
    {"type": "point", "id": "pivot", "at": [50, 50, 0]},
    {"type": "point", "id": "handle", "at": [100, 50, 0]}
  ],
  "constraints": [
    {"type": "dragged", "point": "pivot"},
    {"type": "distance", "between": ["pivot", "handle"], "value": 60}
  ]
}
```

The handle will move to satisfy the distance, but the pivot stays exactly at (50, 50, 0).

---

## Iterative Design Workflow

Complex models should be built incrementally, testing at each stage.

### Step 1: Start Simple
```python
# Just the base
g = GeometryBuilder()
g.add_point("origin", 0, 0, 0, fixed=True)
g.add_point("p1", 100, 0, 0)
g.add_distance("origin", "p1", 100)
# Test: python step1.py | slvsx solve -
```

### Step 2: Add One Feature
```python
# Add a second point
g.add_point("p2", 100, 100, 0)
g.add_distance("p1", "p2", 100)
# Test again
```

### Step 3: Check Constraint Count
- Under-constrained = extra DOF = model can move freely
- Over-constrained = conflict = solver fails
- Fully constrained = exact solution

### Step 4: Visualize Often
```bash
python model.py | slvsx solve - | slvsx export -f svg --view isometric -o model.svg
open model.svg  # or xdg-open on Linux
```

### Step 5: Use Parameters
```python
g.parameters["width"] = 100
g.parameters["height"] = 50
# Reference in entities with "$width", "$height"
```

---

## Circle Orientation (Normal Vectors)

Circles have a `normal` vector that defines their orientation in 3D space. This affects how they render in different views.

### Default: XY Plane
```json
{"type": "circle", "center": [50, 50, 0], "diameter": 40, "normal": [0, 0, 1]}
```
Points along +Z, renders as full circle in XY view.

### On a Wall (YZ Plane)
```json
{"type": "circle", "center": [0, 50, 50], "diameter": 40, "normal": [1, 0, 0]}
```
Points along +X, renders as full circle in YZ view, as a line in XY view.

### Calculating Normals for Walls

For a hole in a wall facing direction (dx, dy, dz), the normal is simply (dx, dy, dz) normalized.

Front wall (facing -Y): `normal: [0, -1, 0]`
Side wall (facing +X): `normal: [1, 0, 0]`
Floor (facing +Z): `normal: [0, 0, 1]`

---

## Tips and Best Practices

1. **Name entities descriptively**: `roof_front_left` not `p47`
2. **Use prefixes for groups**: `base_fl`, `base_fr`, `roof_ridge`
3. **Fix the origin**: Always have at least one fixed point as reference
4. **Build in layers**: Base → structure → details → decorations
5. **Test constraint counts**: Too many = overconstrained, too few = floating
6. **Export often**: Visual feedback catches errors early
7. **Use isometric view**: See 3D geometry from a useful angle
8. **Document parameters**: What does each dimension control?

---

## Complete Birdhouse Example

See `examples/generators/birdhouse.py` for a complete example that uses:
- Hollow box for wall thickness
- Gable roof generation
- Hole patterns for entrance
- Dowel/cylinder for perch
- Parametric dimensions

Run it:
```bash
cd examples/generators
python birdhouse.py | slvsx solve - | slvsx export -f svg --view isometric -o birdhouse.svg
```

