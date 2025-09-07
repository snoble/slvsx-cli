# Example 10: Equal Length Lines

**[← Coincident Points](09_coincident.md)** | **[Next: Symmetric Constraints →](11_symmetric.md)**

## The Story

In many designs, you need multiple elements to be the same size - think of table legs, fence posts, or gear teeth. The equal length constraint ensures lines have identical lengths without specifying what that length should be.

Let's create a parallelogram where opposite sides are equal.

## The Entities

We'll create:
1. Four points forming a quadrilateral
2. Four lines as sides
3. Constraints to make it a parallelogram

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
      "at": [120, 50, 0]
    },
    {
      "type": "point",
      "id": "D",
      "at": [20, 50, 0]
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
      "type": "fixed",
      "entity": "B"
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
      "type": "equal_length",
      "a": "AB",
      "b": "CD"
    },
    {
      "type": "equal_length",
      "a": "BC",
      "b": "DA"
    }
  ]
}
```

## Understanding the Code

- **`equal_length`**: Forces lines to have the same length
- **No specified length**: The solver finds a length that satisfies all constraints
- **Parallelogram properties**: Parallel + equal opposite sides

## The Solution

The solver adjusts points to create a perfect parallelogram:

```json
{
  "status": "ok",
  "entities": {
    "A": { "at": [0.0, 0.0, 0.0] },
    "B": { "at": [100.0, 0.0, 0.0] },
    "C": { "at": [120.0, 50.0, 0.0] },
    "D": { "at": [20.0, 50.0, 0.0] }
  }
}
```

Opposite sides are now exactly equal: AB = CD = 100mm, BC = DA ≈ 53.85mm

## Visual Output

![Equal Length](10_equal_length.svg)

## Design Applications

- **Regular polygons**: All sides equal
- **Trusses**: Repeating identical members
- **Furniture**: Multiple legs of same height
- **Architecture**: Uniform spacing

## Key Takeaway

Equal length constraints create relationships between lines without fixing absolute dimensions. This is powerful for creating scalable, proportional designs where relative sizes matter more than absolute measurements.

**[Next: Symmetric Constraints →](11_symmetric.md)**