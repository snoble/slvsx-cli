# Perpendicular and Parallel Constraints

These constraints control the relative orientation of lines without specifying exact angles.

## Perpendicular Constraint

Forces two lines to meet at exactly 90 degrees.

### Example: Rectangle Construction

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "id": "p1",
      "type": "Point",
      "x": 0,
      "y": 0
    },
    {
      "id": "p2",
      "type": "Point",
      "x": 150,
      "y": 0
    },
    {
      "id": "p3",
      "type": "Point",
      "x": 150,
      "y": 100
    },
    {
      "id": "p4",
      "type": "Point",
      "x": 0,
      "y": 100
    },
    {
      "id": "bottom",
      "type": "Line",
      "points": ["p1", "p2"]
    },
    {
      "id": "right",
      "type": "Line",
      "points": ["p2", "p3"]
    },
    {
      "id": "top",
      "type": "Line",
      "points": ["p3", "p4"]
    },
    {
      "id": "left",
      "type": "Line",
      "points": ["p4", "p1"]
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "p1"
    },
    {
      "type": "Fixed",
      "entity": "p2"
    },
    {
      "type": "Perpendicular",
      "entities": ["bottom", "right"]
    },
    {
      "type": "Perpendicular",
      "entities": ["right", "top"]
    },
    {
      "type": "Perpendicular",
      "entities": ["top", "left"]
    },
    {
      "type": "Perpendicular",
      "entities": ["left", "bottom"]
    }
  ]
}
```

## Parallel Constraint

Forces two lines to have the same direction (never intersect).

### Example: Parallelogram

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "id": "p1",
      "type": "Point",
      "x": 0,
      "y": 0
    },
    {
      "id": "p2",
      "type": "Point",
      "x": 100,
      "y": 0
    },
    {
      "id": "p3",
      "type": "Point",
      "x": 130,
      "y": 80
    },
    {
      "id": "p4",
      "type": "Point",
      "x": 30,
      "y": 80
    },
    {
      "id": "bottom",
      "type": "Line",
      "points": ["p1", "p2"]
    },
    {
      "id": "right",
      "type": "Line",
      "points": ["p2", "p3"]
    },
    {
      "id": "top",
      "type": "Line",
      "points": ["p3", "p4"]
    },
    {
      "id": "left",
      "type": "Line",
      "points": ["p4", "p1"]
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "p1"
    },
    {
      "type": "Fixed",
      "entity": "p2"
    },
    {
      "type": "Parallel",
      "entities": ["bottom", "top"]
    },
    {
      "type": "Parallel",
      "entities": ["left", "right"]
    },
    {
      "type": "Distance",
      "entities": ["p1", "p2"],
      "distance": 100
    },
    {
      "type": "Distance",
      "entities": ["p3", "p4"],
      "distance": 100
    }
  ]
}
```

## Key Points

1. **No angle needed**: These constraints automatically enforce 90° or 0° relative angles
2. **Structural stability**: Perfect for creating rectangles, squares, and grid structures
3. **Combine with distance**: Add distance constraints to fully define dimensions

## Common Use Cases

- **Architectural floor plans**: Ensure walls meet at right angles
- **Machine frames**: Create rectangular support structures
- **PCB layouts**: Align traces and components
- **Parallel guides**: Railway tracks, conveyor systems

## Run the Examples

```bash
# Rectangle with perpendicular sides
slvsx solve examples/constraints/04_perpendicular.json

# Parallelogram with parallel sides
slvsx solve examples/constraints/04_parallel.json
```