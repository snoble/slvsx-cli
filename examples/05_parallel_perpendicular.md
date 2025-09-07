# Example 05: Parallel and Perpendicular Lines

**[← Building a Triangle](04_triangle.md)** | **[Next: Working with Angles →](06_angles.md)**

## The Story

Parallel and perpendicular lines are everywhere in design - from architectural floor plans to mechanical drawings. These constraints let us specify geometric relationships without worrying about exact positions or angles.

Let's create a simple rectangle using these constraints instead of distances.

## The Entities

We'll create a rectangle using:
1. Four points forming corners
2. Four lines forming sides
3. Parallel and perpendicular constraints

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
      "at": [100, 0, 0]
    },
    {
      "type": "point",
      "id": "C",
      "at": [100, 50, 0]
    },
    {
      "type": "point",
      "id": "D",
      "at": [0, 50, 0]
    },
    {
      "type": "line",
      "id": "AB",
      "p1": "A",
      "p2": "B"
    },
    {
      "type": "line",
      "id": "BC",
      "p1": "B",
      "p2": "C"
    },
    {
      "type": "line",
      "id": "CD",
      "p1": "C",
      "p2": "D"
    },
    {
      "type": "line",
      "id": "DA",
      "p1": "D",
      "p2": "A"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "A"
    },
    {
      "type": "horizontal",
      "a": "AB"
    },
    {
      "type": "vertical",
      "a": "DA"
    },
    {
      "type": "parallel",
      "a": "AB",
      "b": "CD"
    },
    {
      "type": "parallel",
      "a": "BC",
      "b": "DA"
    },
    {
      "type": "distance",
      "between": ["A", "B"],
      "value": 100
    },
    {
      "type": "distance",
      "between": ["A", "D"],
      "value": 50
    }
  ]
}
```

## Understanding the Code

- **`horizontal` and `vertical`**: Special cases of directional constraints
- **`parallel`**: Forces two lines to have the same direction (but not position)
- **Combining constraints**: We use parallel + distance to fully define the rectangle

## The Solution

The solver creates a perfect rectangle:

```json
{
  "status": "ok",
  "entities": {
    "A": { "at": [0.0, 0.0, 0.0] },
    "B": { "at": [100.0, 0.0, 0.0] },
    "C": { "at": [100.0, 50.0, 0.0] },
    "D": { "at": [0.0, 50.0, 0.0] }
  }
}
```

## Visual Output

![Parallel and Perpendicular](05_parallel_perpendicular.svg)

## Alternative: Using Perpendicular

We could replace the parallel constraints with perpendicular:

```json
{
  "type": "perpendicular",
  "a": "AB",
  "b": "BC"
}
```

This says "these lines meet at 90 degrees" - perfect for corners!

## Key Takeaway

Geometric constraints (parallel, perpendicular, horizontal, vertical) define *relationships* not *positions*. Combine them with distance or fixed constraints to fully define your sketch.

**[Next: Working with Angles →](06_angles.md)**