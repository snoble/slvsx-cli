# Angle Constraint

The `Angle` constraint sets the angle between two lines or the absolute angle of a line relative to the horizontal.

## What it does

- **Between two lines**: Sets the angle at their intersection point
- **Single line**: Sets the angle relative to the positive X-axis
- **Angles in degrees**: All angles are specified in degrees (0-360)

## Example: Adjustable Scissor Mechanism

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "scissor_angle": 30
  },
  "entities": [
    {
      "id": "pivot",
      "type": "Point",
      "x": 0,
      "y": 0
    },
    {
      "id": "blade1_end",
      "type": "Point",
      "x": 100,
      "y": 0
    },
    {
      "id": "blade2_end",
      "type": "Point",
      "x": 86.6,
      "y": 50
    },
    {
      "id": "blade1",
      "type": "Line",
      "points": ["pivot", "blade1_end"]
    },
    {
      "id": "blade2",
      "type": "Line",
      "points": ["pivot", "blade2_end"]
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "pivot"
    },
    {
      "type": "Distance",
      "entities": ["pivot", "blade1_end"],
      "distance": 100
    },
    {
      "type": "Distance",
      "entities": ["pivot", "blade2_end"],
      "distance": 100
    },
    {
      "type": "Angle",
      "entities": ["blade1", "blade2"],
      "angle": "$scissor_angle"
    }
  ]
}
```

## Absolute Angle Example

Set a line to a specific angle from horizontal:

```json
{
  "entities": [
    {
      "id": "origin",
      "type": "Point",
      "x": 0,
      "y": 0
    },
    {
      "id": "end",
      "type": "Point",
      "x": 100,
      "y": 100
    },
    {
      "id": "angled_line",
      "type": "Line",
      "points": ["origin", "end"]
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "origin"
    },
    {
      "type": "Distance",
      "entities": ["origin", "end"],
      "distance": 100
    },
    {
      "type": "Angle",
      "entities": ["angled_line"],
      "angle": 45
    }
  ]
}
```

## Key Points

1. **Degrees not radians**: Always specify angles in degrees
2. **Direction matters**: Angles are measured counter-clockwise from the first line to the second
3. **Range**: Angles wrap around (370° = 10°)
4. **Complementary constraints**: Often combined with distance constraints

## Common Use Cases

- **Mechanical linkages**: Set crank angles and joint positions
- **Roof trusses**: Define pitch angles
- **Folding mechanisms**: Control deployment angles
- **Robot arms**: Specify joint angles

## Run the Example

```bash
slvsx solve examples/constraints/03_angle_constraint.json
```

Try changing the `scissor_angle` parameter to see how the mechanism adjusts!