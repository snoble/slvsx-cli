# Slider-Crank Mechanism

A classic SolveSpace tutorial example demonstrating a slider-crank mechanism (like in an engine).

## Story

The slider-crank mechanism converts rotational motion into linear motion. This is the fundamental mechanism in:
- Internal combustion engines
- Pumps and compressors
- Various industrial machinery

## Mechanism Components

- **Crank**: Rotates around fixed pivot
- **Connecting Rod**: Links crank to piston
- **Piston**: Moves along horizontal line (slider)

## Constraints

1. Crank pivot is fixed
2. Piston moves along horizontal line
3. Crank length is parametric
4. Connecting rod length is parametric
5. Crank angle controls position

## Usage

```bash
# Solve with default parameters
slvsx solve examples/20_slider_crank.json

# Rotate crank to different angles
slvsx solve examples/20_slider_crank.json \
  --param crank_angle=90

# Export visualization
slvsx export --format svg examples/20_slider_crank.json -o slider_crank.svg
```

## Missing Features

This example requires:
- **Horizontal** constraint (not implemented)
- **Angle** constraint (not implemented)

Once these are implemented, this will demonstrate classic kinematic mechanism design.

## Related Tutorials

- [SolveSpace Linkages Tutorial](https://solvespace.com/tutorial.pl)
- Kinematics of mechanisms

