# AI Modeling Guide for SLVSX

This guide documents common pitfalls and best practices discovered while using SLVSX for geometric constraint modeling. **Read this before attempting to create models.**

## Critical Pitfalls

### 1. Circles Don't Track Points

**Problem**: Circle entities have a `center` field that takes coordinates, NOT a reference to a point entity. This means circles stay at their initial position even after solving.

```json
// BAD - circle won't move when "pivot" moves
{
  "type": "circle",
  "id": "bearing",
  "center": [50, 50, 0],  // These are fixed coordinates!
  "diameter": 20
}

// WORKAROUND - use a point entity to track the position
{
  "type": "point",
  "id": "pivot",
  "at": [50, 50, 0]
}
// The circle will NOT follow pivot - this is a known limitation
```

**Best Practice**: For moving parts, use points and lines. Only use circles for fixed decorative elements or reference geometry.

### 2. Constraint Field Names Vary

Different constraints use different field names. Check the schema carefully:

| Constraint | Fields |
|------------|--------|
| `distance` | `between: [point1, point2]`, `value` |
| `perpendicular` | `a: line1`, `b: line2` (NOT `entities`) |
| `parallel` | `entities: [line1, line2]` |
| `angle` | `between: [line1, line2]`, `value` |
| `midpoint` | `point`, `of: line` (NOT `line`) |
| `equal_length` | `entities: [line1, line2, ...]` |
| `point_on_line` | `point`, `line` |
| `coincident` | `entities: [point1, point2]` |
| `fixed` | `entity`, `workplane` (optional, for 2D points) |

### 3. Horizontal/Vertical Require 2D Geometry

**Problem**: `horizontal` and `vertical` constraints only work with 2D lines in a workplane. They will fail on 3D lines.

```json
// BAD - horizontal on 3D line fails
{
  "type": "horizontal",
  "entity": "some_3d_line",
  "workplane": "xy_plane"  // Even with workplane, 3D lines fail!
}

// GOOD - use 2D geometry
{
  "type": "plane",
  "id": "xy_plane", 
  "origin": [0, 0, 0],
  "normal": [0, 0, 1]
},
{
  "type": "point2_d",  // Note: point2_d, not point_2d
  "id": "p1",
  "at": [0, 0],
  "workplane": "xy_plane"
},
{
  "type": "line2_d",
  "id": "horizontal_line",
  "p1": "p1",
  "p2": "p2", 
  "workplane": "xy_plane"
},
{
  "type": "horizontal",
  "entity": "horizontal_line",
  "workplane": "xy_plane"
}
```

**Best Practice**: For 3D models, use `perpendicular` and `parallel` constraints instead of horizontal/vertical.

### 4. Over-Constraining Causes Failures

**Problem**: Adding too many constraints causes "Invalid solver system" or "Overconstrained" errors.

```json
// BAD - redundant constraints
{
  "type": "fixed",
  "entity": "point_a"
},
{
  "type": "distance",
  "between": ["point_a", "point_a"],  // Distance to itself!
  "value": 0
}

// BAD - conflicting constraints
{
  "type": "distance",
  "between": ["p1", "p2"],
  "value": 100
},
{
  "type": "distance", 
  "between": ["p1", "p2"],
  "value": 150  // Conflicts with above!
}
```

**Best Practice**: 
- Start with minimal constraints
- Add one constraint at a time and verify the solve works
- Fixed points don't need additional position constraints
- Count degrees of freedom: each 3D point has 3 DOF, each constraint removes DOF

### 5. Entity ID Case Sensitivity

Entity IDs are case-sensitive:
```json
// These are DIFFERENT entities
"id": "Point1"
"id": "point1"  // Different from Point1!
```

### 6. Parameter References Must Use $ Prefix

```json
// BAD
"value": "width"

// GOOD  
"value": "$width"
```

### 7. Entity Type Names Use snake_case

```json
// BAD
"type": "Point2D"
"type": "point_2d"

// GOOD
"type": "point2_d"  // Note the underscore placement
"type": "line2_d"
```

### 8. Fixed Constraint for 2D Points Needs Workplane

```json
// BAD - 2D point will drift without workplane
{
  "type": "fixed",
  "entity": "my_2d_point"
}

// GOOD
{
  "type": "fixed",
  "entity": "my_2d_point",
  "workplane": "xy_plane"
}
```

### 9. Angle Constraint Behavior

The angle constraint measures the angle between two lines. The angle value is in degrees.

**Important**: 
- Avoid degenerate angles (0° or 180°) as these can cause solver convergence failures
- Start with geometry that differs from your target angle (e.g., initial 45° if target is 90°)
- Angle is measured as the acute angle between lines

### 10. Equal Length with Many Entities

Using `equal_length` with many entities (>4) can cause internal ID collisions. Split into multiple constraints:

```json
// RISKY with many entities
{
  "type": "equal_length",
  "entities": ["line1", "line2", "line3", "line4", "line5"]
}

// SAFER - chain constraints
{
  "type": "equal_length",
  "entities": ["line1", "line2"]
},
{
  "type": "equal_length", 
  "entities": ["line2", "line3"]
}
```

## Recommended Workflow

### Step 1: Start Simple
Begin with just a few fixed points and basic geometry:

```json
{
  "entities": [
    {"type": "point", "id": "origin", "at": [0, 0, 0]},
    {"type": "point", "id": "p1", "at": [100, 0, 0]}
  ],
  "constraints": [
    {"type": "fixed", "entity": "origin"}
  ]
}
```

### Step 2: Add Geometry Incrementally
Add one entity at a time and verify it solves:

```bash
slvsx solve my_model.json
```

### Step 3: Add Constraints One at a Time
Add constraints incrementally. If the solver fails, the last constraint is the problem.

### Step 4: Use Parameters for Dimensions
Define all dimensions as parameters for easy modification:

```json
{
  "parameters": {
    "width": 100,
    "height": 50,
    "depth": 75
  }
}
```

### Step 5: Test Different Views
Export in different views to verify 3D structure:

```bash
slvsx export model.json -v xy -o front.svg
slvsx export model.json -v xz -o side.svg
slvsx export model.json -v isometric -o perspective.svg
```

## Common Error Messages

| Error | Likely Cause |
|-------|--------------|
| `Invalid solver system` | Contradictory constraints or impossible geometry |
| `Overconstrained` | Too many constraints for the DOF |
| `Handle isn't unique` | Internal ID collision - simplify model |
| `missing field 'xyz'` | Wrong field name for constraint type |
| `Unexpected horizontal/vertical constraint in 3d` | Using H/V on 3D lines - use 2D geometry |
| `invalid type: map, expected a sequence` | Trying to parse a solution file as input |
| `Solver failed to converge` | Degenerate geometry (0° or 180° angles) |

### 11. Solution Files vs Input Files

**Problem**: Solution output files (`*_solution.json`) have a different format than input files. Don't try to parse them as input.

```json
// INPUT file - entities is an ARRAY
{
  "entities": [
    {"type": "point", "id": "p1", "at": [0, 0, 0]}
  ]
}

// SOLUTION file - entities is a MAP
{
  "entities": {
    "p1": {"at": [0.0, 0.0, 0.0]}
  }
}
```

## Construction Geometry

Mark helper entities as construction geometry so they don't appear in final renders:

```json
{
  "type": "line",
  "id": "centerline",
  "p1": "p1",
  "p2": "p2",
  "construction": true  // Won't render in output
}
```

**Note**: Construction geometry filtering is not yet implemented in the SVG exporter.

## Example: Simple Mechanism

Here's a working slider-crank mechanism that avoids all pitfalls:

```json
{
  "schema": "slvs-json/1",
  "parameters": {
    "crank_radius": 50,
    "rod_length": 150,
    "crank_angle": 45
  },
  "entities": [
    {"type": "point", "id": "crank_center", "at": [0, 0, 0]},
    {"type": "point", "id": "crank_pin", "at": [35, 35, 0]},
    {"type": "point", "id": "piston", "at": [160, 0, 0]},
    {"type": "point", "id": "guide_left", "at": [-50, 0, 0]},
    {"type": "point", "id": "guide_right", "at": [250, 0, 0]},
    {"type": "line", "id": "crank", "p1": "crank_center", "p2": "crank_pin"},
    {"type": "line", "id": "rod", "p1": "crank_pin", "p2": "piston"},
    {"type": "line", "id": "guide", "p1": "guide_left", "p2": "guide_right"}
  ],
  "constraints": [
    {"type": "fixed", "entity": "crank_center"},
    {"type": "fixed", "entity": "guide_left"},
    {"type": "fixed", "entity": "guide_right"},
    {"type": "distance", "between": ["crank_center", "crank_pin"], "value": "$crank_radius"},
    {"type": "distance", "between": ["crank_pin", "piston"], "value": "$rod_length"},
    {"type": "point_on_line", "point": "piston", "line": "guide"},
    {"type": "angle", "between": ["crank", "guide"], "value": "$crank_angle"}
  ]
}
```

## Debugging Tips

1. **Simplify**: Remove constraints until it works, then add back one at a time
2. **Check output**: Look at solved positions to see what moved unexpectedly
3. **Use fixed points**: Anchor your model with at least one fixed point
4. **Verify geometry**: Export to SVG and visually inspect
5. **Check DOF**: 3 DOF per 3D point, 2 DOF per 2D point. Constraints remove DOF.

