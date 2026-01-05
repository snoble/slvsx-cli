# Iterative Design Best Practices

Building complex constraint problems iteratively is the key to success with SLVSX. This guide shows you how to start simple and gradually add complexity.

## Why Iterative Design?

Constraint solvers work best when you:
1. **Start with a working foundation** - Get something simple working first
2. **Add complexity incrementally** - Test each addition before moving on
3. **Debug systematically** - When something breaks, you know exactly what changed

## The Iterative Process

### Step 1: Start with Fixed Points

Always begin by fixing reference points. These anchor your geometry in space.

```json
{
  "entities": [
    {"type": "point", "id": "origin", "at": [0, 0, 0]}
  ],
  "constraints": [
    {"type": "fixed", "entity": "origin"}
  ]
}
```

**Why:** Without fixed points, your geometry has infinite degrees of freedom and can't be solved.

### Step 2: Add Basic Structure

Add the core entities (points, lines) that define your shape's skeleton.

```json
{
  "entities": [
    {"type": "point", "id": "origin", "at": [0, 0, 0]},
    {"type": "point", "id": "corner1", "at": [100, 0, 0]},
    {"type": "point", "id": "corner2", "at": [0, 100, 0]},
    {"type": "line", "id": "edge1", "p1": "origin", "p2": "corner1"},
    {"type": "line", "id": "edge2", "p1": "origin", "p2": "corner2"}
  ],
  "constraints": [
    {"type": "fixed", "entity": "origin"},
    {"type": "distance", "between": ["origin", "corner1"], "value": 100},
    {"type": "distance", "between": ["origin", "corner2"], "value": 100}
  ]
}
```

**Test:** Verify this solves before adding more.

### Step 3: Add Relationships

Once basic structure works, add geometric relationships (parallel, perpendicular, angles).

```json
{
  "constraints": [
    // ... existing constraints ...
    {"type": "perpendicular", "a": "edge1", "b": "edge2"}
  ]
}
```

**Why:** Relationships constrain geometry without fixing exact positions.

### Step 4: Add Parameters

Make dimensions parametric so you can easily adjust them.

```json
{
  "parameters": {
    "width": 100.0,
    "height": 100.0
  },
  "constraints": [
    {"type": "distance", "between": ["origin", "corner1"], "value": "$width"},
    {"type": "distance", "between": ["origin", "corner2"], "value": "$height"}
  ]
}
```

**Benefit:** Easy to explore design variations.

### Step 5: Add Complexity Gradually

Add features one at a time, testing after each addition:
- Additional points/lines
- Circles and arcs
- More complex constraints
- 3D geometry

## Real Example: Building a Birdhouse

Let's see how we built the birdhouse example iteratively:

### Iteration 1: Basic Box

Start with just the base rectangle:

```json
{
  "entities": [
    {"type": "point", "id": "base_front_left", "at": [0, 0, 0]},
    {"type": "point", "id": "base_front_right", "at": [150, 0, 0]},
    {"type": "point", "id": "base_back_left", "at": [0, 120, 0]},
    {"type": "point", "id": "base_back_right", "at": [150, 120, 0]}
  ],
  "constraints": [
    {"type": "fixed", "entity": "base_front_left"},
    {"type": "distance", "between": ["base_front_left", "base_front_right"], "value": 150},
    {"type": "distance", "between": ["base_front_left", "base_back_left"], "value": 120}
  ]
}
```

**Status:** ✅ Works (DOF: 0)

### Iteration 2: Add Walls

Add vertical walls by adding top points and connecting them:

```json
{
  "entities": [
    // ... base points ...
    {"type": "point", "id": "top_front_left", "at": [0, 0, 200]},
    {"type": "point", "id": "top_front_right", "at": [150, 0, 200]},
    {"type": "point", "id": "top_back_left", "at": [0, 120, 200]},
    {"type": "point", "id": "top_back_right", "at": [150, 120, 200]},
    {"type": "line", "id": "front_left_edge", "p1": "base_front_left", "p2": "top_front_left"}
    // ... more edges ...
  ],
  "constraints": [
    // ... existing constraints ...
    {"type": "distance", "between": ["base_front_left", "top_front_left"], "value": 200}
  ]
}
```

**Status:** ✅ Still works

### Iteration 3: Add Roof

Add the roof peak and roof edges:

```json
{
  "entities": [
    // ... existing entities ...
    {"type": "point", "id": "roof_peak", "at": [75, 0, 250]},
    {"type": "line", "id": "roof_front_left", "p1": "top_front_left", "p2": "roof_peak"}
    // ... more roof edges ...
  ],
  "constraints": [
    // ... existing constraints ...
    {"type": "coincident", "at": "roof_peak", "of": ["top_front"]},
    {"type": "distance", "between": ["top_front_left", "roof_peak"], "value": 50},
    {"type": "equal_length", "entities": ["roof_front_left", "roof_front_right", "roof_back_left", "roof_back_right"]}
  ]
}
```

**Status:** ✅ Complete birdhouse

## Common Pitfalls and Solutions

### Problem: "System is inconsistent"

**Cause:** Conflicting constraints (e.g., two different distances between same points)

**Solution:** 
- Remove redundant constraints
- Check for contradictions
- Use `equal_length` instead of multiple `distance` constraints when appropriate

### Problem: "Solver did not converge"

**Cause:** Poor initial guesses or over-constrained system

**Solution:**
- Provide better initial positions in `at` fields
- Remove unnecessary constraints
- Simplify the problem, then add complexity back

### Problem: "Too many unknowns"

**Cause:** Under-constrained system

**Solution:**
- Add more constraints
- Fix more reference points
- Use geometric relationships (parallel, perpendicular)

### Problem: Constraint seems to have no effect

**Cause:** Constraint is redundant or conflicts with existing constraints

**Solution:**
- Remove the constraint and see if solution changes
- Check if constraint is already implied by others
- Verify constraint references correct entities

## Best Practices Checklist

- [ ] Start with fixed reference points
- [ ] Add entities one at a time
- [ ] Test after each addition
- [ ] Use parameters for dimensions
- [ ] Fix one thing at a time when debugging
- [ ] Keep initial guesses reasonable
- [ ] Use appropriate constraint types
- [ ] Document your design decisions

## Debugging Workflow

When something breaks:

1. **Identify the last working version** - What was the last iteration that worked?
2. **Isolate the problem** - What did you add that broke it?
3. **Simplify** - Remove the problematic addition
4. **Add back carefully** - Add it back with different approach
5. **Test incrementally** - Verify each small change

## Example: Debugging Over-Constrained System

```bash
# Start with working version
slvsx solve birdhouse_step2.json  # ✅ Works

# Add roof
slvsx solve birdhouse_step3.json  # ❌ Error: System is inconsistent

# What changed? Added roof constraints. Let's simplify:
# Remove equal_length constraint, keep just distance
slvsx solve birdhouse_step3_simple.json  # ✅ Works

# Now add equal_length back
slvsx solve birdhouse_step3.json  # ✅ Works!
```

## Advanced: Building Mechanisms

For mechanisms (linkages, etc.), the process is similar but you want to preserve degrees of freedom:

1. **Fix the frame** - Ground points/links
2. **Add links** - One at a time with distance constraints
3. **Add joints** - Coincident constraints at connection points
4. **Test motion** - Verify DOF > 0 for mechanisms
5. **Add input** - Angle or distance constraints for actuation

See [examples/22_slider_crank.json](../examples/22_slider_crank.json) for a complete mechanism example.

## Summary

**The Golden Rule:** Build incrementally, test frequently, debug systematically.

Every complex constraint problem is just a series of simple problems stacked together. By building iteratively, you:
- Catch errors early
- Understand what each constraint does
- Create maintainable, understandable designs
- Learn the solver's behavior

Start simple, add complexity gradually, and you'll build robust constraint systems every time.

