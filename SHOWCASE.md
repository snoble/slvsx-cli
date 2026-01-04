# SLVSX Showcase: What Can You Build?

This document demonstrates the powerful capabilities of SLVSX through real, working examples. Perfect for exploring what's possible and inspiring new projects.

## 🎯 Quick Wins: Solve These in Seconds

### 1. Triangle from Two Distances

**Problem**: Given two fixed points and distances to a third point, find where the third point must be.

```bash
slvsx solve examples/02_triangle.json
```

**What it demonstrates**: 
- Basic geometric construction
- Distance constraints
- Multiple solutions (two possible positions)

**Use cases**: Triangulation, positioning systems, GPS-like calculations

### 2. Parametric Hinge with Angle Control

**Problem**: Create a hinge mechanism where two arms meet at a specific angle.

```bash
slvsx solve examples/08_angles.json
```

**What it demonstrates**:
- Angle constraints
- Horizontal alignment
- Parametric design (change `hinge_angle` parameter)

**Use cases**: Mechanical linkages, robotic arms, folding mechanisms

### 3. Symmetric Arrowhead

**Problem**: Design a symmetric arrowhead shape with precise control.

```bash
slvsx solve examples/11_symmetric.json
```

**What it demonstrates**:
- Symmetry constraints
- Point-on-line constraints
- Complex geometric relationships

**Use cases**: Logo design, decorative patterns, architectural elements

## 🔧 Real-World Applications

### Kinematic Mechanisms

#### Four-Bar Linkage
```bash
slvsx solve examples/ai-examples/four_bar_linkage.json
slvsx export -f svg examples/ai-examples/four_bar_linkage.json -o linkage.svg
```

**What it does**: Validates a classic four-bar linkage mechanism used in:
- Engine piston mechanisms
- Robotic joints
- Mechanical toys
- Industrial automation

**Key constraints**:
- Fixed ground points
- Link lengths (crank, coupler, rocker)
- Proper kinematic closure

#### Planetary Gear System
```bash
slvsx solve examples/ai-examples/gear_meshing.json
```

**What it does**: Positions planetary gears around a sun gear with proper meshing distances.

**Key constraints**:
- Fixed sun gear center
- Equal distances from sun to planets
- Proper spacing between planets

**Use cases**: Gearbox design, transmission systems, mechanical power distribution

### Parametric Design

#### Parametric Square
Create a square where the side length is a parameter:

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "side_length": 80.0
  },
  "entities": [
    {"type": "point", "id": "A", "at": [0, 0, 0]},
    {"type": "point", "id": "B", "at": [80, 0, 0]},
    {"type": "point", "id": "C", "at": [80, 80, 0]},
    {"type": "point", "id": "D", "at": [0, 80, 0]},
    {"type": "line", "id": "AB", "p1": "A", "p2": "B"},
    {"type": "line", "id": "BC", "p1": "B", "p2": "C"},
    {"type": "line", "id": "CD", "p1": "C", "p2": "D"},
    {"type": "line", "id": "DA", "p1": "D", "p2": "A"}
  ],
  "constraints": [
    {"type": "fixed", "entity": "A"},
    {"type": "distance", "between": ["A", "B"], "value": "$side_length"},
    {"type": "distance", "between": ["B", "C"], "value": "$side_length"},
    {"type": "distance", "between": ["C", "D"], "value": "$side_length"},
    {"type": "distance", "between": ["D", "A"], "value": "$side_length"},
    {"type": "perpendicular", "a": "AB", "b": "BC"}
  ]
}
```

**Try it**:
```bash
# Change the side length
slvsx solve examples/19_parametric_square.json
# Modify the JSON to use different side_length values
```

**Use cases**: 
- Design optimization
- Batch generation of similar shapes
- Template-based design systems

## 🤖 AI Agent Workflows

### Pattern 1: Constraint-Based Design Generation

AI agents can generate designs by specifying constraints rather than exact coordinates:

```python
import json
import subprocess

def design_rectangle(width, height):
    """Generate a rectangle design from constraints."""
    problem = {
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "A", "at": [0, 0, 0]},
            {"type": "point", "id": "B", "at": [width, 0, 0]},
            {"type": "point", "id": "C", "at": [width, height, 0]},
            {"type": "point", "id": "D", "at": [0, height, 0]},
            {"type": "line", "id": "AB", "p1": "A", "p2": "B"},
            {"type": "line", "id": "BC", "p1": "B", "p2": "C"},
            {"type": "line", "id": "CD", "p1": "C", "p2": "D"},
            {"type": "line", "id": "DA", "p1": "D", "p2": "A"}
        ],
        "constraints": [
            {"type": "fixed", "entity": "A"},
            {"type": "horizontal", "a": "AB"},
            {"type": "vertical", "a": "BC"},
            {"type": "distance", "between": ["A", "B"], "value": width},
            {"type": "distance", "between": ["A", "D"], "value": height}
        ]
    }
    
    result = subprocess.run(
        ["slvsx", "solve", "-"],
        input=json.dumps(problem),
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        solution = json.loads(result.stdout)
        return solution["entities"]
    else:
        raise ValueError(f"Solver failed: {result.stderr}")

# Generate different sized rectangles
for w, h in [(100, 50), (200, 100), (150, 75)]:
    coords = design_rectangle(w, h)
    print(f"Rectangle {w}x{h}: {coords}")
```

### Pattern 2: Mechanism Validation

AI agents can validate mechanical designs before manufacturing:

```python
def validate_linkage(link_lengths):
    """Validate a four-bar linkage is physically possible."""
    problem = {
        "schema": "slvs-json/1",
        "units": "mm",
        "parameters": {f"link_{i}": length for i, length in enumerate(link_lengths)},
        # ... entity and constraint definitions
    }
    
    result = subprocess.run(
        ["slvsx", "validate", "-"],
        input=json.dumps(problem),
        capture_output=True,
        text=True
    )
    
    return result.returncode == 0

# Check if a linkage design is valid
if validate_linkage([70, 100, 90, 120]):
    print("✓ Linkage is valid!")
else:
    print("✗ Linkage cannot be assembled")
```

### Pattern 3: Design Space Exploration

AI agents can explore design spaces systematically:

```python
def explore_angles(base_length, angle_range):
    """Explore different angle configurations."""
    solutions = []
    for angle in angle_range:
        problem = {
            "schema": "slvs-json/1",
            "units": "mm",
            "parameters": {
                "arm_length": base_length,
                "hinge_angle": angle
            },
            # ... entity and constraint definitions
        }
        
        result = subprocess.run(
            ["slvsx", "solve", "-"],
            input=json.dumps(problem),
            capture_output=True,
            text=True
        )
        
        if result.returncode == 0:
            solutions.append((angle, json.loads(result.stdout)))
    
    return solutions

# Find all valid configurations
valid_configs = explore_angles(80, range(0, 180, 10))
print(f"Found {len(valid_configs)} valid configurations")
```

## 🎨 Creative Applications

### 1. Logo Design
Use symmetry and equal-length constraints to create balanced logos:

```bash
slvsx solve examples/11_symmetric.json
slvsx export -f svg examples/11_symmetric.json -o logo.svg
```

### 2. Architectural Layouts
Design floor plans with precise room dimensions:

- Fixed reference points (corners)
- Distance constraints (room dimensions)
- Perpendicular walls
- Equal-length parallel walls

### 3. Mechanical Drawings
Create technical drawings with proper constraints:

- Dimension lines
- Geometric tolerances
- Assembly relationships

## 📊 Export Capabilities

### SVG Visualization
```bash
slvsx export -f svg examples/08_angles.json -o output.svg
```
Perfect for:
- Documentation
- Presentations
- Web visualization
- Design review

### DXF Export
```bash
slvsx export -f dxf examples/02_triangle.json -o output.dxf
```
Perfect for:
- CAD import
- Manufacturing drawings
- CNC programming

### STL Export (3D)
```bash
slvsx export -f stl examples/04_3d_tetrahedron.json -o output.stl
```
Perfect for:
- 3D printing
- Rapid prototyping
- Manufacturing

## 🔍 What Makes SLVSX Special?

### 1. **Mathematical Precision**
Not just "close enough" - solutions satisfy constraints exactly (within numerical precision).

### 2. **Constraint-First Design**
Describe *what* you want, not *how* to draw it. The solver figures out the positions.

### 3. **AI-Friendly Interface**
JSON input/output makes it perfect for programmatic use by AI agents.

### 4. **Fast & Reliable**
Native C++ solver wrapped in Rust - fast enough for interactive use.

### 5. **Multiple Export Formats**
One constraint definition, multiple output formats for different use cases.

## 🚀 Next Steps

1. **Try the examples**: Run through the examples in `examples/` directory
2. **Modify parameters**: Change parameter values to see how designs adapt
3. **Create your own**: Start with a simple shape and add constraints
4. **Export and visualize**: Use SVG export to see your designs
5. **Integrate with AI**: Use the Python patterns above in your AI workflows

## 📚 Learn More

- [Examples Directory](examples/) - Comprehensive tutorial series
- [AI Examples](examples/ai-examples/) - Ready-to-use constraint problems
- [Usage Guide](docs/USAGE_EXAMPLES.md) - Detailed usage patterns
- [MCP Integration](docs/MCP_INTEGRATION.md) - Use with Claude and other AI

---

**Ready to build something amazing?** Start with `slvsx solve examples/02_triangle.json` and see what you can create!

