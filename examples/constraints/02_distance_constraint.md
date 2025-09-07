# Distance Constraint

The `Distance` constraint sets the distance between two points or between a point and a line.

## What it does

- **Point to Point**: Sets the Euclidean distance between two points
- **Point to Line**: Sets the perpendicular distance from a point to a line
- **Works in 3D**: Calculates true 3D distance when z-coordinates are present

## Example: Diamond Shape with Equal Sides

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "id": "top",
      "type": "Point",
      "x": 50,
      "y": 100
    },
    {
      "id": "right",
      "type": "Point",
      "x": 100,
      "y": 50
    },
    {
      "id": "bottom",
      "type": "Point",
      "x": 50,
      "y": 0
    },
    {
      "id": "left",
      "type": "Point",
      "x": 0,
      "y": 50
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "bottom"
    },
    {
      "type": "Fixed", 
      "entity": "top"
    },
    {
      "type": "Distance",
      "entities": ["top", "right"],
      "distance": 70.711
    },
    {
      "type": "Distance",
      "entities": ["right", "bottom"],
      "distance": 70.711
    },
    {
      "type": "Distance",
      "entities": ["bottom", "left"],
      "distance": 70.711
    },
    {
      "type": "Distance",
      "entities": ["left", "top"],
      "distance": 70.711
    }
  ]
}
```

## Parametric Distance

You can use parameters to make distances adjustable:

```json
{
  "parameters": {
    "side_length": 75
  },
  "constraints": [
    {
      "type": "Distance",
      "entities": ["p1", "p2"],
      "distance": "$side_length"
    }
  ]
}
```

## Key Points

1. **Units matter**: Distance values are in the units specified in the document
2. **Precision**: The solver finds distances to high precision (typically < 0.001 units)
3. **Over-constraining**: Be careful not to specify conflicting distances

## Common Use Cases

- **Mechanical design**: Set link lengths in mechanisms
- **Architecture**: Define room dimensions
- **Manufacturing**: Specify hole spacing and clearances
- **Parametric models**: Use variables for easy dimension changes

## Run the Example

```bash
slvsx solve examples/constraints/02_distance_constraint.json
```

The solver will position the left and right points to form a perfect diamond with all sides equal to 70.711mm (which is 100/√2, forming a square rotated 45°).