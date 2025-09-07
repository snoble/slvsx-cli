# SLVSX - SolveSpace Constraint Solver CLI

A command-line interface for the SolveSpace geometric constraint solver, designed for programmatic generation and solving of complex mechanical systems like planetary gears.

## Features

- **JSON-based constraint definition**: Define geometric entities and constraints in a readable JSON format
- **Multiple export formats**: SVG, STL, DXF output for manufacturing and visualization
- **Gear system support**: Built-in support for complex gear trains including planetary and double planetary systems
- **Phase calculation**: Automatic calculation of gear phases for proper meshing
- **Validation**: Comprehensive validation of geometric constraints and gear meshing

## Installation

### Using Nix (Recommended)

```bash
nix-shell
cargo build --release
```

### Manual Build

Requirements:
- Rust 1.70+
- CMake 3.10+
- C++ compiler with C++11 support

```bash
# Build libslvs
cd libslvs/SolveSpaceLib
mkdir build && cd build
cmake ..
make

# Build SLVSX
cd ../../..
cargo build --release
```

## Usage

### Basic Commands

```bash
# Validate a constraint file
slvsx validate input.json

# Solve constraints
slvsx solve input.json

# Export to SVG
slvsx export input.json --format svg --output output.svg

# Export to STL
slvsx export input.json --format stl --output output.stl
```

### JSON Format

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "parameters": {
    "module": 2.0,
    "sun_teeth": 24,
    "planet_teeth": 12,
    "ring_teeth": 72
  },
  "entities": [
    {
      "type": "gear",
      "id": "sun",
      "center": [0, 0, 0],
      "teeth": "$sun_teeth",
      "module": "$module",
      "internal": false
    }
  ],
  "constraints": [
    {
      "type": "mesh",
      "gear1": "sun",
      "gear2": "planet1"
    }
  ]
}
```

## Examples

See the `examples/` directory for complete examples including:
- Simple gear pairs
- Planetary gear systems
- Double planetary systems with triangular meshing
- Complex multi-stage gear trains

## Development

### Running Tests

```bash
cargo test --all-features
```

### Test Coverage

```bash
cargo tarpaulin --out Html --output-dir coverage
```

## Architecture

The project is organized as a Rust workspace with the following crates:

- `slvsx-core`: Core solver integration and constraint handling
- `slvsx-exporters`: SVG, STL, DXF export functionality
- `slvsx`: CLI application

## License

This project incorporates code from SolveSpace, which is licensed under GPLv3.
See LICENSE file for full details.

## Attribution

This project is based on:
- [SolveSpace](https://solvespace.com) by Jonathan Westhues and contributors
- libslvs - The SolveSpace constraint solver library

## Contributing

Contributions are welcome! Please ensure:
- All tests pass
- Code coverage remains at 100%
- Documentation is updated for new features