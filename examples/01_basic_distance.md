# Basic Distance Constraint

This example demonstrates the fundamental distance constraint in SLVSX. It shows how to constrain the distance between two points to a specific value using parameters.

## Problem

Create two points connected by a line, where:
- One point is fixed at the origin
- The second point must be exactly 100mm away from the first
- The initial position of the second point doesn't matter - the solver will find a valid position

## Solution

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "length": 100.0
  },
  "entities": [
    {
      "type": "point",
      "id": "start",
      "at": [0, 0, 0]
    },
    {
      "type": "point", 
      "id": "end",
      "at": [50, 30, 0]
    },
    {
      "type": "line",
      "id": "connector",
      "p1": "start",
      "p2": "end"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "start"
    },
    {
      "type": "distance",
      "between": ["start", "end"],
      "value": "$length"
    }
  ]
}
```

## Usage

```bash
# Solve the constraint system
slvsx solve examples/01_basic_distance.json

# Export visualization
slvsx export examples/01_basic_distance.json --format svg --output basic_distance.svg
```

## Result

The solver automatically moves the "end" point to position `(85.75, 51.45, 0)`, which is exactly 100mm from the origin, while preserving the direction from the initial guess position.

![Basic Distance](outputs/01_basic_distance.svg)

## Key Concepts

- **Fixed constraint**: Prevents a point from moving during solving
- **Distance constraint**: Enforces exact distance between two points
- **Parameters**: Use `$parameter_name` to reference values defined in the parameters section
- **Initial position**: The solver uses initial positions as hints but will move entities to satisfy constraints

This forms the foundation for more complex constraint systems where multiple distance constraints can define precise geometric relationships.