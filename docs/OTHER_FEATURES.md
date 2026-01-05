# Other SolveSpace Library Features

Beyond constraints, SolveSpace has several other features that could be helpful to users:

## Entity Types Not Yet Fully Exposed

### 1. **Cubic Bezier Curves** (`SLVS_E_CUBIC`)

**What it does**: Defines a cubic Bezier curve with 4 control points.

**Why it's useful**: 
- Smooth curves for organic shapes
- More flexible than arcs for complex curves
- Essential for advanced geometric modeling

**Use case**: 
- Decorative elements (curved edges, organic shapes)
- Smooth transitions between features
- Complex 2D/3D curves

**Current status**: 
- Constraint `CubicLineTangent` implemented
- But `Cubic` entity type not yet exposed in IR

**Implementation needed**:
```json
{
  "type": "cubic",
  "id": "curve1",
  "control_points": ["p1", "p2", "p3", "p4"],
  "workplane": "wp1"  // optional
}
```

### 2. **Proper Arc Entities** (`SLVS_E_ARC_OF_CIRCLE`)

**What it does**: Defines an arc of a circle with center, start point, end point, and normal.

**Why it's useful**: 
- More accurate than simplified circle representation
- Supports arc-specific constraints (arc length ratios, differences)
- Better for circular features

**Current status**: 
- We have `Arc` in IR but it's simplified
- Arc-related constraints implemented but may not work fully without proper arc entities

**Implementation needed**: Full arc entity support with:
- Center point
- Start point
- End point  
- Normal (for 3D arcs)
- Workplane (for 2D arcs)

### 3. **2D Points** (`SLVS_E_POINT_IN_2D`)

**What it does**: Points constrained to a workplane (2D coordinates).

**Why it's useful**: 
- Simplifies 2D sketches within 3D space
- More natural for planar features
- Reduces degrees of freedom automatically

**Use case**: 
- Drawing on faces
- 2D features within planes
- Simplified constraint systems

**Implementation needed**:
```json
{
  "type": "point_2d",
  "id": "p1",
  "at": [u, v],  // 2D coordinates
  "workplane": "wp1"
}
```

### 4. **Face Entities**

**What it does**: Represents a surface/face (for 3D solid modeling).

**Why it's useful**: 
- Enables `PT_ON_FACE` and `PT_FACE_DISTANCE` constraints
- Essential for 3D solid modeling
- Better constraint modeling for surfaces

**Current status**: 
- Constraints `PointOnFace` and `PointFaceDistance` implemented
- But face entity type not yet exposed

**Implementation needed**: Face entity support (likely requires groups/operations)

## Advanced Features

### 5. **Groups** (`hGroup`)

**What it does**: Organizes entities and constraints into logical groups.

**Why it's useful**: 
- Modular design (separate groups for different features)
- Step-and-repeat operations
- Extrude/revolve operations
- Better organization for complex models

**Use case**: 
- Separate groups for base, walls, roof in birdhouse
- Parametric arrays (step and repeat)
- Assembly modeling

**Current status**: Not exposed at all

**Implementation needed**: Group support in IR and solver

### 6. **Construction Geometry**

**What it does**: Entities that don't appear in final output but help with constraints.

**Why it's useful**: 
- Helper geometry for complex constraints
- Cleaner final models
- Better constraint organization

**Use case**: 
- Construction lines for symmetry
- Helper points for positioning
- Temporary geometry for complex constraints

**Current status**: Not exposed

**Implementation needed**: `construction: true` flag on entities

### 7. **Reference Dimensions**

**What it does**: Dimensions that don't constrain the system (for display only).

**Why it's useful**: 
- Show measurements without adding constraints
- Documentation dimensions
- Display-only annotations

**Current status**: Not exposed

**Implementation needed**: `reference: true` flag on constraints

### 8. **Dragged Parameters** (`SLVS_C_WHERE_DRAGGED`)

**What it does**: Marks parameters that should change minimally during solving.

**Why it's useful**: 
- Better solver behavior for interactive editing
- Preserves user intent
- More predictable solving

**Current status**: Not exposed

**Implementation needed**: Support for dragged parameter specification

## Priority Recommendations

### High Priority (Would Significantly Improve Usability)

1. **Proper Arc Entities** - Needed for arc-related constraints to work fully
2. **2D Points** - Simplifies 2D sketches significantly
3. **Construction Geometry** - Very useful for complex models

### Medium Priority (Nice to Have)

4. **Cubic Bezier Curves** - Enables smooth curves
5. **Groups** - Better organization, enables advanced operations
6. **Reference Dimensions** - Better documentation

### Low Priority (Specialized)

7. **Face Entities** - Requires solid modeling support
8. **Dragged Parameters** - More for interactive use

## Implementation Notes

### Arc Entities

Proper arc support requires:
- Center point entity
- Start point entity
- End point entity
- Normal entity (for 3D)
- Workplane (for 2D)

Example:
```json
{
  "type": "arc",
  "id": "arc1",
  "center": "center_point",
  "start": "start_point",
  "end": "end_point",
  "normal": [0, 0, 1],  // or reference to normal entity
  "workplane": "wp1"  // optional, for 2D arcs
}
```

### 2D Points

2D points require:
- Workplane reference
- 2D coordinates (u, v)

Example:
```json
{
  "type": "point_2d",
  "id": "p1",
  "at": [10.0, 20.0],
  "workplane": "front_face"
}
```

### Construction Geometry

Simple flag addition:
```json
{
  "type": "line",
  "id": "helper_line",
  "p1": "p1",
  "p2": "p2",
  "construction": true
}
```

## References

- `libslvs-static/include/slvs.h` - Entity type definitions
- `libslvs-static/src/sketch.h` - Group and entity structures
- `libslvs-static/src/slvs/lib.cpp` - Example entity creation

