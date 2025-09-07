# SLVSX - SolveSpace Constraint Solver CLI

> **✅ STATUS**: Real libslvs constraint solver integration complete! The CLI provides a generic interface to the SolveSpace constraint solver with support for points, lines, circles, and distance/fixed constraints.

A command-line interface for the SolveSpace geometric constraint solver that turns mechanical design from manual sketching and complex math into simple constraint specification. Perfect for AI agents and automated mechanical system generation.

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

### Example: Four-Bar Linkage

Without SLVSX, designing a four-bar linkage requires:
- **Complex trigonometry** to solve the position equations
- **Iterative calculations** for different input angles
- **Manual verification** that the linkage doesn't bind or reach singularities
- **Guess-and-check** for link length optimization

With SLVSX, you simply specify the constraints:

```json
{
  "schema": "slvs-json/1", 
  "units": "mm",
  "parameters": {
    "link1_length": 50.0,
    "link2_length": 80.0, 
    "link3_length": 70.0,
    "link4_length": 40.0,
    "input_angle": 45.0
  },
  "entities": [
    {"type": "point", "id": "ground_a", "at": [0, 0, 0]},
    {"type": "point", "id": "ground_b", "at": ["$link1_length", 0, 0]},
    {"type": "point", "id": "joint_p", "at": [10, 10, 0]},
    {"type": "point", "id": "joint_q", "at": [30, 20, 0]},
    {"type": "line", "id": "input_link", "p1": "ground_a", "p2": "joint_p"},
    {"type": "line", "id": "coupler", "p1": "joint_p", "p2": "joint_q"},
    {"type": "line", "id": "output_link", "p1": "joint_q", "p2": "ground_b"}
  ],
  "constraints": [
    {"type": "fixed", "entity": "ground_a"},
    {"type": "fixed", "entity": "ground_b"}, 
    {"type": "distance", "between": ["ground_a", "joint_p"], "value": "$link2_length"},
    {"type": "distance", "between": ["joint_p", "joint_q"], "value": "$link3_length"},
    {"type": "distance", "between": ["joint_q", "ground_b"], "value": "$link4_length"},
    {"type": "angle", "between": ["ground_link", "input_link"], "value": "$input_angle"}
  ]
}
```

The solver automatically:
✅ Calculates exact joint positions  
✅ Handles the complex trigonometry  
✅ Validates the configuration is feasible  
✅ Enables parameter sweeps for optimization  

```bash
# Solve the linkage - no math required!
slvsx solve examples/readme_example.json

# Export to SVG to visualize the solved linkage  
slvsx export examples/readme_example.json --format svg --output examples/outputs/linkage.svg
```

**Output: Solved Four-Bar Linkage**

![Four-Bar Linkage](examples/outputs/four_bar_linkage.svg)

*SLVSX automatically calculated the exact joint positions that satisfy all distance constraints (link lengths 80mm, 70mm, 40mm).*

## Examples

See the `examples/` directory for complete examples including:
- **Four-bar linkages** - Motion analysis without complex trigonometry
- **Simple gear pairs** - Automatic center distance calculation
- **Planetary systems** - Complex gear trains solved automatically  
- **Constraint problems** - Let the solver find valid configurations

Each example shows how SLVSX replaces manual math and guesswork with simple constraint specification.

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