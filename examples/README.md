# SLVSX Examples

This directory contains examples demonstrating SLVSX's capabilities for solving various mechanical constraint problems.

## 🔗 Linkage Analysis

### Four-Bar Linkage (`four_bar_linkage.json`)

A classic mechanical linkage where an input crank drives an output rocker through a coupler link.

```bash
# Solve the linkage configuration
slvsx solve examples/testdata/four_bar_linkage.json

# Export as SVG for visualization  
slvsx export --format svg examples/testdata/four_bar_linkage.json -o linkage.svg

# Vary input angle to trace the mechanism
slvsx solve examples/testdata/four_bar_linkage.json --param input_angle=0
slvsx solve examples/testdata/four_bar_linkage.json --param input_angle=90
slvsx solve examples/testdata/four_bar_linkage.json --param input_angle=180
```

**Use Case**: Design mechanical systems with specific motion requirements, analyze workspace, optimize link lengths.

## 🔧 Simple Constraints

### Two Touching Circles (`simple_two_circles.json`)

Demonstrates basic distance constraints between circular features.

```bash
slvsx solve examples/testdata/simple_two_circles.json
```

**Use Case**: Layout of cylindrical components, bearing positioning, pipe routing.

### Triangle (`triangle.json`)

Three points constrained to form a triangle with specific side lengths.

```bash
slvsx solve examples/testdata/triangle.json
```

**Use Case**: Truss analysis, structural layout, triangulation problems.

## ⚙️ Gear Systems

### Simple Two Gears (`simple_two_gears.json`)

Two meshing gears with proper center distance and phase alignment.

```bash
slvsx solve examples/testdata/simple_two_gears.json
slvsx export --format svg examples/testdata/simple_two_gears.json -o gears.svg
```

**Use Case**: Basic gear train design, speed reduction analysis.

### Three Planet System (`three_planet_system.json`)

Planetary gear system with sun, three planets, and ring gear.

```bash
slvsx solve examples/testdata/three_planet_system.json
```

**Use Case**: Automatic transmission design, high-ratio reduction, compact mechanisms.

## 🏗️ Complex Assemblies

### Planetary Complete (`planetary_complete.json`)

Full planetary gear system with all constraints properly defined.

```bash
slvsx solve examples/testdata/planetary_complete.json
slvsx export --format stl examples/testdata/planetary_complete.json -o planetary.stl
```

**Use Case**: Complete mechanical system design, 3D printing preparation, assembly validation.

## 📐 Parameter Studies

Most examples include parameters that can be varied:

```bash
# Explore design space by varying parameters
slvsx solve examples/testdata/four_bar_linkage.json \
  --param link2_length=60 \
  --param link3_length=90 \
  --param input_angle=30

# Batch analysis with different configurations
for angle in 0 30 60 90 120 150 180; do
  slvsx solve examples/testdata/four_bar_linkage.json \
    --param input_angle=$angle \
    --export svg -o "linkage_${angle}deg.svg"
done
```

## 🎯 AI Agent Examples

These examples are perfect for AI agents because they:

1. **Accept programmatic input** - JSON specification instead of GUI interaction
2. **Support parameter variation** - Easy to explore design spaces
3. **Provide mathematical validation** - Constraint satisfaction guarantees feasibility
4. **Export multiple formats** - SVG for visualization, STL for manufacturing

### Agent Workflow Example

```python
import subprocess
import json

# Agent generates constraint specification
linkage_spec = {
    "schema": "slvs-json/1",
    "units": "mm", 
    "parameters": {"input_angle": 45, "link_length": 80},
    # ... constraint definition
}

# Solve constraints
result = subprocess.run([
    "slvsx", "solve", "-"], 
    input=json.dumps(linkage_spec),
    capture_output=True, text=True
)

# Parse solution
solution = json.loads(result.stdout)
if solution["status"] == "ok":
    print("✓ Valid mechanism found!")
    # Export for visualization/manufacturing
    subprocess.run([
        "slvsx", "export", "--format", "svg", 
        "-", "-o", "result.svg"
    ], input=json.dumps(linkage_spec), text=True)
```

## 🔍 Example Categories

- **📏 Geometric**: Basic shapes, distances, angles
- **🔗 Kinematic**: Linkages, motion constraints  
- **⚙️ Mechanical**: Gears, assemblies, interference checking
- **📐 Parametric**: Design optimization, parameter studies
- **🏭 Manufacturing**: 3D printing, assembly validation

Each example demonstrates SLVSX's ability to turn mechanical design from manual sketching into mathematical constraint solving - perfect for AI agents doing systematic mechanical problem solving.