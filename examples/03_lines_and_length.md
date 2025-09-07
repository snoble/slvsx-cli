# Example 03: Creating Lines

**[← Distance Constraint](02_distance_constraint.md)** | **[Next: Building a Triangle →](04_triangle.md)**

## The Story

Points are useful, but most designs need lines. In SLVSX, a line connects two points. The line itself doesn't have a position - it's defined entirely by its endpoints.

This is powerful: move the points, and the line follows automatically. It's like having rubber bands stretched between pins on a board.

## The Entities

We'll create:
1. Two points: "A" and "B"
2. A line connecting them
3. Fix point A and control the line's length

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "A",
      "at": [0, 0, 0]
    },
    {
      "type": "point",
      "id": "B",
      "at": [80, 20, 0]
    },
    {
      "type": "line",
      "id": "AB",
      "p1": "A",
      "p2": "B"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "A"
    },
    {
      "type": "distance",
      "between": ["A", "B"],
      "value": 100.0
    }
  ]
}
```

## Understanding the Code

- **Line entity**: References two points by their IDs
- **No line position**: The line doesn't have an `at` field - it's determined by its endpoints
- **Distance on points**: We constrain the distance between points, not the line itself

## The Solution

```json
{
  "entities": {
    "A": {
      "at": [0.0, 0.0, 0.0]
    },
    "B": {
      "at": [97.014, 24.254, 0.0]
    },
    "AB": {
      "p1": [0.0, 0.0, 0.0],
      "p2": [97.014, 24.254, 0.0]
    }
  }
}
```

The solver:
1. Kept A fixed at the origin
2. Moved B to be exactly 100mm away
3. The line automatically connects the solved positions

## Visual Output

![Line with Length](03_lines_and_length.svg)

## Key Takeaway

Lines are dependent entities - they follow their points. This creates a hierarchy:
- **Points**: The fundamental entities with positions
- **Lines**: Connect points
- **Constraints**: Control how points (and thus lines) can move

This hierarchical thinking is essential for complex sketches.

**[Next: Building a Triangle →](04_triangle.md)**