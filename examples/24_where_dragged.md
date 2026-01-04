# WHERE_DRAGGED Constraint Example

This example demonstrates the `dragged` constraint (WHERE_DRAGGED), which absolutely locks a point to its current position.

## What This Example Shows

- **WHERE_DRAGGED constraint**: `p2` is locked to its current position using the `dragged` constraint
- **Absolute locking**: Unlike `preserve`, this constraint absolutely prevents the point from moving
- **3D point**: The constraint works on 3D points (set `workplane: null`)

## How It Works

The `dragged` constraint creates a `SLVS_C_WHERE_DRAGGED` constraint in SolveSpace, which locks a point to its exact current position. This is more aggressive than the `preserve` flag, which only minimizes changes.

**Use `preserve` when**: You want the solver to prefer keeping an entity stable but allow it to move if necessary.

**Use `dragged` when**: You absolutely need a point to stay exactly where it is.

## Usage

```bash
slvsx solve examples/24_where_dragged.json
```

## 2D Points

For 2D points, specify the workplane:

```json
{
  "type": "dragged",
  "point": "p1",
  "workplane": "xy_plane"
}
```

## Next Steps

- See `examples/23_preserve_flag.json` for softer preservation
- See `examples/25_iterative_refinement.json` for iterative design workflow

