use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use slvsx_core::{
    solver::{Solver, SolverConfig},
    InputDocument,
};
use std::io::{self, BufRead, BufReader, Write};

/// MCP JSON-RPC request
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// MCP JSON-RPC response
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

/// MCP JSON-RPC error
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// MCP server capabilities
#[derive(Debug, Serialize)]
struct ServerCapabilities {
    tools: ToolCapabilities,
}

#[derive(Debug, Serialize)]
struct ToolCapabilities {}

/// MCP tool definition
#[derive(Debug, Serialize)]
struct Tool {
    name: String,
    description: String,
    input_schema: Value,
}

/// Initialize request params
#[derive(Debug, Deserialize)]
struct InitializeParams {
    protocol_version: String,
    capabilities: Value,
    client_info: Option<Value>,
}

/// Initialize result
#[derive(Debug, Serialize)]
struct InitializeResult {
    protocol_version: String,
    capabilities: ServerCapabilities,
    server_info: ServerInfo,
}

#[derive(Debug, Serialize)]
struct ServerInfo {
    name: String,
    version: String,
}

/// List tools result
#[derive(Debug, Serialize)]
struct ListToolsResult {
    tools: Vec<Tool>,
}

/// Call tool params
#[derive(Debug, Deserialize)]
struct CallToolParams {
    name: String,
    #[serde(default)]
    arguments: Value,
}

/// Tool result content
#[derive(Debug, Serialize)]
struct ToolContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

/// Tool result
#[derive(Debug, Serialize)]
struct ToolResult {
    content: Vec<ToolContent>,
}

pub fn run_mcp_server() -> Result<()> {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(&mut stdin_lock);

    let mut initialized = false;
    let mut request_id: Option<Value> = None;

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        
        if bytes_read == 0 {
            break; // EOF
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse JSON-RPC request
        let request: JsonRpcRequest = match serde_json::from_str(line) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Failed to parse request: {}", e);
                continue;
            }
        };

        request_id = request.id.clone();

        // Handle initialize
        if request.method == "initialize" {
            let params: InitializeParams = serde_json::from_value(
                request.params.unwrap_or(Value::Object(serde_json::Map::new()))
            )?;

            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id.clone(),
                result: Some(serde_json::to_value(InitializeResult {
                    protocol_version: "2024-11-05".to_string(),
                    capabilities: ServerCapabilities {
                        tools: ToolCapabilities {},
                    },
                    server_info: ServerInfo {
                        name: "slvsx".to_string(),
                        version: env!("CARGO_PKG_VERSION").to_string(),
                    },
                })?),
                error: None,
            };

            let response_json = serde_json::to_string(&response)?;
            writeln!(stdout, "{}", response_json)?;
            stdout.flush()?;
            initialized = true;
            continue;
        }

        // Handle initialized notification
        if request.method == "notifications/initialized" {
            continue;
        }

        if !initialized {
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id.clone(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32002,
                    message: "Server not initialized".to_string(),
                    data: None,
                }),
            };
            let response_json = serde_json::to_string(&response)?;
            writeln!(stdout, "{}", response_json)?;
            stdout.flush()?;
            continue;
        }

        // Handle list tools
        if request.method == "tools/list" {
            let tools = vec![
                Tool {
                    name: "solve_constraints".to_string(),
                    description: "Solve geometric constraints using SLVSX solver".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "constraints": {
                                "type": "object",
                                "description": "JSON constraint document following SLVSX schema"
                            }
                        },
                        "required": ["constraints"]
                    }),
                },
                Tool {
                    name: "validate_constraints".to_string(),
                    description: "Validate a constraint document without solving".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "constraints": {
                                "type": "object",
                                "description": "JSON constraint document to validate"
                            }
                        },
                        "required": ["constraints"]
                    }),
                },
                Tool {
                    name: "export_solution".to_string(),
                    description: "Solve constraints and export result to SVG, DXF, STL, or SLVS format".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "constraints": {
                                "type": "object",
                                "description": "JSON constraint document"
                            },
                            "format": {
                                "type": "string",
                                "enum": ["svg", "dxf", "stl", "slvs"],
                                "default": "svg"
                            },
                            "view": {
                                "type": "string",
                                "enum": ["xy", "xz", "yz"],
                                "default": "xy"
                            }
                        },
                        "required": ["constraints"]
                    }),
                },
                Tool {
                    name: "get_capabilities".to_string(),
                    description: "Get supported constraint types and export formats".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {}
                    }),
                },
                Tool {
                    name: "render_solution".to_string(),
                    description: "Solve constraints and return SVG as base64-encoded string".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "constraints": {
                                "type": "object",
                                "description": "JSON constraint document"
                            },
                            "view": {
                                "type": "string",
                                "enum": ["xy", "xz", "yz"],
                                "default": "xy"
                            }
                        },
                        "required": ["constraints"]
                    }),
                },
            ];

            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id.clone(),
                result: Some(serde_json::to_value(ListToolsResult { tools })?),
                error: None,
            };

            let response_json = serde_json::to_string(&response)?;
            writeln!(stdout, "{}", response_json)?;
            stdout.flush()?;
            continue;
        }

        // Handle tool calls
        if request.method == "tools/call" {
            let params: CallToolParams = serde_json::from_value(
                request.params.unwrap_or(Value::Object(serde_json::Map::new()))
            ).context("Failed to parse tool call params")?;

            let result = match handle_tool_call(&params.name, &params.arguments)? {
                Ok(content) => ToolResult { content },
                Err(error_msg) => ToolResult {
                    content: vec![ToolContent {
                        content_type: "text".to_string(),
                        text: format!("Error: {}", error_msg),
                    }],
                },
            };

            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id.clone(),
                result: Some(serde_json::to_value(result)?),
                error: None,
            };

            let response_json = serde_json::to_string(&response)?;
            writeln!(stdout, "{}", response_json)?;
            stdout.flush()?;
            continue;
        }

        // Unknown method
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request_id.clone(),
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
        };
        let response_json = serde_json::to_string(&response)?;
        writeln!(stdout, "{}", response_json)?;
        stdout.flush()?;
    }

    Ok(())
}

fn handle_tool_call(name: &str, arguments: &Value) -> Result<Result<Vec<ToolContent>, String>> {
    match name {
        "solve_constraints" => {
            let constraints = arguments
                .get("constraints")
                .ok_or_else(|| "Missing 'constraints' parameter".to_string())?;

            let doc: InputDocument = serde_json::from_value(constraints.clone())
                .map_err(|e| format!("Invalid constraint document: {}", e))?;

            let solver = Solver::new(SolverConfig::default());
            let result = solver.solve(&doc)
                .map_err(|e| format!("Solver error: {}", e))?;

            let result_json = serde_json::to_string_pretty(&result)
                .map_err(|e| format!("Failed to serialize result: {}", e))?;

            Ok(Ok(vec![ToolContent {
                content_type: "text".to_string(),
                text: result_json,
            }]))
        }
        "validate_constraints" => {
            let constraints = arguments
                .get("constraints")
                .ok_or_else(|| "Missing 'constraints' parameter".to_string())?;

            let doc: InputDocument = serde_json::from_value(constraints.clone())
                .map_err(|e| format!("Invalid constraint document: {}", e))?;

            let validator = slvsx_core::validator::Validator::new();
            validator.validate(&doc)
                .map_err(|e| format!("Validation failed: {}", e))?;

            Ok(Ok(vec![ToolContent {
                content_type: "text".to_string(),
                text: "✓ Document is valid".to_string(),
            }]))
        }
        "export_solution" => {
            let constraints = arguments
                .get("constraints")
                .ok_or_else(|| "Missing 'constraints' parameter".to_string())?;

            let format = arguments
                .get("format")
                .and_then(|v| v.as_str())
                .unwrap_or("svg");

            let view = arguments
                .get("view")
                .and_then(|v| v.as_str())
                .unwrap_or("xy");

            let doc: InputDocument = serde_json::from_value(constraints.clone())
                .map_err(|e| format!("Invalid constraint document: {}", e))?;

            let solver = Solver::new(SolverConfig::default());
            let result = solver.solve(&doc)
                .map_err(|e| format!("Solver error: {}", e))?;

            let entities = result.entities.unwrap_or_default();

            let output_data = match format {
                "svg" => {
                    use slvsx_exporters::svg::{SvgExporter, ViewPlane as SvgViewPlane};
                    let view_plane = match view {
                        "xy" => SvgViewPlane::XY,
                        "xz" => SvgViewPlane::XZ,
                        "yz" => SvgViewPlane::YZ,
                        _ => SvgViewPlane::XY,
                    };
                    let exporter = SvgExporter::new(view_plane);
                    exporter.export(&entities)
                        .map_err(|e| format!("Export error: {}", e))?
                        .into_bytes()
                }
                "dxf" => {
                    use slvsx_exporters::dxf::DxfExporter;
                    let exporter = DxfExporter::new();
                    exporter.export(&entities)
                        .map_err(|e| format!("Export error: {}", e))?
                        .into_bytes()
                }
                "slvs" => {
                    use slvsx_exporters::slvs::SlvsExporter;
                    let exporter = SlvsExporter::new();
                    exporter.export(&entities)
                        .map_err(|e| format!("Export error: {}", e))?
                        .into_bytes()
                }
                "stl" => {
                    use slvsx_exporters::stl::StlExporter;
                    let exporter = StlExporter::new(100.0);
                    exporter.export(&entities)
                        .map_err(|e| format!("Export error: {}", e))?
                }
                _ => return Ok(Err(format!("Unsupported format: {}", format))),
            };

            let content = match format {
                "svg" | "dxf" | "slvs" => String::from_utf8(output_data)
                    .map_err(|e| format!("Failed to convert to string: {}", e))?,
                "stl" => {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD.encode(&output_data)
                }
                _ => unreachable!(),
            };

            Ok(Ok(vec![ToolContent {
                content_type: if format == "stl" { "text" } else { format }.to_string(),
                text: content,
            }]))
        }
        "get_capabilities" => {
            let version = env!("CARGO_PKG_VERSION");
            let capabilities = serde_json::json!({
                "version": version,
                "entities": ["point", "line", "circle", "arc", "plane"],
                "constraints": [
                    "coincident", "distance", "angle", "perpendicular", "parallel",
                    "horizontal", "vertical", "equal_length", "equal_radius", "tangent",
                    "point_on_line", "point_on_circle", "fixed"
                ],
                "export_formats": ["svg", "dxf", "slvs", "stl"],
                "units": ["mm", "cm", "m", "in", "ft"]
            });

            Ok(Ok(vec![ToolContent {
                content_type: "text".to_string(),
                text: serde_json::to_string_pretty(&capabilities)?,
            }]))
        }
        "render_solution" => {
            let constraints = arguments
                .get("constraints")
                .ok_or_else(|| "Missing 'constraints' parameter".to_string())?;

            let view = arguments
                .get("view")
                .and_then(|v| v.as_str())
                .unwrap_or("xy");

            let doc: InputDocument = serde_json::from_value(constraints.clone())
                .map_err(|e| format!("Invalid constraint document: {}", e))?;

            let solver = Solver::new(SolverConfig::default());
            let result = solver.solve(&doc)
                .map_err(|e| format!("Solver error: {}", e))?;

            let entities = result.entities.unwrap_or_default();

            use slvsx_exporters::svg::{SvgExporter, ViewPlane as SvgViewPlane};
            let view_plane = match view {
                "xy" => SvgViewPlane::XY,
                "xz" => SvgViewPlane::XZ,
                "yz" => SvgViewPlane::YZ,
                _ => SvgViewPlane::XY,
            };
            let exporter = SvgExporter::new(view_plane);
            let svg = exporter.export(&entities)
                .map_err(|e| format!("Export error: {}", e))?;

            // Encode SVG as base64
            let svg_bytes = svg.into_bytes();
            use base64::Engine;
            let base64_svg = base64::engine::general_purpose::STANDARD.encode(&svg_bytes);

            Ok(Ok(vec![ToolContent {
                content_type: "text".to_string(),
                text: base64_svg,
            }]))
        }
        _ => Ok(Err(format!("Unknown tool: {}", name))),
    }
}

