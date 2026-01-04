use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use slvsx_core::{
    solver::{Solver, SolverConfig},
    InputDocument,
};
use std::fs;
use std::io::{self, Read, Write};

mod json_error;
mod mcp;
use json_error::parse_json_with_context;

#[derive(Parser)]
#[command(name = "slvsx")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum ExportFormat {
    Svg,
    Dxf,
    Slvs,
    Stl,
}

#[derive(Clone, ValueEnum)]
enum ViewPlane {
    Xy,
    Xz,
    Yz,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate input file schema and references
    Validate {
        /// Input file path (use - for stdin)
        file: String,
    },
    /// Solve the constraint system
    Solve {
        /// Input file path (use - for stdin)
        file: String,
    },
    /// Export solved system to various formats
    Export {
        /// Input file path (use - for stdin)
        file: String,

        #[arg(short, long, default_value = "svg")]
        format: ExportFormat,

        #[arg(short, long, default_value = "xy")]
        view: ViewPlane,

        #[arg(short, long)]
        output: Option<String>,
    },
    /// Show capabilities
    Capabilities,
    /// Run MCP server mode (for AI agent integration)
    McpServer,
}

fn read_input(path: &str) -> Result<String> {
    if path == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        Ok(fs::read_to_string(path)?)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { file } => {
            let input = read_input(&file)?;
            let doc: InputDocument = parse_json_with_context(&input, &file)?;

            // Validate the document
            let validator = slvsx_core::validator::Validator::new();
            validator.validate(&doc)?;

            eprintln!("✓ Document is valid");
            Ok(())
        }
        Commands::Solve { file } => {
            let input = read_input(&file)?;
            let doc: InputDocument = parse_json_with_context(&input, &file)?;

            // Mock solve for now
            let solver = Solver::new(SolverConfig::default());
            let result = solver.solve(&doc)?;

            // Generic constraint solving - no gear-specific validation

            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        Commands::Export {
            file,
            format,
            view,
            output,
        } => {
            let input = read_input(&file)?;
            let doc: InputDocument = parse_json_with_context(&input, &file)?;

            // First solve the constraints
            let solver = Solver::new(SolverConfig::default());
            let result = solver.solve(&doc)?;

            // Use the solved entities for export
            let entities = result.entities.unwrap_or_default();

            let output_data = match format {
                ExportFormat::Svg => {
                    use slvsx_exporters::svg::{SvgExporter, ViewPlane as SvgViewPlane};
                    let view_plane = match view {
                        ViewPlane::Xy => SvgViewPlane::XY,
                        ViewPlane::Xz => SvgViewPlane::XZ,
                        ViewPlane::Yz => SvgViewPlane::YZ,
                    };
                    let exporter = SvgExporter::new(view_plane);
                    exporter.export(&entities)?.into_bytes()
                }
                ExportFormat::Dxf => {
                    use slvsx_exporters::dxf::DxfExporter;
                    let exporter = DxfExporter::new();
                    exporter.export(&entities)?.into_bytes()
                }
                ExportFormat::Slvs => {
                    use slvsx_exporters::slvs::SlvsExporter;
                    let exporter = SlvsExporter::new();
                    exporter.export(&entities)?.into_bytes()
                }
                ExportFormat::Stl => {
                    use slvsx_exporters::stl::StlExporter;
                    let exporter = StlExporter::new(100.0);
                    exporter.export(&entities)?
                }
            };

            if let Some(output_path) = output {
                fs::write(output_path, output_data)?;
            } else {
                io::stdout().write_all(&output_data)?;
            }

            Ok(())
        }
        Commands::Capabilities => {
            let version = env!("CARGO_PKG_VERSION");
            println!(
                r#"{{
  "version": "{}",
  "entities": ["point", "line", "circle", "arc", "plane"],
  "constraints": [
    "coincident", "distance", "angle", "perpendicular", "parallel",
    "horizontal", "vertical", "equal_length", "equal_radius", "tangent",
    "point_on_line", "point_on_circle", "fixed"
  ],
  "export_formats": ["svg", "dxf", "slvs", "stl"],
  "units": ["mm", "cm", "m", "in", "ft"]
}}"#,
                version
            );
            Ok(())
        }
        Commands::McpServer => {
            mcp::run_mcp_server()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        // Test that CLI parsing works
        let cli = Cli::parse_from(["slvsx", "capabilities"]);
        matches!(cli.command, Commands::Capabilities);
    }
}
