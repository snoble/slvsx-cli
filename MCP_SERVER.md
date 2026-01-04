# Using SLVSX as an MCP Server

## Overview

SLVSX can be used as an MCP (Model Context Protocol) server, allowing AI agents to solve constraint problems programmatically. This enables agents to leverage the SolveSpace constraint solver (libslvs) for geometric and engineering calculations.

## Installation for MCP Usage

### Option 1: Direct Binary (Recommended)
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

### Option 2: Build from Source
```bash
git clone https://github.com/snoble/slvsx-cli
cd slvsx-cli
nix-shell build.nix --run "cargo build --release"
sudo cp target/release/slvsx /usr/local/bin/
```

### Option 3: NPM Package (Future)
```bash
# TODO: Once published to npm
npm install -g slvsx-cli
```

## MCP Server Configuration

Add SLVSX to your MCP configuration file (typically `~/.config/mcp/servers.json` or similar):

```json
{
  "servers": {
    "slvsx": {
      "command": "/usr/local/bin/slvsx",
      "args": ["mcp-server"],
      "description": "SolveSpace constraint solver for geometric problems"
    }
  }
}
```

## Using SLVSX from an AI Agent

### Basic Example

```python
# Example for a Python-based agent
import json
import subprocess

def solve_constraints(constraint_json):
    """Solve geometric constraints using SLVSX"""
    result = subprocess.run(
        ['slvsx', 'solve', '-'],
        input=json.dumps(constraint_json),
        capture_output=True,
        text=True
    )
    return json.loads(result.stdout)

# Example: Find point position with distance constraint
problem = {
    "schema": "slvs-json/1",
    "units": "mm",
    "parameters": {},
    "entities": [
        {"type": "point", "id": "A", "at": [0, 0, 0]},
        {"type": "point", "id": "B", "at": [100, 0, 0]}
    ],
    "constraints": [
        {"type": "fixed", "entity": "A"},
        {"type": "distance", "between": ["A", "B"], "value": 75.0}
    ]
}

solution = solve_constraints(problem)
print(f"Point B moved to: {solution['entities']['B']['at']}")
```

### MCP Protocol Usage

When configured as an MCP server, SLVSX exposes the following tools:

#### `solve_constraints`
Solves a constraint system and returns positioned entities.

**Input:**
```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {},
  "entities": [...],
  "constraints": [...]
}
```

**Output:**
```json
{
  "status": "ok",
  "diagnostics": {
    "iters": 1,
    "residual": 0.0,
    "dof": 0,
    "time_ms": 1
  },
  "entities": {
    "entity_id": {"at": [x, y, z]},
    ...
  }
}
```

#### `validate_constraints`
Checks if a constraint system is valid without solving.

#### `export_solution`
Exports solved geometry to various formats (SVG, DXF, etc.).

## Common Use Cases for AI Agents

### 1. Engineering Design Validation
```javascript
// Check if a mechanism design is valid
const mechanism = {
  schema: "slvs-json/1",
  units: "mm",
  entities: [
    // Define linkage points
  ],
  constraints: [
    // Define joint constraints
  ]
};

const result = await mcp.call('slvsx', 'solve_constraints', mechanism);
if (result.status !== 'ok') {
  console.log('Design is overconstrained or invalid');
}
```

### 2. Geometric Problem Solving
```python
# Find the third vertex of a triangle given two vertices and all side lengths
triangle_problem = {
    "schema": "slvs-json/1",
    "units": "mm",
    "entities": [
        {"type": "point", "id": "A", "at": [0, 0, 0]},
        {"type": "point", "id": "B", "at": [100, 0, 0]},
        {"type": "point", "id": "C", "at": [50, 50, 0]}  # Initial guess
    ],
    "constraints": [
        {"type": "fixed", "entity": "A"},
        {"type": "fixed", "entity": "B"},
        {"type": "distance", "between": ["A", "C"], "value": 80},
        {"type": "distance", "between": ["B", "C"], "value": 60}
    ]
}
```

### 3. CAD-like Operations
```typescript
// Constrain lines to be parallel or perpendicular
const parallel_lines = {
  schema: "slvs-json/1",
  entities: [
    {type: "line", id: "L1", p1: "A", p2: "B"},
    {type: "line", id: "L2", p1: "C", p2: "D"}
  ],
  constraints: [
    {type: "parallel", entities: ["L1", "L2"]}
  ]
};
```

## Error Handling

SLVSX returns specific error codes that agents should handle:

- `status: "ok"` - Constraints solved successfully
- `status: "overconstrained"` - Too many constraints (no solution exists)
- `status: "underconstrained"` - Too few constraints (infinite solutions)
- `status: "error"` - Invalid input or solver failure

## Best Practices for AI Agents

1. **Always validate input** - Use the JSON schema to ensure correct format
2. **Check degrees of freedom** - The `dof` field indicates remaining unconstrained movements
3. **Handle solver failures gracefully** - Overconstrained systems are common in user-provided problems
4. **Use parameters** - Parameterize dimensions for easy modification
5. **Start simple** - Build constraint systems incrementally

## Implementation Status

- [x] CLI tool with solve command
- [x] JSON input/output format
- [x] Static binary distribution
- [ ] MCP server mode implementation
- [ ] NPM package publication
- [ ] Tool registration in MCP registry

## Contributing

To add MCP server support:

1. Implement the MCP protocol handler in `crates/cli/src/mcp.rs`
2. Add server mode to the CLI arguments
3. Implement bidirectional JSON-RPC communication
4. Add tool definitions for constraint operations
5. Test with Claude Desktop or other MCP clients

## Resources

- [MCP Protocol Specification](https://github.com/anthropics/mcp)
- [SLVSX JSON Format](./examples/README.md)
- [SolveSpace Documentation](http://solvespace.com/ref.pl)