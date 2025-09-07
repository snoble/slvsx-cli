# SLVSX - SolveSpace Constraint Solver CLI

A command-line interface and library for the SolveSpace geometric constraint solver, providing JSON-based constraint specification with multi-language support through WASM.

## Features

- **JSON-based constraint specification** - Define geometric constraints using a simple, type-safe JSON schema
- **Multi-language support** - Use from Rust, JavaScript/TypeScript (via WASM), or any language that can generate JSON
- **MCP Server** - Use with Claude Desktop and other AI assistants via Model Context Protocol
- **Type-safe workflow** - Rust types → JSON Schema → TypeScript/Python types, ensuring compatibility across languages
- **Cross-platform** - Native binaries for Linux, macOS, Windows, plus WASM for browsers/Node.js
- **Comprehensive constraint support** - Points, lines, circles, distance/angle/perpendicular constraints, and more
- **Export formats** - SVG, DXF, STL output for solved geometries
- **Parameter support** - Define parametric designs with variables

## Installation

### Native Binary

Download the latest release for your platform from the [releases page](https://github.com/snoble/slvsx-cli/releases):

```bash
# Linux/macOS
curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-linux-x86_64.tar.gz | tar xz
./slvsx --help

# Windows
# Download slvsx-windows-x86_64.exe.zip and extract
```

### From Source

```bash
git clone https://github.com/snoble/slvsx-cli.git
cd slvsx-cli
nix-shell build.nix  # Sets up complete dev environment
cargo build --release
./target/release/slvsx --help
```

### WASM Module (JavaScript/TypeScript)

```bash
npm install @slvsx/core
```

```javascript
import init, { WasmSolver } from '@slvsx/core';

await init();
const solver = new WasmSolver();
const result = solver.solve(constraintJson);
```

### MCP Server (for Claude Desktop)

```bash
npm install -g @slvsx/mcp-server
```

Then add to Claude Desktop config:
```json
{
  "mcpServers": {
    "slvsx": {
      "command": "npx",
      "args": ["@slvsx/mcp-server"]
    }
  }
}
```

See [MCP_README.md](MCP_README.md) for detailed setup.

## Usage

### Command Line

```bash
# Solve constraints and output results
slvsx solve input.json

# Validate constraint document
slvsx validate input.json

# Export to SVG
slvsx export --format svg --output design.svg input.json

# Generate JSON schema for type generation
slvsx schema > schema.json
```

### JSON Constraint Format

The solver accepts constraints in JSON format following this structure:

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "width": 100,
    "height": 50
  },
  "entities": [
    {
      "id": "p1",
      "type": "Point",
      "x": 0,
      "y": 0
    },
    {
      "id": "p2", 
      "type": "Point",
      "x": "$width",
      "y": 0
    },
    {
      "id": "line1",
      "type": "Line",
      "points": ["p1", "p2"]
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "p1"
    },
    {
      "type": "HorizontalDistance",
      "entities": ["p1", "p2"],
      "distance": "$width"
    }
  ]
}
```

### Type-Safe Development

#### Generate Types from Schema

```bash
# Generate JSON schema
slvsx schema > schema.json

# TypeScript
npx json-schema-to-typescript schema.json -o types.ts

# Python
datamodel-codegen --input schema.json --output types.py
```

#### TypeScript Example

```typescript
import { InputDocument, Entity, Constraint } from './types';

function createTriangle(sideLength: number): InputDocument {
  return {
    schema: 'slvs-json/1',
    units: 'mm',
    entities: [
      { id: 'p1', type: 'Point', x: 0, y: 0 },
      { id: 'p2', type: 'Point', x: sideLength, y: 0 },
      { id: 'p3', type: 'Point', x: sideLength/2, y: sideLength * 0.866 }
    ],
    constraints: [
      { type: 'Fixed', entity: 'p1' },
      { type: 'Distance', entities: ['p1', 'p2'], distance: sideLength },
      { type: 'Distance', entities: ['p2', 'p3'], distance: sideLength },
      { type: 'Distance', entities: ['p3', 'p1'], distance: sideLength }
    ]
  };
}
```

## Architecture

### Core Components

1. **`slvsx-core`** - Rust library with constraint solver logic
   - FFI bindings to libslvs (SolveSpace's C++ solver)
   - JSON schema generation from Rust types
   - WASM compilation support
   - Mock solver for testing

2. **`slvsx-cli`** - Command-line interface
   - Constraint solving and validation
   - Export to various formats (SVG, DXF, STL)
   - Schema generation for type safety

3. **`slvsx-exporters`** - Format conversion
   - SVG generation with proper scaling
   - DXF export for CAD software
   - STL for 3D printing

### Type Generation Flow

```
Rust Types (source of truth)
    ↓
JSON Schema (schemars)
    ↓
TypeScript/Python/etc Types (json-schema-to-typescript, datamodel-codegen)
```

This ensures all language bindings stay in sync with the Rust implementation.

## Supported Constraints

### Entities
- **Point** - 2D/3D points with x, y, z coordinates
- **Line** - Line segments between two points
- **Circle** - Circles with center and radius
- **Arc** - Circular arcs with start/end angles
- **Cubic** - Cubic Bezier curves

### Constraints
- **Fixed** - Fix entity position
- **Distance** - Set distance between points/lines
- **Angle** - Set angle between lines
- **Perpendicular** - Make lines perpendicular
- **Parallel** - Make lines parallel
- **Horizontal/Vertical** - Align to axes
- **PointOnLine** - Constrain point to line
- **PointOnCircle** - Constrain point to circle
- **Radius** - Set circle/arc radius
- **Equal** - Make distances/radii equal
- **Symmetric** - Mirror symmetry constraint

## Examples

See the [`examples/`](examples/) directory for complete examples including:

- Basic shapes (triangles, squares, polygons)
- Parametric designs with variables
- Mechanical linkages and mechanisms
- 3D constraints and assemblies
- Over-constrained system detection
- TypeScript constraint generation

## Development

### Prerequisites

```bash
# Install Nix package manager
curl -L https://nixos.org/nix/install | sh

# Enter development environment
nix-shell build.nix
```

This provides:
- Rust toolchain with WASM target
- CMake for building libslvs
- wasm-pack for WASM builds
- Testing and coverage tools

### Building

```bash
# Native CLI
cargo build --release

# WASM module
cd crates/core
wasm-pack build --target web --features wasm

# Run tests
cargo test

# Generate coverage
cargo tarpaulin
```

### Testing

```bash
# Unit tests
cargo test

# Integration tests with real solver
cargo test --features real-solver

# WASM tests
wasm-pack test --node
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

GPLv3 - See [LICENSE](LICENSE) file for details

This project incorporates code from SolveSpace, which is licensed under GPLv3. As a result, this entire project must be distributed under GPLv3 terms.

## Acknowledgments

Built on top of [SolveSpace](https://solvespace.com/)'s powerful constraint solver library.