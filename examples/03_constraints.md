# Understanding Constraint Systems: Over-constrained vs Properly Constrained

This example demonstrates the difference between properly constrained and over-constrained systems, helping you understand how to avoid common constraint mistakes.

## Over-Constrained System (❌ FAILS)

### The Problem
```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "length1": 100.0,
    "length2": 80.0
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
      "type": "line",
      "id": "AB",
      "p1": "A",
      "p2": "B"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "A"
    },
    {
      "type": "fixed",
      "entity": "B"  
    },
    {
      "type": "distance",
      "between": ["A", "B"],
      "value": "$length1"
    },
    {
      "type": "distance", 
      "between": ["A", "B"],
      "value": "$length2"
    }
  ]
}
```

### Why It Fails
This system has **conflicting constraints**:
1. Both points A and B are **fixed** at their initial positions
2. The distance between them is initially 100mm
3. We're asking for the distance to be both 100mm AND 80mm simultaneously
4. **Impossible!** The points can't move (they're fixed) but also can't satisfy both distance requirements

### Testing the Over-Constrained System
```bash
slvsx solve examples/03_overconstrained.json
# Result: Error: FFI error: Solver failed with code 1
```

## Properly Constrained System (✅ WORKS)

### The Solution
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
      "id": "A",
      "at": [0, 0, 0]
    },
    {
      "type": "point",
      "id": "B", 
      "at": [80, 20, 0]
    },
    {
      "type": "line",
      "id": "AB",
      "p1": "A",
      "p2": "B"
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
      "value": "$length"
    }
  ]
}
```

### Why It Works
This system has **consistent constraints**:
1. Point A is fixed at the origin
2. Point B can move freely
3. The distance between A and B must be exactly 100mm
4. **Solvable!** Point B can be positioned anywhere on a circle of radius 100mm centered at A

### Testing the Properly Constrained System
```bash  
slvsx solve examples/03_correctly_constrained.json
# Result: Point B moves to (97.01, 24.25, 0) - exactly 100mm from A
```

## Constraint Analysis

### Degrees of Freedom (DOF)
- **2D Point**: 2 DOF (can move in X and Y)
- **Fixed constraint**: Removes 2 DOF  
- **Distance constraint**: Removes 1 DOF

### Over-Constrained System Analysis
- Point A: 2 DOF - 2 (fixed) = 0 DOF
- Point B: 2 DOF - 2 (fixed) = 0 DOF  
- Total: 0 DOF available
- Distance constraints: 2 constraints trying to remove 2 DOF
- **Problem**: No freedom to adjust, but conflicting requirements

### Properly Constrained System Analysis
- Point A: 2 DOF - 2 (fixed) = 0 DOF
- Point B: 2 DOF - 1 (distance) = 1 DOF  
- Total: 1 DOF (Point B can rotate around A at fixed distance)
- **Perfect**: System is **exactly constrained**

## Common Over-Constraint Patterns

1. **Fixing too many points**: Don't fix both endpoints of a line if you also constrain the line's length
2. **Redundant constraints**: Don't apply multiple distance constraints to the same point pair
3. **Conflicting requirements**: Don't ask for impossible combinations (like a triangle with sides 10, 10, and 100)

## Visual Output

![Properly Constrained System](https://raw.githubusercontent.com/snoble/slvsx-cli/main/examples/03_constraints.svg)

*This shows the properly constrained system. The over-constrained system cannot be visualized as it fails to solve.*

## Key Lessons

✅ **Good Practice**: Use the minimum constraints needed to define your geometry  
✅ **Good Practice**: Fix one point to establish a reference frame  
✅ **Good Practice**: Let the solver position other points to satisfy constraints  

❌ **Avoid**: Fixing multiple points when you also constrain distances between them  
❌ **Avoid**: Applying multiple conflicting constraints to the same entities  
❌ **Avoid**: Over-defining systems - less is often more  

The constraint solver is powerful, but it needs geometric freedom to find solutions!