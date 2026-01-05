# Example 22: Slider-Crank Mechanism

**[← Complex Mechanisms](16_complex_mechanisms.md)** | **[Back to Introduction →](00_introduction.md)**

## The Story

The slider-crank mechanism is one of the most important mechanisms in mechanical engineering. It converts rotary motion (from the crank) into linear motion (of the piston). This is the fundamental mechanism used in internal combustion engines, compressors, and many other machines.

## How It Works

The slider-crank consists of:
- **Crank**: A rotating link pivoted at one end
- **Connecting Rod**: Links the crank to the piston
- **Piston**: Slides along a fixed path (cylinder)
- **Fixed Pivot**: The crank's rotation center

As the crank rotates, the connecting rod pushes/pulls the piston along its linear path.

## The Constraints

```json
{
  "parameters": {
    "crank_length": 30.0,
    "connecting_rod_length": 80.0,
    "crank_angle": 45.0
  }
}
```

**Key Constraints**:
1. **Fixed crank pivot** - The rotation center is anchored
2. **Crank length** - Distance from pivot to crank end
3. **Connecting rod length** - Distance from crank end to piston
4. **Crank angle** - The rotation angle of the crank
5. **Piston on path** - Piston must slide along horizontal line

## Degrees of Freedom

This mechanism has **1 DOF** - the crank angle. Once you specify the crank angle, all other positions are determined by the constraint solver.

## Parametric Design

Change the crank angle to see the piston position:
- `crank_angle = 0°` → Piston at maximum extension
- `crank_angle = 90°` → Piston at mid-stroke
- `crank_angle = 180°` → Piston at minimum extension

## Real-World Applications

- **Internal Combustion Engines**: Converts piston motion to crankshaft rotation
- **Compressors**: Pumps air/gas by piston motion
- **Steam Engines**: Historical power generation
- **Pumps**: Positive displacement pumps
- **Robotics**: Linear actuators

## Visual Output

![Slider-Crank Mechanism](22_slider_crank.svg)

## Key Takeaways

1. **Mechanisms have DOF** - They can move within constraints
2. **Point-on-line** - Essential for sliding motion
3. **Angle constraints** - Control input motion
4. **Parametric design** - Easy to explore different configurations

## Try It

```bash
# Solve with default parameters
slvsx solve examples/22_slider_crank.json

# Try different crank angles
slvsx solve examples/22_slider_crank.json --param crank_angle=90

# Export visualization
slvsx export -f svg examples/22_slider_crank.json -o slider_crank.svg
```

---

**[Back to Introduction →](00_introduction.md)**

