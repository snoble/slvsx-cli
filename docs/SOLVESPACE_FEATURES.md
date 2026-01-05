# SolveSpace Features Not Yet Exposed

This document lists SolveSpace library features that could make constraint modeling easier but aren't yet exposed in SLVSX.

## High-Value Features for Complex Models

### 1. **Point-in-Plane Constraint** (`SLVS_C_PT_IN_PLANE`)

**What it does**: Constrains a point to lie in a plane.

**Why it's useful**: Currently, to constrain a point to a face (like the front face of the birdhouse), we have to use `point_on_line` with an edge, which only constrains 2D. `PT_IN_PLANE` would constrain the point to the entire plane.

**Use case**: Positioning the entrance hole on the front face of the birdhouse - instead of:
```python
# Current workaround - constrains to edge, then distances
constraints.append({"type": "point_on_line", "point": "entrance_center", "line": "front_left_edge"})
constraints.append({"type": "distance", "between": ["base_front_left", "entrance_center"], "value": "$entrance_height"})
constraints.append({"type": "distance", "between": ["top_front_left", "entrance_center"], "value": 76.0})
```

We could do:
```python
# With PT_IN_PLANE - much cleaner!
constraints.append({"type": "point_in_plane", "point": "entrance_center", "plane": "front_face"})
constraints.append({"type": "distance", "between": ["base_front_left", "entrance_center"], "value": "$entrance_height"})
constraints.append({"type": "distance", "between": ["top_front_left", "entrance_center"], "value": 76.0})
```

**Implementation**: Requires workplane support (see below).

### 2. **Point-on-Face Constraint** (`SLVS_C_PT_ON_FACE`)

**What it does**: Constrains a point to lie on a face (surface).

**Why it's useful**: Similar to `PT_IN_PLANE` but for curved surfaces or specific faces.

**Use case**: Positioning features on specific faces of 3D objects.

### 3. **Workplanes** (`SLVS_E_WORKPLANE`)

**What it does**: Defines a 2D coordinate system (plane) for constraining geometry.

**Why it's useful**: 
- Makes plane-based constraints possible (`PT_IN_PLANE`, `PT_ON_FACE`)
- Simplifies 2D sketches within 3D space
- Enables more natural constraint modeling

**Use case**: 
- Define the front face of the birdhouse as a workplane
- Constrain entrance hole to that workplane
- Much cleaner than current edge-based approach

**Example**:
```python
# Create workplane for front face
add_workplane("front_face", origin="base_front_left", normal=[0, 1, 0])

# Constrain point to workplane
constraints.append({"type": "point_in_plane", "point": "entrance_center", "plane": "front_face"})
```

### 4. **Point-to-Plane Distance** (`SLVS_C_PT_PLANE_DISTANCE`)

**What it does**: Constrains the distance from a point to a plane.

**Why it's useful**: Easier than calculating 3D distance and constraining it.

**Use case**: Positioning features at specific distances from faces.

### 5. **Point-to-Line Distance** (`SLVS_C_PT_LINE_DISTANCE`)

**What it does**: Constrains the distance from a point to a line (perpendicular distance).

**Why it's useful**: For positioning features relative to edges.

**Use case**: Positioning ventilation holes at specific distances from edges.

### 6. **Projected Point Distance** (`SLVS_C_PROJ_PT_DISTANCE`)

**What it does**: Distance between points projected onto a plane.

**Why it's useful**: For 2D measurements within a plane.

**Use case**: Measuring distances in a specific view/plane.

## Additional Constraints

### 7. **Length Ratio** (`SLVS_C_LENGTH_RATIO`)

**What it does**: Constrains the ratio between two lengths.

**Why it's useful**: For proportional designs (e.g., "this edge is 1.5x longer than that edge").

**Use case**: Architectural proportions, golden ratio designs.

### 8. **Equal Angle** (`SLVS_C_EQUAL_ANGLE`)

**What it does**: Makes two angles equal.

**Why it's useful**: For symmetric angular features.

**Use case**: Roof angles, decorative elements.

### 9. **Symmetric Horizontal/Vertical** (`SLVS_C_SYMMETRIC_HORIZ`, `SLVS_C_SYMMETRIC_VERT`)

**What it does**: Symmetry about horizontal or vertical axes (simpler than `SYMMETRIC_LINE`).

**Why it's useful**: Common case that's easier than general symmetry.

**Use case**: Symmetric roofs, symmetric features.

### 10. **Diameter Constraint** (`SLVS_C_DIAMETER`)

**What it does**: Constrains circle diameter directly.

**Why it's useful**: More direct than radius constraints.

**Use case**: Entrance holes, decorative circles.

### 11. **Same Orientation** (`SLVS_C_SAME_ORIENTATION`)

**What it does**: Constrains entities to have the same orientation.

**Why it's useful**: For parallel planes, aligned features.

**Use case**: Ensuring faces are parallel, features are aligned.

## Entity Types Not Exposed

### 12. **2D Points** (`SLVS_E_POINT_IN_2D`)

**What it does**: Points constrained to a workplane (2D coordinates).

**Why it's useful**: Simplifies 2D sketches within 3D space.

**Use case**: Drawing on faces, 2D features.

### 13. **Normals** (`SLVS_E_NORMAL_IN_3D`, `SLVS_E_NORMAL_IN_2D`)

**What it does**: Defines direction vectors (for workplanes, etc.).

**Why it's useful**: Required for workplanes and plane-based constraints.

**Use case**: Defining workplane orientations.

## Priority Recommendations

For making the birdhouse (and similar projects) easier:

1. **HIGH PRIORITY**: 
   - **Workplanes** - Foundation for plane-based constraints
   - **PT_IN_PLANE** - Solves entrance hole positioning elegantly
   - **PT_PLANE_DISTANCE** - Useful for feature positioning

2. **MEDIUM PRIORITY**:
   - **PT_LINE_DISTANCE** - Useful for edge-relative positioning
   - **LENGTH_RATIO** - For proportional designs
   - **SYMMETRIC_HORIZ/VERT** - Common symmetry cases

3. **LOW PRIORITY** (nice to have):
   - **EQUAL_ANGLE** - Can be done with angle constraints
   - **DIAMETER** - Can use radius
   - **SAME_ORIENTATION** - Can use parallel/perpendicular

## Implementation Notes

### Workplanes

Workplanes require:
- Normal entity (direction vector)
- Origin point
- Group assignment

Example C code:
```c
// Create normal
Slvs_Entity normal = Slvs_MakeNormal3D(normal_id, group, qw, qx, qy, qz);

// Create workplane
Slvs_Entity wp = Slvs_MakeWorkplane(wp_id, group, origin_point, normal);
```

### PT_IN_PLANE Constraint

Requires:
- Point entity
- Workplane entity
- Group

Example C code:
```c
Slvs_MakeConstraint(
    constraint_id, group, SLVS_C_PT_IN_PLANE, workplane,
    0, point, 0, 0, 0
);
```

## Current Status

**Already Available**:
- ✅ `Plane` entity type exists in IR (`crates/core/src/ir.rs`)
- ❌ Plane entities are not yet processed in the solver
- ❌ Plane-based constraints (`PT_IN_PLANE`, `PT_PLANE_DISTANCE`) not implemented

**What's Missing**:
- FFI bindings for workplane creation
- FFI bindings for `PT_IN_PLANE` constraint
- FFI bindings for `PT_PLANE_DISTANCE` constraint
- Solver processing for Plane entities
- Constraint registry support for plane-based constraints

## Birdhouse Example: Before vs After

### Current Approach (Complex)
```python
# Constrain entrance hole to front face - requires multiple constraints
constraints.append({"type": "point_on_line", "point": "entrance_center", "line": "front_left_edge"})
constraints.append({"type": "distance", "between": ["base_front_left", "entrance_center"], "value": "$entrance_height"})
constraints.append({"type": "distance", "between": ["top_front_left", "entrance_center"], "value": 76.0})
# Problem: Overconstrained if not careful, hard to reason about
```

### With PT_IN_PLANE (Simple)
```python
# Define front face as a plane
entities.append({
    "type": "plane",
    "id": "front_face",
    "origin": [0, 0, 0],  # base_front_left
    "normal": [0, 1, 0]   # Y-axis normal (front face is Y=0 plane)
})

# Constrain entrance hole to front face - one constraint!
constraints.append({"type": "point_in_plane", "point": "entrance_center", "plane": "front_face"})
# Then just position within the plane
constraints.append({"type": "distance", "between": ["base_front_left", "entrance_center"], "value": "$entrance_height"})
constraints.append({"type": "distance", "between": ["top_front_left", "entrance_center"], "value": 76.0})
# Much cleaner and less error-prone!
```

## Implementation Roadmap

### Phase 1: Plane Entity Support
1. Add FFI binding for creating workplanes from Plane entities
2. Process Plane entities in solver
3. Map Plane to SolveSpace workplane

### Phase 2: Plane-Based Constraints
1. Add `PT_IN_PLANE` constraint to IR
2. Add FFI binding for `PT_IN_PLANE`
3. Add constraint registry support
4. Add tests

### Phase 3: Distance Constraints
1. Add `PT_PLANE_DISTANCE` constraint
2. Add `PT_LINE_DISTANCE` constraint
3. Implement FFI bindings
4. Update constraint registry

## References

- `libslvs-static/include/slvs.h` - Complete API reference
- `libslvs-static/src/slvs/lib.cpp` - Example implementations
- `crates/core/src/ir.rs` - Current IR definitions (Plane already exists!)
- SolveSpace documentation (DOC.txt in libslvs-static)

