# Geometry Generators

These Python scripts generate SLVSX JSON files using high-level abstractions.

## Why Use Generators?

Writing raw JSON for complex geometry is tedious and error-prone. These scripts provide:

- **Reusable components**: `add_box()`, `add_hollow_box()`, `add_gable_roof()`, etc.
- **Parametric design**: Change one value to resize the entire model
- **Automatic constraints**: Wall thickness maintained by distance constraints
- **Math operations**: Loops, calculations, conditionals

## Available Generators

### `birdhouse.py`

Creates a birdhouse with:
- Hollow walls (inner + outer shells)
- Gable roof
- Entrance hole
- Perch

```bash
# Generate JSON
python birdhouse.py > ../21_birdhouse.json

# Pipe directly to solver
python birdhouse.py | slvsx solve -

# Render
python birdhouse.py | slvsx export -f svg -v isometric -
```

Customize by editing the parameters:
```python
birdhouse = build_birdhouse(
    width=150,           # Overall width
    depth=150,           # Overall depth  
    wall_height=180,     # Height to roof
    roof_height=80,      # Ridge height above walls
    wall_thickness=10,   # Wall thickness
    entrance_diameter=40, # Hole size
)
```

## GeometryBuilder API

The `GeometryBuilder` class provides CAD-like abstractions:

```python
from birdhouse import GeometryBuilder, Point3D

g = GeometryBuilder()

# Low-level
g.add_point("p1", Point3D(0, 0, 0), fixed=True)
g.add_line("l1", "p1", "p2")
g.add_circle("c1", Point3D(50, 50, 0), diameter=40)

# High-level
g.add_rectangle("base", Point3D(0, 0, 0), width=100, depth=100)
g.add_box("cube", Point3D(0, 0, 0), width=100, depth=100, height=100)
g.add_hollow_box("walls", origin, width, depth, height, wall_thickness=10)
g.add_gable_roof("roof", corner_points, ridge_height=50)
g.add_hole("entrance", outer_pos, inner_pos, diameter=40)
g.add_dowel("perch", start, end, diameter=8)

# Output
print(g.to_json())
```

## Creating Your Own Generator

1. Copy `birdhouse.py` as a template
2. Import `GeometryBuilder` and `Point3D`
3. Define a build function with your parameters
4. Add geometry using the builder methods
5. Print the JSON output

Example - a simple bracket:
```python
def build_bracket(width=50, height=100, thickness=5):
    g = GeometryBuilder()
    
    # L-shaped profile
    g.add_point("p1", Point3D(0, 0, 0), fixed=True)
    g.add_point("p2", Point3D(width, 0, 0))
    g.add_point("p3", Point3D(width, thickness, 0))
    g.add_point("p4", Point3D(thickness, thickness, 0))
    g.add_point("p5", Point3D(thickness, height, 0))
    g.add_point("p6", Point3D(0, height, 0))
    
    # Connect the outline
    for i in range(1, 6):
        g.add_line(f"edge{i}", f"p{i}", f"p{i+1}")
    g.add_line("edge6", "p6", "p1")
    
    return g
```

