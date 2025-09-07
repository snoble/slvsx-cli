# Example 07: Point on Line Constraint

**[← Working with Circles](https://github.com/snoble/slvsx-cli/blob/main/examples/06_circles.md)** | **[Next: Symmetric Constraints →](https://github.com/snoble/slvsx-cli/blob/main/examples/08_symmetric.md)**

## The Story

Sometimes you know a point should lie somewhere along a line, but you don't know exactly where. Think of a slider on a rail, or a bead on a wire. The point can move along the line but can't leave it.

This constraint is powerful for mechanisms where parts slide along guides.

## The Entities

We'll create:
1. A line to act as a guide rail
2. A point constrained to slide along it
3. Another point to "pull" the slider

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "rail_start",
      "at": [0, 0, 0]
    },
    {
      "type": "point",
      "id": "rail_end",
      "at": [100, 0, 0]
    },
    {
      "type": "line",
      "id": "rail",
      "p1": "rail_start",
      "p2": "rail_end"
    },
    {
      "type": "point",
      "id": "slider",
      "at": [30, 5, 0]
    },
    {
      "type": "point",
      "id": "target",
      "at": [40, 50, 0]
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "rail_start"
    },
    {
      "type": "fixed",
      "entity": "rail_end"
    },
    {
      "type": "fixed",
      "entity": "target"
    },
    {
      "type": "point_on_line",
      "point": "slider",
      "line": "rail"
    },
    {
      "type": "distance",
      "between": ["slider", "target"],
      "value": 30
    }
  ]
}
```

## Understanding the Code

- **`point_on_line`**: Forces the point to lie somewhere along the infinite line
- **Slider position**: The solver finds where along the rail the slider should be
- **Distance to target**: Pulls the slider to a specific position along the rail

## The Solution

The solver finds the optimal position along the rail:

```json
{
  "status": "ok",
  "entities": {
    "slider": { "at": [40.0, 0.0, 0.0] },
    "rail": {
      "p1": [0.0, 0.0, 0.0],
      "p2": [100.0, 0.0, 0.0]
    }
  }
}
```

The slider moved to x=40 on the rail - directly below the target point, maintaining the 30mm distance!

## Visual Output

![Point on Line](https://raw.githubusercontent.com/snoble/slvsx-cli/main/examples/07_point_on_line.svg)

## Real-World Applications

- **Linear bearings**: Parts that slide along rails
- **Cam followers**: Points tracking along cam profiles  
- **Projected dimensions**: Finding the projection of a point onto a line

## Key Takeaway

The `point_on_line` constraint adds one degree of freedom - the point can slide along the line but can't leave it. Combine with other constraints to fully define the position.

**[Next: Symmetric Constraints →](https://github.com/snoble/slvsx-cli/blob/main/examples/08_symmetric.md)**