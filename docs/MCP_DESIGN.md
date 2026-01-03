# MCP Server Design

This document describes the design for implementing native MCP server support in slvsx.

## Overview

The MCP (Model Context Protocol) server allows AI agents like Claude to interact with slvsx through a standardized protocol. The server runs locally and communicates via stdio.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Claude Desktop                          │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   MCP Client                         │   │
│  └─────────────────────────────────────────────────────┘   │
│                           │                                 │
│                      stdio (JSON-RPC)                       │
│                           │                                 │
└───────────────────────────┼─────────────────────────────────┘
                            │
┌───────────────────────────┼─────────────────────────────────┐
│                           ▼                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              slvsx mcp-server                        │   │
│  │                                                      │   │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  │   │
│  │  │   Tools     │  │  Resources   │  │  Protocol  │  │   │
│  │  │             │  │              │  │  Handler   │  │   │
│  │  │ solve       │  │ docs/        │  │            │  │   │
│  │  │ validate    │  │ schema/      │  │ JSON-RPC   │  │   │
│  │  │ render      │  │ examples/    │  │ stdio      │  │   │
│  │  │ export      │  │              │  │            │  │   │
│  │  └─────────────┘  └──────────────┘  └────────────┘  │   │
│  │         │                │                           │   │
│  │         ▼                ▼                           │   │
│  │  ┌─────────────────────────────────────────────────┐│   │
│  │  │              slvsx-core                          ││   │
│  │  │  Solver, Validator, Exporters                   ││   │
│  │  └─────────────────────────────────────────────────┘│   │
│  └─────────────────────────────────────────────────────┘   │
│                        slvsx binary                         │
└─────────────────────────────────────────────────────────────┘
```

## Dependencies

Using the official Rust MCP SDK:

```toml
[dependencies]
rmcp = { version = "0.8", features = ["server"] }
tokio = { version = "1", features = ["full"] }
```

See: https://github.com/modelcontextprotocol/rust-sdk

## CLI Integration

Add new subcommand to existing CLI:

```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...

    /// Run as MCP server (stdio transport)
    McpServer,
}
```

Usage:
```bash
slvsx mcp-server
```

Claude Desktop config:
```json
{
  "mcpServers": {
    "slvsx": {
      "command": "slvsx",
      "args": ["mcp-server"]
    }
  }
}
```

## Tools

### solve_constraints

Solve a geometric constraint system.

**Input:**
```json
{
  "constraints": {
    "schema_version": "0.3.0",
    "units": "mm",
    "entities": { ... },
    "constraints": [ ... ]
  }
}
```

**Output:**
```json
{
  "status": "ok",
  "entities": { ... },
  "diagnostics": {
    "dof": 0,
    "iterations": 5
  }
}
```

### validate_constraints

Check if a constraint document is valid without solving.

**Input:** Same as solve_constraints
**Output:** Validation result with any errors

### render_solution

Solve and return an inline SVG image that Claude can display.

**Input:**
```json
{
  "constraints": { ... },
  "view": "xy",       // optional: xy, xz, yz
  "width": 800,       // optional
  "height": 600       // optional
}
```

**Output:** MCP image content type with SVG data

```json
{
  "content": [
    {
      "type": "image",
      "data": "<base64-encoded-svg>",
      "mimeType": "image/svg+xml"
    }
  ]
}
```

### export_solution

Solve and export to a file format.

**Input:**
```json
{
  "constraints": { ... },
  "format": "svg"     // svg, dxf, stl
}
```

**Output:** File content as text or base64

### get_capabilities

List supported constraint types and features.

**Output:**
```json
{
  "version": "0.1.7",
  "entities": ["point", "line", "circle", ...],
  "constraints": ["distance", "angle", ...],
  "export_formats": ["svg", "dxf", "stl"]
}
```

## Resources

MCP resources allow Claude to browse and search documentation.

### Resource URIs

- `slvsx://docs/constraints` - List of constraint types with descriptions
- `slvsx://docs/entities` - List of entity types
- `slvsx://docs/schema` - JSON schema for input documents
- `slvsx://examples/{name}` - Example constraint problems

### Implementation

Resources are read-only and embedded in the binary:

```rust
#[derive(Resource)]
struct DocsResource {
    #[uri("slvsx://docs/constraints")]
    constraints: String,

    #[uri("slvsx://docs/entities")]
    entities: String,
}
```

## File Structure

```
crates/cli/src/
├── main.rs           # Add McpServer command
├── mcp/
│   ├── mod.rs        # MCP module
│   ├── server.rs     # Server setup and protocol
│   ├── tools.rs      # Tool implementations
│   └── resources.rs  # Resource implementations
└── ...
```

## Implementation Steps

1. **Add dependencies** to `crates/cli/Cargo.toml`
2. **Create mcp module** with server setup
3. **Implement tools** wrapping existing solver/exporter code
4. **Add resources** with embedded documentation
5. **Add McpServer command** to CLI
6. **Test with Claude Desktop**
7. **Update documentation**

## Testing

### Unit Tests
- Tool input/output serialization
- Resource URI parsing
- Error handling

### Integration Tests
- Full MCP protocol exchange
- Tool invocation with sample constraints
- Resource listing and reading

### Manual Testing
- Add to Claude Desktop
- Verify tools appear in tool list
- Solve constraints through chat
- View rendered images

## Error Handling

All errors are returned as MCP error responses:

```json
{
  "error": {
    "code": -32000,
    "message": "Solver error: overconstrained system",
    "data": {
      "dof": -2,
      "problematic_constraints": [...]
    }
  }
}
```

## Security Considerations

- Server only runs locally (stdio transport)
- No file system access beyond reading embedded resources
- No network access
- Input validation on all tool parameters

## Future Enhancements

- WebSocket transport for remote access
- Persistent session with undo/redo
- Interactive constraint editing
- Animation of mechanism motion
