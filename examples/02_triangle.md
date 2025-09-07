# Equilateral Triangle

This example demonstrates creating an equilateral triangle using distance constraints. The solver automatically positions points to satisfy multiple distance requirements simultaneously.

## Problem

Create an equilateral triangle where all three sides are exactly 60mm long:
- Point A is fixed at the origin  
- Points B and C can move to satisfy the constraints
- All three sides (AB, BC, CA) must be equal length

## Solution

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
      "at": [100, 0, 0]
    },
    {
      "type": "point",
      "id": "C",
      "at": [50, 50, 0]
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
      "type": "distance",
      "between": ["A", "B"],
      "value": "$side_length"
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

## Usage

```bash
# Solve the constraint system
slvsx solve examples/02_triangle.json

# Export visualization  
slvsx export examples/02_triangle.json --format svg --output triangle.svg
```

## Result

The solver positions:
- Point A at `(0, 0, 0)` (fixed)
- Point B at `(59.68, -6.16, 0)`  
- Point C at `(35.18, 48.60, 0)`

All distances are exactly 60mm, forming a perfect equilateral triangle.

![Equilateral Triangle](outputs/02_triangle.svg)

## Key Concepts

- **Multiple distance constraints**: Three distance constraints working together
- **Over-constraint avoidance**: Only one point is fixed to avoid over-constraining the system
- **Simultaneous solving**: The solver finds a solution that satisfies all constraints at once
- **Initial positions as hints**: The solver uses the initial point positions as starting points but moves them as needed

This example shows how geometric constraints can automatically create precise shapes without manual calculation of angles or coordinates.