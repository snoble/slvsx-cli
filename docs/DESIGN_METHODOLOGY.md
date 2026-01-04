# Design Methodology for High-Quality Constraint-Based Models

This document outlines the methodology for creating high-quality geometric constraint models in SLVSX. This process was developed through building the birdhouse example and can be applied to any complex design project.

## Core Principles

1. **Iterative Development**: Build incrementally, verify at each step
2. **Visual Verification**: Check renders frequently, not just solver success
3. **Programmatic Generation**: Use code to generate JSON, not hand-writing
4. **Reference Research**: Study real-world examples for proportions and features
5. **Constraint Debugging**: Understand constraint conflicts systematically
6. **Progressive Refinement**: Fix one issue at a time

## The Process

### Phase 1: Research and Planning

**Goal**: Understand what you're building and gather reference materials.

1. **Research Real-World Examples**
   - Find photos, plans, or specifications
   - Note key dimensions and proportions
   - Identify critical features
   - Document design requirements

2. **Plan the Constraint Strategy**
   - Identify fixed reference points
   - Plan entity hierarchy (base → structure → details)
   - List required constraints by category
   - Anticipate potential constraint conflicts

3. **Set Up Development Environment**
   - Create a programmatic generator (Python/JavaScript/etc.)
   - Set up render verification workflow
   - Create test script to solve and render automatically

### Phase 2: Foundation (Simple → Complex)

**Goal**: Build a solid, well-constrained base before adding details.

1. **Start with Fixed Points**
   - Define base reference points
   - Use `fixed` constraints for anchors
   - Verify solver can handle the base

2. **Add Basic Structure**
   - Build primary geometry (box, frame, etc.)
   - Use distance constraints for dimensions
   - Use horizontal/vertical for alignment
   - Verify each addition solves successfully

3. **Verify Basic Shape**
   - Generate renders (XY, XZ, YZ, Isometric)
   - Check proportions visually
   - Ensure geometry looks correct
   - Fix any obvious issues before proceeding

### Phase 3: Feature Addition

**Goal**: Add features one at a time, verifying each works.

1. **Add Features Incrementally**
   - One feature per iteration
   - Solve and render after each addition
   - Fix issues before moving on

2. **Use Appropriate Constraints**
   - `point_on_line` for features on edges
   - `coincident` for alignment
   - `distance` for positioning
   - `equal_length` for symmetry

3. **Handle Constraint Conflicts**
   - If overconstrained: Remove redundant constraints
   - If underconstrained: Add missing constraints
   - If invalid system: Check entity references
   - Document what works and why

### Phase 4: Refinement

**Goal**: Polish the design to production quality.

1. **Perfect Proportions**
   - Compare to reference materials
   - Adjust parameters iteratively
   - Verify all dimensions make sense

2. **Add Finishing Details**
   - Decorative elements
   - Functional features (vents, drainage, etc.)
   - Ensure details don't break main structure

3. **Final Verification**
   - Solve successfully
   - Render all views
   - Check for visual quality
   - Verify parameters work correctly

## Tools and Techniques

### Programmatic Generation

**Why**: Hand-writing JSON is error-prone and hard to iterate.

**How**:
- Use Python, JavaScript, or any language that can generate JSON
- Create helper functions for common patterns
- Use loops for repetitive structures
- Use variables/parameters for dimensions

**Example**:
```python
def add_point(id_name, x, y, z):
    entities.append({
        "type": "point",
        "id": id_name,
        "at": [x, y, z]
    })

# Use loops for repetitive structures
for i in range(4):
    add_point(f"corner_{i}", x[i], y[i], z[i])
```

### Visual Verification Workflow

**Why**: Solver success doesn't guarantee visual correctness.

**How**:
1. Solve the model
2. Generate renders for all views (XY, XZ, YZ, Isometric)
3. Visually inspect each render
4. Compare to reference materials
5. Fix issues before proceeding

**Automation**:
```bash
# Create a script that solves and renders
./solve_and_render.sh examples/birdhouse.json
```

### Constraint Debugging Strategy

**When solver fails**:

1. **Overconstrained**: Remove redundant constraints
   - Check for duplicate distance constraints
   - Remove constraints implied by others (e.g., if vertical + horizontal → parallel)
   - Simplify constraint approach

2. **Underconstrained**: Add missing constraints
   - Check DOF in solver output
   - Add constraints to fix free variables
   - Use fixed points strategically

3. **Invalid System**: Check entity references
   - Verify all entity IDs exist
   - Check constraint entity types match
   - Ensure entities are defined before use

4. **Didn't Converge**: Adjust initial guesses
   - Check if constraints are physically possible
   - Verify parameter values are reasonable
   - Try different constraint approaches

### Progressive Constraint Addition

**Strategy**: Add constraints in logical groups, verify each group.

1. **Foundation Constraints**
   - Fixed points
   - Base dimensions
   - Verify: Basic shape solves

2. **Structure Constraints**
   - Vertical/horizontal alignment
   - Parallel/perpendicular relationships
   - Verify: Structure is properly aligned

3. **Feature Constraints**
   - Position features relative to structure
   - Use appropriate constraint types
   - Verify: Features are correctly positioned

4. **Refinement Constraints**
   - Symmetry constraints
   - Equal length/radius
   - Verify: Design is polished

## Common Patterns

### Positioning a Point on a Face

**Problem**: Constrain a point to lie on a plane (e.g., front face).

**Solution**: Use `point_on_line` with an edge of that face, then constrain distances.

```python
# Point on front face (Y=0)
constraints.append({
    "type": "point_on_line",
    "point": "feature_point",
    "line": "front_left_edge"  # Edge on the front face
})
# Then constrain X and Z with distances
```

### Centering Features

**Problem**: Center a feature horizontally/vertically.

**Solution**: Use distances from opposite corners or edges.

```python
# Center horizontally: distance from left = distance from right
constraints.append({
    "type": "distance",
    "between": ["left_corner", "center_point"],
    "value": "$width / 2"
})
constraints.append({
    "type": "distance",
    "between": ["right_corner", "center_point"],
    "value": "$width / 2"
})
```

### Symmetric Roofs/Structures

**Problem**: Create symmetric roof or structure.

**Solution**: Use `equal_length` for all ridge lines, position peak relative to center.

```python
# All roof ridges equal length
constraints.append({
    "type": "equal_length",
    "entities": ["ridge_1", "ridge_2", "ridge_3", "ridge_4"]
})
# Peak positioned relative to center
constraints.append({
    "type": "distance",
    "between": ["center_point", "peak"],
    "value": "$roof_height"
})
```

## Quality Checklist

Before considering a design complete:

- [ ] Solves successfully (no errors)
- [ ] All views render correctly (XY, XZ, YZ, Isometric)
- [ ] Proportions match reference materials
- [ ] All features are properly constrained
- [ ] Parameters work correctly (test different values)
- [ ] Code is well-organized and documented
- [ ] Renders look professional
- [ ] No redundant constraints
- [ ] Design is parametric (uses parameters, not hard-coded values)

## Example: Birdhouse Project

See `examples/generate_birdhouse.py` for a complete example applying this methodology:

1. **Research**: Studied classic birdhouse designs, dimensions, features
2. **Foundation**: Built base box with proper constraints
3. **Structure**: Added roof with symmetric constraints
4. **Features**: Added entrance hole and perch
5. **Refinement**: Adjusted proportions, verified renders

## Next Steps

After mastering this methodology:

1. Apply to new projects (furniture, mechanisms, architectural elements)
2. Build a library of reusable patterns
3. Create templates for common design types
4. Develop automated quality checks
5. Document domain-specific best practices

## References

- `examples/generate_birdhouse.py` - Complete example
- `docs/ITERATIVE_DESIGN.md` - Iterative design principles
- `examples/` - Other examples showing various techniques

