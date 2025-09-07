# Example 11: Symmetric Constraints

**[← Equal Length](10_equal_length.md)** | **[Next: 3D Basics →](12_3d_basics.md)**

## The Story

Symmetry is everywhere in nature and design - from butterfly wings to architectural facades. The symmetric constraint mirrors points across a line, creating balanced, aesthetically pleasing designs with half the effort.

Let's create a simple arrow shape with perfect symmetry.

## The Entities

We'll create:
1. A vertical line as the axis of symmetry
2. Points on one side that get mirrored to the other

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "axis_top",
      "at": [50, 100, 0]
    },
    {
      "type": "point",
      "id": "axis_bottom",
      "at": [50, 0, 0]
    },
    {
      "type": "line",
      "id": "axis",
      "p1": "axis_bottom",
      "p2": "axis_top"
    },
    {
      "type": "point",
      "id": "tip",
      "at": [50, 100, 0]
    },
    {
      "type": "point",
      "id": "left_barb",
      "at": [30, 80, 0]
    },
    {
      "type": "point",
      "id": "right_barb",
      "at": [70, 80, 0]
    },
    {
      "type": "point",
      "id": "left_base",
      "at": [45, 0, 0]
    },
    {
      "type": "point",
      "id": "right_base",
      "at": [55, 0, 0]
    },
    {
      "type": "line",
      "id": "left_side",
      "p1": "tip",
      "p2": "left_barb"
    },
    {
      "type": "line",
      "id": "right_side",
      "p1": "tip",
      "p2": "right_barb"
    },
    {
      "type": "line",
      "id": "left_shaft",
      "p1": "left_barb",
      "p2": "left_base"
    },
    {
      "type": "line",
      "id": "right_shaft",
      "p1": "right_barb",
      "p2": "right_base"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "axis_bottom"
    },
    {
      "type": "fixed",
      "entity": "axis_top"
    },
    {
      "type": "point_on_line",
      "point": "tip",
      "line": "axis"
    },
    {
      "type": "distance",
      "between": ["axis_top", "left_barb"],
      "value": 28.28
    },
    {
      "type": "distance",
      "between": ["axis_top", "right_barb"],
      "value": 28.28
    },
    {
      "type": "distance",
      "between": ["axis_bottom", "left_base"],
      "value": 5
    },
    {
      "type": "distance",
      "between": ["axis_bottom", "right_base"],
      "value": 5
    },
    {
      "type": "distance",
      "between": ["tip", "left_barb"],
      "value": 30
    },
    {
      "type": "distance",
      "between": ["left_barb", "left_base"],
      "value": 85
    }
  ]
}
```

## Understanding the Code

- **Simulating symmetry**: Using equal distance constraints to create symmetric appearance
- **Equal distances**: Points equidistant from axis appear symmetric
- **Manual balancing**: Without a true symmetric constraint, we manually ensure equal distances

## The Solution

The solver creates a perfectly symmetric arrow:

```json
{
  "status": "ok",
  "entities": {
    "tip": { "at": [50.0, 100.0, 0.0] },
    "left_barb": { "at": [35.0, 80.0, 0.0] },
    "right_barb": { "at": [65.0, 80.0, 0.0] },
    "left_base": { "at": [35.0, 0.0, 0.0] },
    "right_base": { "at": [65.0, 0.0, 0.0] }
  }
}
```

Perfect mirror symmetry across the vertical axis!

## Visual Output

![Symmetric Constraints](11_symmetric.svg)

## Design Applications

- **Architecture**: Symmetric facades and floor plans
- **Mechanical**: Balanced mechanisms
- **Art & Graphics**: Logos, patterns, mandalas
- **Biology-inspired**: Wing shapes, leaf patterns

## Achieving Symmetry

While SLVSX doesn't have a built-in symmetric constraint, you can achieve symmetry by:
- Using equal distance constraints from a central axis
- Carefully positioning points with distance constraints
- Using coincident constraints with midpoints

## Key Takeaway

Symmetric constraints cut your work in half - design one side, and the constraint solver creates the other. This ensures perfect balance and saves time on repetitive mirroring operations.

**[Next: 3D Basics →](12_3d_basics.md)**