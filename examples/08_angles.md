# Example 08: Working with Angles

**[← Point on Line](https://github.com/snoble/slvsx-cli/blob/main/examples/07_point_on_line.md)** | **[Next: Coincident Points →](https://github.com/snoble/slvsx-cli/blob/main/examples/09_coincident.md)**

## The Story

Angles are fundamental in design - from the 45° miter joint in woodworking to the precise angles in origami. SLVSX can constrain the angle between two lines, letting you create everything from regular polygons to complex mechanical linkages.

Let's build a simple hinge mechanism where we can control the opening angle.

## The Entities

We'll create:
1. Two lines representing hinge arms
2. A shared pivot point
3. An angle constraint between them

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "arm_length": 80.0,
    "hinge_angle": 45.0
  },
  "entities": [
    {
      "type": "point",
      "id": "pivot",
      "at": [0, 0, 0]
    },
    {
      "type": "point",
      "id": "arm1_end",
      "at": [80, 0, 0]
    },
    {
      "type": "point",
      "id": "arm2_end",
      "at": [60, 60, 0]
    },
    {
      "type": "line",
      "id": "arm1",
      "p1": "pivot",
      "p2": "arm1_end"
    },
    {
      "type": "line",
      "id": "arm2",
      "p1": "pivot",
      "p2": "arm2_end"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "pivot"
    },
    {
      "type": "horizontal",
      "a": "arm1"
    },
    {
      "type": "distance",
      "between": ["pivot", "arm1_end"],
      "value": "$arm_length"
    },
    {
      "type": "distance",
      "between": ["pivot", "arm2_end"],
      "value": "$arm_length"
    },
    {
      "type": "angle",
      "between": ["arm1", "arm2"],
      "value": "$hinge_angle"
    }
  ]
}
```

## Understanding the Code

- **`angle` constraint**: Sets the angle between two lines in degrees
- **Shared pivot**: Both arms start from the same point
- **Parametric angle**: Change `hinge_angle` to open/close the hinge

## The Solution

The solver positions the arms at exactly 45°:

```json
{
  "status": "ok",
  "entities": {
    "pivot": { "at": [0.0, 0.0, 0.0] },
    "arm1_end": { "at": [80.0, 0.0, 0.0] },
    "arm2_end": { "at": [56.569, 56.569, 0.0] }
  }
}
```

Notice arm2_end is at (80×cos(45°), 80×sin(45°)) ≈ (56.57, 56.57) - perfect 45° angle!

## Visual Output

![Angle Constraint](https://raw.githubusercontent.com/snoble/slvsx-cli/main/examples/08_angles.svg)

## Common Angles in Design

- **90°**: Right angles for rectangles, frames
- **45°**: Miter joints, diagonal braces
- **120°**: Hexagonal patterns (internal angle)
- **60°**: Equilateral triangles, hexagon sides

## Key Takeaway

Angle constraints define rotational relationships. Combined with distances, they let you create precise angular mechanisms like hinges, scissors, and folding structures.

**[Next: Coincident Points →](https://github.com/snoble/slvsx-cli/blob/main/examples/09_coincident.md)**