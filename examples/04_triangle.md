# Example 04: Building a Triangle

**[← Creating Lines](03_lines_and_length.md)** | **[Next: Parallel and Perpendicular →](05_parallel_perpendicular.md)**

## The Story

A triangle is the simplest rigid structure in engineering. Three sides with fixed lengths can only form one shape (up to rotation and mirroring). This makes triangles fundamental in trusses, bridges, and mechanical linkages.

Let's build an equilateral triangle - all sides equal length.

## The Entities

We'll create:
1. Three points: A, B, and C
2. Three lines forming the triangle
3. Constraints to make all sides equal

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "side_length": 60.0
  },
  "entities": [
    {
      "type": "point",
      "id": "A",
      "at": [0, 0, 0]
    },
    {
      "type": "point",
      "id": "B",
      "at": [60, 0, 0]
    },
    {
      "type": "point",
      "id": "C",
      "at": [30, 50, 0]
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
      "id": "CA",
      "p1": "C",
      "p2": "A"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "A"
    },
    {
      "type": "fixed",
      "entity": "B"
    },
    {
      "type": "distance",
      "between": ["B", "C"],
      "value": "$side_length"
    },
    {
      "type": "distance",
      "between": ["C", "A"],
      "value": "$side_length"
    }
  ]
}
```

## Understanding the Code

- **Two fixed points**: We fix both A and B to establish the base of our triangle
- **Two distance constraints**: With the base fixed, we only need to constrain the other two sides
- **Parametric design**: Change `side_length` and the whole triangle scales

## The Solution

Running through SLVSX:

```bash
slvsx solve 04_triangle.json
```

```json
{
  "status": "ok",
  "entities": {
    "A": { "at": [0.0, 0.0, 0.0] },
    "B": { "at": [60.0, 0.0, 0.0] },
    "C": { "at": [30.0, 51.961, 0.0] }
  }
}
```

Point C moved to form a perfect equilateral triangle! The height is exactly `60 * sin(60°) ≈ 51.96`.

## Visual Output

![Triangle](04_triangle.svg)

## Key Takeaway

Notice we only needed 4 constraints for 3 points (9 degrees of freedom):
- 2 fixed constraints = 6 DOF removed
- 2 distance constraints = 2 DOF removed
- 1 DOF remains (the triangle could flip below the base)

The solver chose the solution closest to our initial guess.

**[Next: Parallel and Perpendicular →](05_parallel_perpendicular.md)**