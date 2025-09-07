# SLVSX MCP Server

Use SLVSX geometric constraint solver with Claude Desktop and other MCP-compatible AI assistants.

## Installation

### Option 1: Install from npm (when published)

```bash
npm install -g @slvsx/mcp-server
```

### Option 2: Install from source

```bash
# Clone and build SLVSX
git clone https://github.com/snoble/slvsx-cli.git
cd slvsx-cli
cargo build --release

# Install MCP server dependencies
npm install
```

## Configuration

Add to your Claude Desktop config file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "slvsx": {
      "command": "npx",
      "args": ["@slvsx/mcp-server"],
      "env": {
        "SLVSX_BINARY": "/path/to/slvsx"
      }
    }
  }
}
```

Or if installed from source:

```json
{
  "mcpServers": {
    "slvsx": {
      "command": "node",
      "args": ["/path/to/slvsx-cli/mcp-server.js"],
      "env": {
        "SLVSX_BINARY": "/path/to/slvsx-cli/target/release/slvsx"
      }
    }
  }
}
```

## Available Tools

Once configured, Claude will have access to these tools:

### `solve_constraints`
Solve geometric constraints and return the solved positions.

Example:
```
Use the solve_constraints tool to find the positions of a triangle with all sides equal to 100mm
```

### `validate_constraints`
Check if a constraint document is valid without solving.

### `export_to_svg`
Solve constraints and generate an SVG visualization.

Example:
```
Create a four-bar linkage mechanism and export it as SVG
```

### `get_schema`
Get the JSON schema for constraint documents.

### `create_example`
Generate example constraint documents (triangle, square, circle, linkage, parametric, 3d).

## Example Usage in Claude

Once configured, you can ask Claude things like:

- "Create a parametric box design with width 200mm and height 150mm"
- "Design a four-bar linkage with specific link lengths"
- "Solve for the position of points in a triangle where all sides are 100mm"
- "Create a mechanism with a crank that rotates 45 degrees"
- "Generate an SVG of a constrained mechanical system"

## How It Works

1. **Local Execution**: The MCP server runs on your machine, not in the cloud
2. **Binary Required**: You need the SLVSX binary built and accessible
3. **JSON Communication**: Claude sends constraint specifications as JSON
4. **Real Solving**: Uses the actual SolveSpace constraint solver engine

## Troubleshooting

### "SLVSX binary not found"
- Build the project: `cargo build --release`
- Or set `SLVSX_BINARY` environment variable in the config

### "Tool not available in Claude"
- Restart Claude Desktop after updating the config
- Check the config file syntax is valid JSON

### "Solve failed"
- The constraint system may be over-constrained or invalid
- Try using `validate_constraints` first to check the input

## Development

To modify the MCP server:

1. Edit `mcp-server.js`
2. Test locally: `node mcp-server.js`
3. The server uses stdio for communication with Claude

## Publishing to npm

When ready to publish:

```bash
npm login
npm publish --access public
```

Users can then install with:
```bash
npm install -g @slvsx/mcp-server
```

## License

GPL-3.0-or-later (due to SolveSpace dependency)