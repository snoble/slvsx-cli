# Example 02: Two Points and a Distance

**[← Your First Point](01_first_point.md)** | **[Next: Understanding Constraints →](03_constraints.md)**

## The Story

Now we have a fixed point, let's add another point and control the distance between them. This is like using a compass in technical drawing - you fix one leg and sweep an arc at a specific radius.

But which direction? Without additional constraints, the second point could be anywhere on a sphere around the first point. The solver will find *a* valid solution, but it might surprise you!

## The Constraints

We'll:
1. Create two points: "start" and "end"
2. Fix "start" at the origin
3. Set the distance between them to exactly 100mm

## The JSON

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

## Understanding the Code

- **`parameters`**: We define a named value "length" that we can reuse
- **`$length`**: References the parameter - change it in one place, use it everywhere
- **`between`**: The distance constraint needs two entities
- **Initial positions**: We gave "end" a starting position of [50, 30, 0], but the solver will move it!

## The Solution

The solver finds a position for "end" that's exactly 100mm from "start":

```json
{
  "entities": {
    "start": {
      "at": [0.0, 0.0, 0.0]
    },
    "end": {
      "at": [85.749, 51.450, 0.0]
    }
  }
}
```

Notice the solver kept the same *direction* from our initial guess but adjusted the actual distance to be exactly 100mm.

## Visual Output

![Distance Constraint](02_distance_constraint.svg)

## Key Takeaway

The distance constraint creates a sphere of possible positions. The solver picks one based on:
1. Your initial guess (it tries to stay close to it)
2. Other constraints in the system
3. Minimizing overall changes

Want to control direction too? You'll need additional constraints like fixing coordinates or adding angles.

**[Next: Understanding Constraints →](03_constraints.md)**