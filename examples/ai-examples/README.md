# AI-Friendly Examples for SLVSX

This directory contains example constraint problems designed for AI agents to understand and use the SLVSX constraint solver.

## Quick Usage

```bash
# Solve a constraint problem
slvsx solve triangle_solver.json

# Validate without solving
slvsx validate four_bar_linkage.json

# Export to SVG
slvsx export -f svg gear_meshing.json -o output.svg
```

## Example Files

### triangle_solver.json
Find the position of the third vertex of a triangle given:
- Two fixed vertices (A at origin, B at x=100)  
- Distance from A to C = 80mm
- Distance from B to C = 60mm

**Use Case**: Triangulation, positioning, geometric construction

### four_bar_linkage.json
Validate a four-bar linkage mechanism with:
- Two fixed ground points
- Link lengths specified as distance constraints
- Initial guess positions for moving joints

**Use Case**: Mechanism design, kinematic validation

### gear_meshing.json
Position planetary gears around a sun gear with:
- Fixed sun gear at origin
- Three planet gears at proper meshing distances
- Constraints ensure proper spacing

**Use Case**: Gear train design, mechanical systems

## Common Patterns

### 1. Fixed Points
```json
{
  "type": "fixed",
  "entity": "point_name"
}
```
Use to anchor geometry in space.

### 2. Distance Constraints
```json
{
  "type": "distance",
  "entities": ["point1", "point2"],
  "distance": 100
}
```
Set exact distance between entities.

### 3. Angle Constraints
```json
{
  "type": "angle",
  "entities": ["line1", "line2"],
  "angle": 90
}
```
Set angle between lines (in degrees).

### 4. Parallel/Perpendicular
```json
{
  "type": "parallel",
  "entities": ["line1", "line2"]
}
```
Geometric relationships without specific values.

## Tips for AI Agents

1. **Always provide initial guesses** - The solver needs starting positions
2. **Check degrees of freedom** - Ensure problems aren't over/under-constrained
3. **Use consistent units** - Specify units in the document
4. **Handle errors gracefully** - Check exit codes and parse error messages
5. **Start simple** - Add constraints incrementally when debugging

## Error Codes

- `0` - Success
- `1` - General error
- `2` - Validation error (schema, references)
- `3` - Solver error (over/under-constrained)
- `4` - I/O error

## Python Example

```python
import json
import subprocess

def solve_problem(filename):
    with open(filename) as f:
        problem = json.load(f)
    
    result = subprocess.run(
        ['slvsx', 'solve', '-'],
        input=json.dumps(problem),
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        solution = json.loads(result.stdout)
        print(f"Solved! DOF: {solution['diagnostics']['dof']}")
        return solution
    else:
        print(f"Error: {result.stderr}")
        return None
```

## Further Reading

- [SLVSX JSON Schema](../schema/slvs-json.schema.json)
- [MCP Integration Guide](../../docs/MCP_INTEGRATION.md)
- [All Examples](../README.md)