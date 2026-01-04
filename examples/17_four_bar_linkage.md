# Four-Bar Linkage Mechanism

A classic SolveSpace tutorial example demonstrating kinematic linkage design.

## Story

Four-bar linkages are fundamental mechanisms in mechanical engineering. They convert rotational motion into complex paths. This example models a crank-rocker mechanism where:
- The **crank** rotates fully (360°)
- The **rocker** oscillates back and forth
- The **coupler** traces an interesting path

## Mechanism Components

- **Ground**: Fixed base with two pivot points
- **Crank**: Input link that rotates
- **Coupler**: Connects crank to rocker
- **Rocker**: Output link that oscillates

## Constraints

1. Ground pivots are fixed
2. Link lengths are constrained by parameters
3. Input angle controls crank position
4. All links must form a closed loop

## Usage

```bash
# Solve with default parameters
slvsx solve examples/17_four_bar_linkage.json

# Vary the input angle to see different positions
slvsx solve examples/17_four_bar_linkage.json \
  --param input_angle=90

# Export visualization
slvsx export --format svg examples/17_four_bar_linkage.json -o linkage.svg
```

## Missing Features

This example requires the **Angle** constraint, which is currently not implemented. Once implemented, this will demonstrate full four-bar linkage kinematics.

## Related Tutorials

- [SolveSpace Linkages Tutorial](https://solvespace.com/tutorial.pl)
- Classic mechanical engineering textbook: "Mechanisms and Mechanical Devices Sourcebook"

