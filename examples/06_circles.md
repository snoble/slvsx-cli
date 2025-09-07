# Example 06: Working with Circles

**[← Parallel and Perpendicular](05_parallel_perpendicular.md)** | **[Next: Point on Line →](07_point_on_line.md)**

## The Story

Circles appear everywhere in mechanical design - gears, bearings, wheels, and pulleys. In SLVSX, a circle is defined by its center point and diameter. 

Let's create two circles and make them tangent (just touching) - a common requirement in gear design.

## The Entities

We'll create:
1. Two circles with their centers
2. Control their sizes and relative positions

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "circle1_diameter": 40.0,
    "circle2_diameter": 60.0
  },
  "entities": [
    {
      "type": "point",
      "id": "center1",
      "at": [0, 0, 0]
    },
    {
      "type": "point",
      "id": "center2",
      "at": [60, 0, 0]
    },
    {
      "type": "circle",
      "id": "circle1",
      "center": [0, 0, 0],
      "diameter": "$circle1_diameter"
    },
    {
      "type": "circle",
      "id": "circle2",
      "center": [60, 0, 0],
      "diameter": "$circle2_diameter"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "center1"
    },
    {
      "type": "distance",
      "between": ["center1", "center2"],
      "value": 50.0,
      "_comment": "Sum of radii: 20 + 30 = 50 for tangent circles"
    }
  ]
}
```

## Understanding the Code

- **Circle entity**: Needs a center point and diameter
- **Tangent circles**: Distance between centers = sum of radii
- **`_comment`**: JSON allows underscore fields for documentation

## The Solution

Running through SLVSX:

```bash
slvsx solve 06_circles.json
```

```json
{
  "status": "ok",
  "entities": {
    "center1": { "at": [0.0, 0.0, 0.0] },
    "center2": { "at": [50.0, 0.0, 0.0] },
    "circle1": {
      "center": [0.0, 0.0, 0.0],
      "diameter": 40.0
    },
    "circle2": {
      "center": [50.0, 0.0, 0.0],
      "diameter": 60.0
    }
  }
}
```

The circles are perfectly tangent - touching at exactly one point!

## Visual Output

![Circles](06_circles.svg)

## Variations

### Concentric Circles
Make circles share the same center:
```json
{
  "type": "coincident",
  "entities": ["center1", "center2"]
}
```

### Equal Radius
Force circles to be the same size:
```json
{
  "type": "equal_radius",
  "entities": ["circle1", "circle2"]
}
```

## Key Takeaway

Circles introduce curved geometry. The solver handles them just like points and lines - define the entities, add constraints, and let the math work out the positions.

**[Next: Point on Line →](07_point_on_line.md)**