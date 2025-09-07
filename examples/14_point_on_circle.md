# Example 14: Point on Circle

**[← Horizontal & Vertical](https://github.com/snoble/slvsx-cli/blob/main/examples/13_horizontal_vertical.md)** | **[Next: Equal Radius →](https://github.com/snoble/slvsx-cli/blob/main/examples/15_equal_radius.md)**

## The Story

Circular motion is fundamental in mechanisms - from clock hands to planetary gears. The point-on-circle constraint restricts a point to move along a circular path, perfect for modeling rotating parts or circular guides.

Let's create points constrained to a circular path, like beads on a circular wire.

## The Entities

We'll create:
1. A circle to act as the path
2. Multiple points constrained to move along it
3. Lines from center to show the radial connections

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "center",
      "at": [50, 50, 0]
    },
    {
      "type": "circle",
      "id": "path",
      "center": [50, 50, 0],
      "diameter": 80
    },
    {
      "type": "point",
      "id": "p1",
      "at": [90, 50, 0]
    },
    {
      "type": "point",
      "id": "p2",
      "at": [50, 90, 0]
    },
    {
      "type": "point",
      "id": "p3",
      "at": [10, 50, 0]
    },
    {
      "type": "line",
      "id": "radius1",
      "p1": "center",
      "p2": "p1"
    },
    {
      "type": "line",
      "id": "radius2",
      "p1": "center",
      "p2": "p2"
    },
    {
      "type": "line",
      "id": "radius3",
      "p1": "center",
      "p2": "p3"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "center"
    },
    {
      "type": "fixed",
      "entity": "path"
    },
    {
      "type": "point_on_circle",
      "point": "p1",
      "circle": "path"
    },
    {
      "type": "point_on_circle",
      "point": "p2",
      "circle": "path"
    },
    {
      "type": "point_on_circle",
      "point": "p3",
      "circle": "path"
    }
  ]
}
```

## Understanding the Code

- **`point_on_circle`**: Constrains a point to lie on circle's circumference
- **Circle definition**: Needs center coordinates and diameter
- **Freedom of movement**: Points can slide along the circle
- **Radial lines**: Automatically adjust as points move

## The Solution

```json
{
  "status": "ok",
  "diagnostics": {
    "iters": 1,
    "residual": 0.0,
    "dof": 0
  }
}
```

The solver ensures all points lie exactly on the circle's circumference!

## Mechanism Applications

- **Crank mechanisms**: Connecting rod attachments
- **Planetary gears**: Planet positions around sun
- **Cam followers**: Points tracking cam profiles
- **Clock mechanisms**: Hour/minute hand pivots

## Combining with Other Constraints

Point-on-circle works well with:
- **Angle constraints**: Fix angular positions
- **Distance constraints**: Set spacing between points
- **Tangent constraints**: For rolling contact

## Visual Output

![Point on Circle](https://raw.githubusercontent.com/snoble/slvsx-cli/main/examples/14_point_on_circle.svg)

## Key Takeaway

Point-on-circle constraints enable circular motion paths while maintaining other geometric relationships. They're essential for any rotating or orbital mechanism design.

**[Next: Equal Radius →](https://github.com/snoble/slvsx-cli/blob/main/examples/15_equal_radius.md)**