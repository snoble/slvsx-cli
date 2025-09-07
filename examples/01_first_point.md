# Example 01: Your First Point

**[← Introduction](00_introduction.md)** | **[Next: Two Points and a Distance →](02_distance_constraint.md)**

## The Story

Every journey starts with a single step - or in our case, a single point. But here's the thing about constraint solvers: a point floating in space has infinite possible positions. We need to "fix" it somewhere to give our sketch a starting reference.

Think of it like pinning a blueprint to a drafting table. Without that first pin, the whole drawing could slide around.

## The Constraints

We'll create:
1. A single point named "origin"
2. Fix it at coordinates (0, 0, 0)

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "origin",
      "at": [0, 0, 0]
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "origin"
    }
  ]
}
```

## Understanding the Code

- **`entities`**: We define a point with an ID we can reference
- **`at`**: Initial position [x, y, z] - but this is just a starting guess!
- **`constraints`**: The `fixed` constraint tells the solver "this point cannot move"

## The Solution

When we run this through SLVSX:

```bash
slvsx solve 01_first_point.json
```

The solver confirms our point stays exactly where we fixed it:

```json
{
  "entities": {
    "origin": {
      "at": [0.0, 0.0, 0.0]
    }
  }
}
```

## Visual Output

![First Point](01_first_point.svg)

A simple dot at the origin - but it's the foundation for everything we'll build!

## Key Takeaway

The `fixed` constraint is crucial. Without it, the solver has no reference point and can place entities anywhere that satisfies the other constraints. Always anchor at least one point in your sketch.

**[Next: Two Points and a Distance →](02_distance_constraint.md)**