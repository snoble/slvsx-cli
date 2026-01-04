# AI Agent Guide to SLVSX

This guide is designed for AI agents (like Claude, GPT-4, etc.) to understand and use SLVSX effectively for geometric constraint solving tasks.

## Quick Start

```python
import json
import subprocess

def solve_constraints(problem_json):
    """Solve a constraint problem and return the solution."""
    result = subprocess.run(
        ['slvsx', 'solve', '-'],
        input=json.dumps(problem_json),
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        return json.loads(result.stdout)
    else:
        raise ValueError(f"Solver error: {result.stderr}")

# Example: Solve a triangle
triangle = {
    "schema": "slvs-json/1",
    "units": "mm",
    "entities": [
        {"type": "point", "id": "A", "at": [0, 0, 0]},
        {"type": "point", "id": "B", "at": [100, 0, 0]},
        {"type": "point", "id": "C", "at": [50, 50, 0]}
    ],
    "constraints": [
        {"type": "fixed", "entity": "A"},
        {"type": "fixed", "entity": "B"},
        {"type": "distance", "between": ["A", "C"], "value": 80},
        {"type": "distance", "between": ["B", "C"], "value": 60}
    ]
}

solution = solve_constraints(triangle)
print(f"Point C is at: {solution['entities']['C']['at']}")
```

## Understanding the Problem Structure

Every SLVSX problem has this structure:

```json
{
  "schema": "slvs-json/1",      // Schema version (always use this)
  "units": "mm",                 // Units: "mm", "in", "m", etc.
  "parameters": {},              // Optional: Named parameters
  "entities": [],                // Geometric entities (points, lines, circles)
  "constraints": []              // Relationships between entities
}
```

## Entity Types

### Points
```json
{"type": "point", "id": "p1", "at": [x, y, z]}
```
- `id`: Unique identifier (string)
- `at`: Initial position [x, y, z] (used as starting guess)

### Lines
```json
{"type": "line", "id": "l1", "p1": "p1", "p2": "p2"}
```
- `p1`, `p2`: References to point entities

### Circles
```json
{"type": "circle", "id": "c1", "center": [x, y, z], "diameter": 50}
```
- `center`: Center point coordinates
- `diameter`: Circle diameter

## Constraint Types

### Fixed Constraint
```json
{"type": "fixed", "entity": "p1"}
```
Anchors an entity at its current position. Essential for providing reference points.

### Distance Constraint
```json
{"type": "distance", "between": ["p1", "p2"], "value": 100}
```
Sets exact distance between two entities. Can use parameters: `"value": "$param_name"`

### Angle Constraint
```json
{"type": "angle", "between": ["l1", "l2"], "value": 45}
```
Sets angle between two lines (in degrees). Can use parameters.

### Horizontal/Vertical
```json
{"type": "horizontal", "a": "l1"}
{"type": "vertical", "a": "l1"}
```
Aligns a line with horizontal or vertical axis.

### Parallel/Perpendicular
```json
{"type": "parallel", "entities": ["l1", "l2"]}
{"type": "perpendicular", "a": "l1", "b": "l2"}
```
Geometric relationships without specific values.

### Equal Length/Radius
```json
{"type": "equal_length", "entities": ["l1", "l2", "l3"]}
{"type": "equal_radius", "a": "c1", "b": "c2"}
```
Ensures multiple entities have equal size.

### Point on Line/Circle
```json
{"type": "point_on_line", "point": "p1", "line": "l1"}
{"type": "point_on_circle", "point": "p1", "circle": "c1"}
```
Constrains a point to lie on a line or circle.

### Symmetric
```json
{"type": "symmetric", "a": "p1", "b": "p2", "about": "l1"}
```
Makes two entities symmetric about a line.

### Midpoint
```json
{"type": "midpoint", "point": "p1", "of": "l1"}
```
Constrains a point to be the midpoint of a line.

## Common Patterns

### Pattern 1: Triangle from Distances
```python
def create_triangle(point_a, point_b, dist_ac, dist_bc):
    return {
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "A", "at": point_a},
            {"type": "point", "id": "B", "at": point_b},
            {"type": "point", "id": "C", "at": [
                (point_a[0] + point_b[0]) / 2,
                (point_a[1] + point_b[1]) / 2 + 50,
                0
            ]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "A"},
            {"type": "fixed", "entity": "B"},
            {"type": "distance", "between": ["A", "C"], "value": dist_ac},
            {"type": "distance", "between": ["B", "C"], "value": dist_bc}
        ]
    }
```

### Pattern 2: Rectangle
```python
def create_rectangle(width, height, origin=[0, 0, 0]):
    return {
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "A", "at": origin},
            {"type": "point", "id": "B", "at": [origin[0] + width, origin[1], origin[2]]},
            {"type": "point", "id": "C", "at": [origin[0] + width, origin[1] + height, origin[2]]},
            {"type": "point", "id": "D", "at": [origin[0], origin[1] + height, origin[2]]},
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
```

### Pattern 3: Parametric Design
```python
def parametric_design(parameters):
    return {
        "schema": "slvs-json/1",
        "units": "mm",
        "parameters": parameters,
        "entities": [
            # Use $param_name in constraints
        ],
        "constraints": [
            {"type": "distance", "between": ["p1", "p2"], "value": "$length"}
        ]
    }
```

## Error Handling

### Validation Before Solving
```python
def validate_problem(problem_json):
    """Check if problem is valid without solving."""
    result = subprocess.run(
        ['slvsx', 'validate', '-'],
        input=json.dumps(problem_json),
        capture_output=True,
        text=True
    )
    return result.returncode == 0

if not validate_problem(problem):
    print("Problem is invalid!")
```

### Understanding Errors

**Overconstrained**: Too many constraints conflict
```json
{"status": "error", "error": "System is inconsistent (overconstrained)"}
```

**Underconstrained**: Not enough constraints (multiple solutions possible)
```json
{"status": "error", "error": "System has too many unknowns (underconstrained)"}
```

**Invalid Reference**: Entity ID doesn't exist
```json
{"status": "error", "error": "Unknown entity reference 'nonexistent'"}
```

## Best Practices

1. **Always provide initial guesses**: Use realistic `at` values for points
2. **Fix at least one point**: Provides reference frame
3. **Check degrees of freedom**: Ensure proper constraint count
4. **Use parameters**: Makes designs flexible and reusable
5. **Validate first**: Use `slvsx validate` before solving
6. **Handle errors gracefully**: Check return codes and parse error messages

## Example Workflows

### Workflow 1: Design Generation
```python
# 1. Generate constraint specification
design = generate_design_spec()

# 2. Validate
if not validate_problem(design):
    design = fix_design(design)

# 3. Solve
solution = solve_constraints(design)

# 4. Extract results
coordinates = extract_coordinates(solution)

# 5. Export visualization
export_svg(design, "output.svg")
```

### Workflow 2: Parameter Sweep
```python
def parameter_sweep(base_design, param_name, values):
    solutions = []
    for value in values:
        design = base_design.copy()
        design["parameters"][param_name] = value
        try:
            solution = solve_constraints(design)
            solutions.append((value, solution))
        except ValueError:
            continue  # Skip invalid configurations
    return solutions
```

### Workflow 3: Design Validation
```python
def validate_mechanism(link_lengths):
    """Check if mechanism can be assembled."""
    problem = create_linkage_problem(link_lengths)
    return validate_problem(problem) and solve_constraints(problem)["status"] == "ok"
```

## Available Examples

See `examples/` directory for:
- `02_triangle.json` - Basic triangulation
- `08_angles.json` - Angle constraints
- `11_symmetric.json` - Symmetry
- `ai-examples/four_bar_linkage.json` - Kinematic mechanism
- `ai-examples/gear_meshing.json` - Gear positioning

## Further Reading

- [Showcase](SHOWCASE.md) - Impressive use cases
- [Examples](examples/README.md) - Comprehensive tutorial series
- [Schema Reference](schema/slvs-json.schema.json) - Complete schema definition

