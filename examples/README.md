# SLVSX Examples

A comprehensive tutorial series for learning geometric constraint solving with SLVSX.

## Getting Started

These examples are designed to be read in order, building from simple concepts to complex mechanisms. Each example includes:

- **Story & Motivation**: Why this constraint matters
- **Complete JSON**: Copy-paste ready examples
- **Actual Output**: Real results from the SLVSX solver
- **Visualizations**: SVG outputs showing the solution 
- Generated SVG visualization
- Verified solver output

## 🔧 Basic Constraint Examples

### [01. Basic Distance Constraint](01_basic_distance.md)
**File:** `01_basic_distance.json`

Demonstrates fundamental distance constraints between two points. Shows how the solver positions points to satisfy exact distance requirements.

![Basic Distance](01_basic_distance.svg)

### [02. Equilateral Triangle](02_triangle.md) 
**File:** `02_triangle.json`

Creates a triangle with all sides equal length using multiple distance constraints. Demonstrates simultaneous constraint solving.

![Triangle](02_triangle.svg)

### [03. Understanding Constraints](03_constraints.md)
**Files:** `03_overconstrained.json`, `03_correctly_constrained.json`

Shows the difference between properly constrained and over-constrained systems. Critical for understanding constraint design.

## 🏗️ 3D Examples

### [04. 3D Regular Tetrahedron](04_3d_tetrahedron.md)
**File:** `04_3d_tetrahedron.json`

Demonstrates 3D constraint solving by creating a regular tetrahedron with all edges equal length.

![Tetrahedron](04_3d_tetrahedron.svg)

## ⭕ Circle Examples

### 06. Circles with Distance Constraints
**File:** `06_circles.json`

Shows how to work with circles and constrain their center distances.

![Circles](06_circles.svg)


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