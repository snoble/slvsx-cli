# Usage Examples

Real-world examples of using slvsx to solve geometric constraint problems.

## Basic Point Positioning

### Single Fixed Point
```bash
echo '{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {"type": "point", "id": "origin", "at": [0, 0, 0]}
  ],
  "constraints": [
    {"type": "fixed", "entity": "origin"}
  ]
}' | slvsx solve -
```

**Expected Output:**
```json
{
  "status": "ok",
  "diagnostics": {
    "iters": 1,
    "residual": 0.0,
    "dof": 0,
    "time_ms": 1
  },
  "entities": {
    "origin": {
      "at": [0.0, 0.0, 0.0]
    }
  }
}
```

## Distance Constraints

### Two Points with Fixed Distance
```bash
cat << 'EOF' | slvsx solve -
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {"type": "point", "id": "A", "at": [0, 0, 0]},
    {"type": "point", "id": "B", "at": [100, 0, 0]}
  ],
  "constraints": [
    {"type": "fixed", "entity": "A"},
    {"type": "distance", "between": ["A", "B"], "value": 75.0}
  ]
}
EOF
```

**What happens:** Point B moves to maintain exactly 75mm distance from fixed point A.

### Triangle with All Sides Known
```bash
cat << 'EOF' | slvsx solve -
{
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
    {"type": "distance", "between": ["A", "B"], "value": 100},
    {"type": "distance", "between": ["B", "C"], "value": 80},
    {"type": "distance", "between": ["C", "A"], "value": 60}
  ]
}
EOF
```

**What happens:** Point C is positioned to satisfy all three distance constraints.

## Using Parameters

### Parametric Rectangle
```bash
cat << 'EOF' | slvsx solve -
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "width": 150,
    "height": 100
  },
  "entities": [
    {"type": "point", "id": "p1", "at": [0, 0, 0]},
    {"type": "point", "id": "p2", "at": [150, 0, 0]},
    {"type": "point", "id": "p3", "at": [150, 100, 0]},
    {"type": "point", "id": "p4", "at": [0, 100, 0]}
  ],
  "constraints": [
    {"type": "fixed", "entity": "p1"},
    {"type": "distance", "between": ["p1", "p2"], "value": "$width"},
    {"type": "distance", "between": ["p2", "p3"], "value": "$height"},
    {"type": "distance", "between": ["p3", "p4"], "value": "$width"},
    {"type": "distance", "between": ["p4", "p1"], "value": "$height"}
  ]
}
EOF
```

**What happens:** Rectangle dimensions are controlled by parameters. Change `width` or `height` in parameters to resize.

## Lines and Angles

### Perpendicular Lines
```bash
cat << 'EOF' | slvsx solve -
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {"type": "point", "id": "A", "at": [0, 0, 0]},
    {"type": "point", "id": "B", "at": [100, 0, 0]},
    {"type": "point", "id": "C", "at": [100, 50, 0]},
    {"type": "line", "id": "L1", "p1": "A", "p2": "B"},
    {"type": "line", "id": "L2", "p1": "B", "p2": "C"}
  ],
  "constraints": [
    {"type": "fixed", "entity": "A"},
    {"type": "fixed", "entity": "B"},
    {"type": "perpendicular", "entities": ["L1", "L2"]}
  ]
}
EOF
```

## Exporting Results

### Generate SVG Visualization
```bash
# Solve and export to SVG
cat examples/02_distance_constraint.json | slvsx export -f svg - > output.svg

# Or save to file
slvsx export -f svg --output result.svg examples/02_distance_constraint.json
```

### Export to DXF (CAD format)
```bash
slvsx export -f dxf --output design.dxf examples/03_correctly_constrained.json
```

### Export to STL (3D printing)
```bash
slvsx export -f stl --output model.stl examples/04_3d_tetrahedron.json
```

## Common Patterns

### Pattern 1: Fix One Point, Constrain Others
Always start by fixing at least one point to establish a coordinate system:
```json
{"type": "fixed", "entity": "origin"}
```

### Pattern 2: Use Initial Guesses
Provide reasonable initial positions in `at` fields - helps solver converge:
```json
{"type": "point", "id": "target", "at": [50, 50, 0]}  // Good guess
```

### Pattern 3: Check Degrees of Freedom
After solving, check `diagnostics.dof`:
- `dof: 0` = Fully constrained (good!)
- `dof > 0` = Under-constrained (add more constraints)
- `status: "overconstrained"` = Too many constraints (remove some)

### Pattern 4: Validate Before Solving
```bash
# Check if your JSON is valid
slvsx validate my_problem.json

# Then solve
slvsx solve my_problem.json
```

## Error Handling

### Invalid JSON
```bash
echo '{"invalid": json}' | slvsx solve -
```
**Error:** Clear JSON parsing error with line numbers and context.

### Missing Required Fields
```bash
echo '{"entities": []}' | slvsx solve -
```
**Error:** Validation error indicating missing `schema` and `units` fields.

### Over-Constrained System
```bash
# Too many constraints - no solution exists
cat overconstrained.json | slvsx solve -
```
**Status:** `"status": "overconstrained"` with diagnostic information.

### Under-Constrained System
```bash
# Too few constraints - infinite solutions
cat underconstrained.json | slvsx solve -
```
**Status:** `"status": "ok"` but `diagnostics.dof > 0` indicates remaining degrees of freedom.

## Tips for AI Agents

1. **Always validate first** - Use `slvsx validate` before solving
2. **Check diagnostics** - Look at `dof` to ensure proper constraint
3. **Use parameters** - Makes problems easier to modify
4. **Provide good initial guesses** - Helps solver converge faster
5. **Handle errors gracefully** - Over-constrained problems are common

## Next Steps

- See [examples/](../examples/) directory for more complex problems
- Read [JSON Schema](../schema/slvs-json.schema.json) for full specification
- Check [MCP Integration](MCP_INTEGRATION.md) for AI agent usage

