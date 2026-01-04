# SLVSX - SolveSpace Constraint Solver CLI

[![CI Status](https://github.com/snoble/slvsx-cli/actions/workflows/build.yml/badge.svg)](https://github.com/snoble/slvsx-cli/actions)
[![codecov](https://codecov.io/gh/snoble/slvsx-cli/graph/badge.svg)](https://codecov.io/gh/snoble/slvsx-cli)

A command-line tool that makes the SolveSpace geometric constraint solver accessible to AI agents and developers through a simple JSON interface.

## Features

- 🤖 **AI-Ready** - Designed for use by AI agents through subprocess calls
- 📦 **Static Binary** - Single executable with no dependencies
- 🔧 **JSON Interface** - Simple input/output format
- 🎯 **Constraint Solving** - Points, lines, circles, distances, angles, and more
- 📐 **Export Formats** - SVG, DXF, STL output
- 🚀 **Fast** - Native C++ solver wrapped in Rust

## Installation

### Download Static Binary (Recommended)

```bash
# Linux (x86_64)
curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-linux.tar.gz | tar xz
sudo mv slvsx /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-macos-arm64.tar.gz | tar xz
sudo mv slvsx /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-macos-x86_64.tar.gz | tar xz
sudo mv slvsx /usr/local/bin/

# Test installation
slvsx --version
```

### Build from Source

See [docs/BUILDING.md](docs/BUILDING.md) for detailed build instructions.

## Quick Start

### Try It Now

```bash
# Solve a triangle from distances
slvsx solve examples/02_triangle.json

# Create a parametric hinge mechanism
slvsx solve examples/08_angles.json

# Design a symmetric arrowhead
slvsx solve examples/11_symmetric.json

# Export to SVG for visualization
slvsx export -f svg examples/08_angles.json -o output.svg
```

### Basic Example: Triangle from Distances

```bash
# Create a simple constraint problem
cat > triangle.json << 'EOF'
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {"type": "point", "id": "A", "at": [0, 0, 0]},
    {"type": "point", "id": "B", "at": [100, 0, 0]},
    {"type": "point", "id": "C", "at": [50, 50, 0]}
  ],
  "constraints": [
    {"type": "fixed", "entity": "A"},
    {"type": "fixed", "entity": "B"},
    {"type": "distance", "between": ["A", "C"], "value": 80},
    {"type": "distance", "between": ["B", "C"], "value": 60}
  ]
}
EOF

# Solve it
slvsx solve triangle.json

# Export to SVG
slvsx export -f svg triangle.json > triangle.svg
```

**What this does**: Given two fixed points and distances to a third point, SLVSX calculates where the third point must be. This is triangulation - the same math used in GPS!

See [SHOWCASE.md](SHOWCASE.md) for impressive examples and [docs/AI_GUIDE.md](docs/AI_GUIDE.md) for AI agent usage.

### Commands

```bash
slvsx solve input.json          # Solve constraints
slvsx validate input.json       # Check validity
slvsx export -f svg input.json  # Export to SVG
```

### Use from Python

```python
import json, subprocess

def solve(problem):
    result = subprocess.run(
        ['slvsx', 'solve', '-'],
        input=json.dumps(problem),
        capture_output=True,
        text=True
    )
    return json.loads(result.stdout) if result.returncode == 0 else None
```

## For AI Agents

SLVSX is designed to be used by AI agents for solving geometric constraint problems. Perfect for:

- **Constraint-based design generation** - Describe what you want, not how to draw it
- **Mechanism validation** - Check if designs are physically possible
- **Parametric optimization** - Explore design spaces systematically
- **Mathematical precision** - Get exact solutions, not approximations

**Quick Links**:
- [AI Agent Guide](docs/AI_GUIDE.md) - Complete guide for AI usage
- [Showcase](SHOWCASE.md) - Impressive examples and use cases
- [MCP Integration Guide](docs/MCP_INTEGRATION.md) - Use with Claude Desktop
- [AI Examples](examples/ai-examples/) - Ready-to-use constraint problems

## Examples

The [`examples/`](examples/) directory contains many constraint problems:

### 🎯 Quick Wins
- **[Triangle Solver](examples/02_triangle.json)** - Triangulation from distances
- **[Angle Hinge](examples/08_angles.json)** - Parametric hinge mechanism
- **[Symmetric Arrow](examples/11_symmetric.json)** - Symmetry constraints

### 🔧 Real-World Applications
- **[Four-Bar Linkage](examples/ai-examples/four_bar_linkage.json)** - Classic kinematic mechanism
- **[Planetary Gears](examples/ai-examples/gear_meshing.json)** - Gear train positioning
- **[3D Tetrahedron](examples/04_3d_tetrahedron.json)** - Three-dimensional geometry

### 📚 Learning Path
- **[Tutorial Series](examples/README.md)** - Step-by-step learning guide
- **[Constraint Reference](examples/constraints/)** - Detailed constraint examples

See [SHOWCASE.md](SHOWCASE.md) for more impressive examples and use cases!

## Documentation

### Getting Started
- [Showcase](SHOWCASE.md) - What can you build? Impressive examples
- [AI Agent Guide](docs/AI_GUIDE.md) - Complete guide for AI usage
- [Examples Tutorial](examples/README.md) - Step-by-step learning

### Reference
- [Building from Source](docs/BUILDING.md)
- [MCP Integration](docs/MCP_INTEGRATION.md)
- [Development Guide](docs/DEVELOPMENT.md)
- [JSON Schema](schema/slvs-json.schema.json)

### For AI Agents
- [AI Guide](docs/AI_GUIDE.md) - Patterns, examples, best practices
- [MCP Integration](docs/MCP_INTEGRATION.md) - Use with Claude Desktop
- [AI Examples](examples/ai-examples/) - Ready-to-use problems

## License

GPLv3 - See [LICENSE](LICENSE) file for details.

Built on top of [SolveSpace](https://solvespace.com/)'s constraint solver library.