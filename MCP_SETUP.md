# MCP Server Setup for AI Agents

This guide helps AI agents use slvsx as an MCP (Model Context Protocol) server for solving geometry problems.

## Quick Start (For AI Agents)

### 1. Download and Build

```bash
# Clone the repository
git clone https://github.com/snoble/slvsx-cli.git
cd slvsx-cli

# Build the project (includes libslvs-static)
cd libslvs-static
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make
cd ../..

# Build slvsx
export SLVS_LIB_DIR=$PWD/libslvs-static/build
cargo build --release

# Verify it works
./target/release/slvsx --version
```

### 2. Download Pre-built Binary (Alternative)

Check the latest release for pre-built binaries:
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

### 3. Use as MCP Server

#### Option A: Python Wrapper (Recommended)
```python
from mcp_server.slvsx_mcp import SlvsxMCPServer

server = SlvsxMCPServer("./target/release/slvsx")

# Solve a simple triangle
problem = {
    "entities": [
        {"type": "point", "id": "p1", "at": [0, 0, 0]},
        {"type": "point", "id": "p2", "at": [100, 0, 0]},
        {"type": "point", "id": "p3", "at": [50, 50, 0]}
    ],
    "constraints": [
        {"type": "distance", "between": ["p1", "p2"], "value": 100},
        {"type": "distance", "between": ["p2", "p3"], "value": 70.71},
        {"type": "distance", "between": ["p3", "p1"], "value": 70.71}
    ],
    "units": "mm"
}

solution = server.solve_geometry(problem)
print(solution)
```

#### Option B: Direct CLI Usage
```bash
# Solve geometry from JSON
echo '{"schema":"slvs-json/1","entities":[...],"constraints":[...],"units":"mm"}' | ./slvsx solve -

# Export to SVG
echo '{"schema":"slvs-json/1","entities":[...],"constraints":[...],"units":"mm"}' | ./slvsx export -f svg -

# Validate problem
echo '{"schema":"slvs-json/1","entities":[...],"constraints":[...],"units":"mm"}' | ./slvsx validate -
```

## Example Problems

### 1. Simple Point
```json
{
  "schema": "slvs-json/1",
  "entities": [
    {"type": "point", "id": "p1", "at": [0, 0, 0]}
  ],
  "constraints": [],
  "units": "mm"
}
```

### 2. Constrained Line
```json
{
  "schema": "slvs-json/1",
  "entities": [
    {"type": "point", "id": "p1", "at": [0, 0, 0]},
    {"type": "point", "id": "p2", "at": [100, 0, 0]},
    {"type": "line", "id": "line1", "between": ["p1", "p2"]}
  ],
  "constraints": [
    {"type": "distance", "between": ["p1", "p2"], "value": 100}
  ],
  "units": "mm"
}
```

### 3. Triangle with Constraints
```json
{
  "schema": "slvs-json/1",
  "entities": [
    {"type": "point", "id": "p1", "at": [0, 0, 0]},
    {"type": "point", "id": "p2", "at": [100, 0, 0]},
    {"type": "point", "id": "p3", "at": [50, 86.6, 0]}
  ],
  "constraints": [
    {"type": "distance", "between": ["p1", "p2"], "value": 100},
    {"type": "distance", "between": ["p2", "p3"], "value": 100},
    {"type": "distance", "between": ["p3", "p1"], "value": 100}
  ],
  "units": "mm"
}
```

## API Reference

### Methods

#### `solve`
Solves geometric constraints and returns updated positions.

**Input**: Geometry problem in slvs-json format
**Output**: Solution with updated entity positions

#### `export_svg`
Exports the geometry to SVG format.

**Input**: Geometry problem
**Output**: SVG string

#### `validate`
Validates a geometry problem for correctness.

**Input**: Geometry problem
**Output**: Validation result

## Troubleshooting

### Binary not found
- Ensure you've built the project with `cargo build --release`
- Check that the binary is at `./target/release/slvsx`
- On Linux with --target builds, check `./target/x86_64-unknown-linux-gnu/release/slvsx`

### Proc-macro compilation errors
- This has been fixed in the latest version
- Ensure `.cargo/config.toml` doesn't have global rustflags

### Static linking issues
- The binary is statically linked with libslvs
- Only requires system libc (glibc on Linux)

## Integration Tips for AI Agents

1. **Use JSON format**: All input/output is JSON for easy parsing
2. **Start simple**: Begin with points and distance constraints
3. **Check capabilities**: Use the `capabilities` method to see what's available
4. **Handle errors**: Check the `status` field in responses
5. **Validate first**: Use `validate` before `solve` for complex problems

## Support

- GitHub Issues: https://github.com/snoble/slvsx-cli/issues
- Documentation: See examples/ directory for more complex scenarios