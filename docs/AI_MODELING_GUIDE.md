# AI Modeling Guide for SLVSX

This guide documents common pitfalls and best practices discovered while using SLVSX for geometric constraint modeling. **Read this before attempting to create models.**

## Critical Pitfalls

### 1. Circles Can Track Points (NEW!)

Circle entities can now reference a point entity for their center:

```json
// NEW FEATURE - Circle tracks a moving point!
{
  "type": "point",
  "id": "pivot",
  "at": [50, 50, 0]
},
{
  "type": "circle",
  "id": "bearing",
  "center": "pivot",  // Reference to point entity
  "diameter": 20
}

// You can still use coordinates for fixed circles
{
  "type": "circle",
  "id": "fixed_circle",
  "center": [100, 100, 0],  // Fixed coordinates
  "diameter": 30
}
```

**Best Practice**: For mechanism parts that move (gears, bearings, cam followers), reference a point entity. For fixed decorative elements, use coordinates.

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
| `horizontal` | `a: line`, `workplane: plane` (2D only!) |
| `vertical` | `a: line`, `workplane: plane` (2D only!) |
| `symmetric` | `a: point`, `b: point`, `about: line` (**NOT supported in 3D!**) |
| `symmetric_horizontal` | `a: point`, `b: point`, `workplane: plane` |
| `symmetric_vertical` | `a: point`, `b: point`, `workplane: plane` |

## 2D Geometry Setup (Required for horizontal/vertical/symmetric)

Many constraints only work with 2D geometry in a workplane. Here's the complete setup pattern:

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    // Step 1: Create a workplane (required first!)
    {
      "type": "plane",
      "id": "xy_plane",
      "origin": [0, 0, 0],
      "normal": [0, 0, 1]
    },
    // Step 2: Create 2D points IN the workplane
    {
      "type": "point2_d",
      "id": "p1",
      "at": [0, 0],
      "workplane": "xy_plane"
    },
    {
      "type": "point2_d",
      "id": "p2",
      "at": [100, 0],
      "workplane": "xy_plane"
    },
    // Step 3: Create 2D lines between 2D points
    {
      "type": "line2_d",
      "id": "line1",
      "p1": "p1",
      "p2": "p2",
      "workplane": "xy_plane"
    }
  ],
  "constraints": [
    // Step 4: Now horizontal/vertical work!
    {
      "type": "horizontal",
      "a": "line1",
      "workplane": "xy_plane"
    },
    {
      "type": "fixed",
      "entity": "p1",
      "workplane": "xy_plane"
    }
  ]
}
```

**Key Points:**
- Use `plane` to define a workplane first
- Use `point2_d` (NOT `point`) for 2D points
- Use `line2_d` (NOT `line`) for 2D lines
- All 2D entities need `"workplane": "plane_id"`
- Horizontal/vertical/symmetric constraints need `"workplane": "plane_id"`

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

### 5. Tangent Constraints Don't Work with Circles

**Problem**: The `tangent` constraint only works with `arc`, `cubic`, and `line` entities. **It does NOT work with `circle` entities** - attempting to use it will cause an error.

```json
// BAD - circle cannot be tangent
{
  "type": "tangent",
  "a": "my_circle",    // circle entity - NOT SUPPORTED!
  "b": "my_line"
}

// GOOD - use an Arc instead
{
  "type": "arc",
  "id": "my_arc",
  "center": "center_point",
  "start": "arc_start",
  "end": "arc_end",
  "normal": [0, 0, 1],
  "workplane": "xy_plane"
}
// Then tangent constraint works:
{
  "type": "tangent",
  "a": "my_arc",
  "b": "my_line"
}
```

**Why**: In SolveSpace, circles are static (center + radius) while arcs are parametric curves that can be constrained. For tangent behavior, use an `arc` entity.

### 6. Symmetric Constraint Requires 2D Geometry

**Problem**: The `symmetric` constraint (about a line) only works with 2D geometry in a workplane. Using it with 3D points will crash.

**Common Mistakes:**
- Using `axis` instead of `about` - the correct field is `about`
- Using 3D points instead of 2D points
- Forgetting the workplane reference

```json
// BAD - wrong field name and 3D points
{
  "type": "symmetric",
  "a": "point1",      // 3D point - CRASHES!
  "b": "point2",
  "axis": "line"      // WRONG! Should be "about"
}

// GOOD - use symmetric_horizontal or symmetric_vertical with a workplane
{
  "type": "symmetric_horizontal",
  "a": "p1",          // Reference to point (3D or 2D)
  "b": "p2",
  "workplane": "xy_plane"
}

// ALTERNATIVE - symmetric_vertical
{
  "type": "symmetric_vertical",
  "a": "p1",
  "b": "p2", 
  "workplane": "xy_plane"
}
```

**Why**: SolveSpace's `SYMMETRIC_LINE` constraint requires a workplane context. For most symmetry needs, use `symmetric_horizontal` or `symmetric_vertical` with an explicit workplane.

### 8. Entity ID Case Sensitivity

Entity IDs are case-sensitive:
```json
// These are DIFFERENT entities
"id": "Point1"
"id": "point1"  // Different from Point1!
```

### 9. Parameter References Must Use $ Prefix

```json
// BAD
"value": "width"

// GOOD  
"value": "$width"
```

### 10. Entity Type Names Use snake_case

```json
// BAD
"type": "Point2D"
"type": "point_2d"

// GOOD
"type": "point2_d"  // Note the underscore placement
"type": "line2_d"
```

### 11. Fixed Constraint for 2D Points Needs Workplane

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

### 12. Angle Constraint Behavior

The angle constraint measures the angle between two lines. The angle value is in degrees.

**Important**: 
- Avoid degenerate angles (0° or 180°) as these can cause solver convergence failures
- Start with geometry that differs from your target angle (e.g., initial 45° if target is 90°)
- Angle is measured as the acute angle between lines

### 13. Equal Length with Many Entities

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

### 14. Circle Orientation (Normal Vector)

**Problem**: Circles lie in a plane defined by their `normal` vector. If not specified, circles default to the XY plane (normal = [0,0,1]). Circles on walls or other planes **must** specify their normal or they will project incorrectly in different views.

```json
// BAD - circle on front wall uses default XY normal
{
  "type": "circle",
  "id": "entrance_hole",
  "center": [75, 0, 120],
  "diameter": 40
  // Missing normal! Defaults to [0,0,1] which is WRONG for a wall at Y=0
}

// GOOD - specify normal for wall-mounted circle
{
  "type": "circle",
  "id": "entrance_hole", 
  "center": [75, 0, 120],
  "diameter": 40,
  "normal": [0, 1, 0]  // Points along Y-axis (perpendicular to front wall)
}
```

**How normals affect projection**:
- Circle with `normal: [0,0,1]` (XY plane) appears as circle in XY view, line in XZ/YZ views
- Circle with `normal: [0,1,0]` (XZ plane) appears as circle in XZ view, line in XY/YZ views
- Circle with `normal: [1,0,0]` (YZ plane) appears as circle in YZ view, line in XY/XZ views

**For cylindrical features (like dowels/perches)**:
```python
# Calculate normal from dowel direction
dx, dy, dz = end.x - start.x, end.y - start.y, end.z - start.z
length = math.sqrt(dx*dx + dy*dy + dz*dz)
normal = (dx/length, dy/length, dz/length)  # Points along cylinder axis
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

### 15. Solution Files vs Input Files

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

