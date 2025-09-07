# Fixed Constraint

The `Fixed` constraint locks an entity in place, preventing it from moving during solving. This is essential for grounding your constraint system.

## What it does

- **Points**: Fixes the point at its specified x, y, z coordinates
- **Lines**: Fixes both endpoints of the line
- **Circles**: Fixes the center point and radius

## Example: Fixed Triangle Base

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "id": "base_left",
      "type": "Point",
      "x": 0,
      "y": 0
    },
    {
      "id": "base_right", 
      "type": "Point",
      "x": 100,
      "y": 0
    },
    {
      "id": "apex",
      "type": "Point",
      "x": 50,
      "y": 50
    },
    {
      "id": "left_side",
      "type": "Line",
      "points": ["base_left", "apex"]
    },
    {
      "id": "right_side",
      "type": "Line",
      "points": ["base_right", "apex"]
    },
    {
      "id": "base",
      "type": "Line",
      "points": ["base_left", "base_right"]
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "base_left"
    },
    {
      "type": "Fixed",
      "entity": "base_right"
    },
    {
      "type": "Distance",
      "entities": ["base_left", "apex"],
      "distance": 80
    },
    {
      "type": "Distance",
      "entities": ["base_right", "apex"],
      "distance": 80
    }
  ]
}
```

## Key Points

1. **Always fix something**: Without at least one fixed entity, your system can float freely in space
2. **Don't over-constrain**: Fixing too many entities can make the system unsolvable
3. **Strategic placement**: Fix entities that represent your reference frame or ground plane

## Common Use Cases

- **Mechanical linkages**: Fix the ground pivot points
- **Architectural drawings**: Fix the origin or corner points
- **Assembly constraints**: Fix the base component

## Run the Example

```bash
slvsx solve examples/constraints/01_fixed_constraint.json
```

The solver will calculate that the apex must be at approximately (50, 69.28) to satisfy both distance constraints with the fixed base points.