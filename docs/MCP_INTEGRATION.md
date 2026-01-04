# MCP Integration for AI Agents

## Overview

SLVSX provides the SolveSpace constraint solver (libslvs) as a tool for AI agents through the Model Context Protocol (MCP). This enables AI to solve geometric constraint problems programmatically.

## Installation for AI Use

### Option 1: Static Binary
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
```

### Option 2: NPM Package (Coming Soon)
```bash
npm install -g slvsx-cli
```

## Current Status

✅ **Working**: Direct CLI invocation via subprocess  
🚧 **In Progress**: Native MCP server implementation  
📦 **Planned**: NPM package for easy installation

AI agents can currently use SLVSX through subprocess calls to the CLI binary.

## Quick Start for AI Agents

### 1. Install SLVSX
```bash
# Download the static binary (no dependencies needed)
curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-$(uname -s)-$(uname -m) -o /usr/local/bin/slvsx
chmod +x /usr/local/bin/slvsx
```

### 2. Use from Python
```python
import json
import subprocess

def solve_constraints(problem_json):
    """Solve geometric constraints using SLVSX CLI"""
    result = subprocess.run(
        ['slvsx', 'solve', '-'],
        input=json.dumps(problem_json),
        capture_output=True,
        text=True
    )
    if result.returncode == 0:
        return json.loads(result.stdout)
    else:
        raise Exception(f"Solver error: {result.stderr}")
```

### 3. Use from Node.js
```javascript
const { execSync } = require('child_process');

function solveConstraints(problem) {
    const input = JSON.stringify(problem);
    const result = execSync('slvsx solve -', {
        input: input,
        encoding: 'utf8'
    });
    return JSON.parse(result);
}
```

### JSON Input Format
```json
{
  "schema_version": "0.3.0",
  "units": "mm",
  "entities": {
    "p1": {
      "type": "point",
      "point": {"x": 0, "y": 0, "z": 0}
    },
    "p2": {
      "type": "point",
      "point": {"x": 100, "y": 0, "z": 0}
    }
  },
  "constraints": [
    {
      "type": "distance",
      "entities": ["p1", "p2"],
      "distance": 75.0
    }
  ]
}
```

## Common Use Cases for AI

### 1. Find Point Position with Distance Constraints
```python
# Find where to place point C given distances from A and B
problem = {
    "schema_version": "0.3.0",
    "units": "mm",
    "entities": {
        "A": {"type": "point", "point": {"x": 0, "y": 0}},
        "B": {"type": "point", "point": {"x": 100, "y": 0}},
        "C": {"type": "point", "point": {"x": 50, "y": 50}}  # Initial guess
    },
    "constraints": [
        {"type": "fixed", "entity": "A"},
        {"type": "fixed", "entity": "B"},
        {"type": "distance", "entities": ["A", "C"], "distance": 80},
        {"type": "distance", "entities": ["B", "C"], "distance": 60}
    ]
}
```

### 2. Mechanism Validation
```python
# Check if a four-bar linkage can close
linkage = {
    "schema_version": "0.3.0",
    "units": "mm",
    "entities": {
        # Define joints as points
        "A": {"type": "point", "point": {"x": 0, "y": 0}},
        "B": {"type": "point", "point": {"x": 100, "y": 0}},
        "C": {"type": "point", "point": {"x": 120, "y": 80}},
        "D": {"type": "point", "point": {"x": 20, "y": 60}}
    },
    "constraints": [
        {"type": "fixed", "entity": "A"},
        {"type": "fixed", "entity": "B"},
        # Link lengths
        {"type": "distance", "entities": ["A", "D"], "distance": 70},
        {"type": "distance", "entities": ["B", "C"], "distance": 90},
        {"type": "distance", "entities": ["C", "D"], "distance": 100}
    ]
}
```

### 3. Gear Positioning
```python
# Position gears with proper meshing
gears = {
    "schema_version": "0.3.0",
    "units": "mm",
    "entities": {
        "sun": {"type": "circle", "center": {"x": 0, "y": 0}, "radius": 24},
        "planet1": {"type": "circle", "center": {"x": 36, "y": 0}, "radius": 12},
        "planet2": {"type": "circle", "center": {"x": -18, "y": 31.18}, "radius": 12}
    },
    "constraints": [
        {"type": "fixed", "entity": "sun"},
        # Meshing constraints (distance = sum of radii)
        {"type": "distance", "entities": ["sun", "planet1"], "distance": 36},
        {"type": "distance", "entities": ["sun", "planet2"], "distance": 36}
    ]
}
```

## Available Commands

### Validate
Check if constraints are valid without solving:
```bash
slvsx validate input.json
```

### Solve
Solve constraints and output positioned entities:
```bash
slvsx solve input.json
```

### Export
Solve and export to various formats:
```bash
slvsx export -f svg input.json -o output.svg
slvsx export -f dxf input.json -o output.dxf
```

## Error Handling

SLVSX returns specific exit codes:
- `0` - Success
- `1` - General error
- `2` - Validation error
- `3` - Solver error (overconstrained/underconstrained)
- `4` - I/O error

Check stderr for detailed error messages.

## Future MCP Implementation

When implemented, the MCP server will provide:

### Tools
- `solve_constraints` - Solve a constraint system
- `validate_constraints` - Check validity without solving
- `export_solution` - Export to various formats
- `get_capabilities` - List supported constraint types

### Configuration
```json
{
  "servers": {
    "slvsx": {
      "command": "slvsx",
      "args": ["mcp-server"],
      "env": {},
      "schema": {
        "transport": "stdio"
      }
    }
  }
}
```

## Best Practices for AI

1. **Start with simple constraints** - Add complexity gradually
2. **Provide initial guesses** - Helps solver converge
3. **Check degrees of freedom** - Ensure problem is properly constrained
4. **Handle errors gracefully** - Many user problems are overconstrained
5. **Use appropriate units** - Be consistent throughout the problem

## Examples

See the `examples/` directory for more constraint problems:
- `01_first_point.json` - Basic point positioning
- `02_distance_constraint.json` - Distance constraints
- `03_constraints.json` - Multiple constraint types
- `planetary_gears_*.json` - Complex gear systems

## Contributing

To help implement MCP support:
1. See the prototype in `mcp-server.js`
2. Implement in Rust at `crates/cli/src/mcp.rs`
3. Follow the [MCP specification](https://modelcontextprotocol.io)
4. Test with Claude Desktop or other MCP clients