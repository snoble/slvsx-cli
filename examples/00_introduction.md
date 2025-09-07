# SLVSX Examples: Introduction

Welcome to the SLVSX constraint solver examples! These tutorials will guide you through geometric constraint solving, from simple 2D sketches to complex 3D assemblies.

## What is Constraint Solving?

Imagine you're designing a mechanical linkage, laying out a floor plan, or creating a parametric design. You know certain relationships must hold:
- This distance should be exactly 100mm
- These two lines should be perpendicular
- This point should be at the midpoint of that line

A constraint solver takes these relationships and figures out the actual positions that satisfy all constraints simultaneously.

## How These Examples Work

Each example includes:
1. **The Story** - What we're building and why
2. **The Constraints** - The geometric relationships we're defining
3. **The JSON** - The actual constraint specification
4. **The Solution** - What the solver calculated
5. **Visual Output** - An SVG showing the result

## Example Structure

Every SLVSX document has this structure:

```json
{
  "schema": "slvs-json/1",     // Version identifier
  "units": "mm",                // Units (mm, inch, etc.)
  "parameters": {},             // Named values you can reference
  "entities": [],               // Points, lines, circles, etc.
  "constraints": []             // Relationships between entities
}
```

## Navigation

Start with the first example:

**[Next: Example 01 - Your First Point →](01_first_point.md)**

---

## Quick Reference

- **Entities**: point, line, circle, arc, workplane
- **Constraints**: fixed, distance, angle, parallel, perpendicular, coincident, and many more
- **Parameters**: Use `$name` to reference a parameter value

Let's begin!