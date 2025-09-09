# SLVSX - SolveSpace Constraint Solver CLI

[![CI Status](https://github.com/snoble/slvsx-cli/actions/workflows/build.yml/badge.svg)](https://github.com/snoble/slvsx-cli/actions)

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
# Linux/macOS - Download and install in one command
curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-$(uname -s)-$(uname -m) -o slvsx
chmod +x slvsx
sudo mv slvsx /usr/local/bin/

# Test installation
slvsx --version
```

### Build from Source

See [docs/BUILDING.md](docs/BUILDING.md) for detailed build instructions.

## Quick Start

### Basic Example

```bash
# Create a simple constraint problem
cat > triangle.json << 'EOF'
{
  "schema_version": "0.3.0",
  "units": "mm",
  "entities": {
    "A": {"type": "point", "point": {"x": 0, "y": 0}},
    "B": {"type": "point", "point": {"x": 100, "y": 0}},
    "C": {"type": "point", "point": {"x": 50, "y": 50}}
  },
  "constraints": [
    {"type": "fixed", "entity": "A"},
    {"type": "fixed", "entity": "B"},
    {"type": "distance", "entities": ["A", "C"], "distance": 80},
    {"type": "distance", "entities": ["B", "C"], "distance": 60}
  ]
}
EOF

# Solve it
slvsx solve triangle.json
```

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

SLVSX is designed to be used by AI agents for solving geometric constraint problems. See:

- [MCP Integration Guide](docs/MCP_INTEGRATION.md) - How to use with Claude and other AI
- [AI Examples](examples/ai-examples/) - Ready-to-use constraint problems
- [MCP Server](MCP_SERVER.md) - Future native MCP support

## Examples

The [`examples/`](examples/) directory contains many constraint problems:

- [AI Examples](examples/ai-examples/) - Designed for AI agent use
- [Basic Shapes](examples/01_first_point.json) - Simple geometric constructions
- [Mechanisms](examples/four_bar_linkage.json) - Kinematic linkages
- [Gears](examples/planetary_gears_simple.json) - Gear train positioning

## Documentation

- [Building from Source](docs/BUILDING.md)
- [MCP Integration](docs/MCP_INTEGRATION.md)
- [Development Guide](docs/DEVELOPMENT.md)
- [JSON Schema](schema/slvs-json.schema.json)

## License

GPLv3 - See [LICENSE](LICENSE) file for details.

Built on top of [SolveSpace](https://solvespace.com/)'s constraint solver library.