use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use slvsx_core::{InputDocument, solver::{Solver, SolverConfig}};
use std::fs;
use std::io::{self, Read, Write};

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
            let doc: InputDocument = serde_json::from_str(&input)?;
            
            // Validate the document
            let validator = slvsx_core::validator::Validator::new();
            validator.validate(&doc)?;
            
            eprintln!("✓ Document is valid");
            Ok(())
        }
        Commands::Solve { file } => {
            let input = read_input(&file)?;
            let doc: InputDocument = serde_json::from_str(&input)?;
            
            // Mock solve for now
            let solver = Solver::new(SolverConfig::default());
            let result = solver.solve(&doc)?;
            
            // Run distance validator on the solution
            if let Some(entities) = &result.entities {
                use slvsx_core::distance_validator::{validate_all_distances, check_critical_distances};
                use slvsx_core::phase_validator::GearData;
                use std::collections::HashMap;
                
                // Convert entities to GearData for validation
                let mut gears = HashMap::new();
                let mut mesh_constraints = Vec::new();
                
                for (id, entity) in entities {
                    if let slvsx_core::ir::ResolvedEntity::Gear { 
                        center, teeth, module, phase, internal, ..
                    } = entity {
                        gears.insert(id.clone(), GearData {
                            id: id.clone(),
                            center: [center[0], center[1]],
                            teeth: *teeth,
                            module: *module,
                            phase: *phase,
                            internal: *internal,
                        });
                    }
                }
                
                // Get mesh constraints from the document
                for constraint in &doc.constraints {
                    if let slvsx_core::Constraint::Mesh { gear1, gear2 } = constraint {
                        mesh_constraints.push((gear1.clone(), gear2.clone()));
                    }
                }
                
                // Validate distances
                let validations = validate_all_distances(&gears, &mesh_constraints, 0.1);
                
                // Check for errors
                if let Err(errors) = check_critical_distances(&validations) {
                    eprintln!("⚠️  DISTANCE VALIDATION WARNINGS:");
                    for error in errors {
                        eprintln!("  - {}", error);
                    }
                } else {
                    eprintln!("✅ DISTANCE VALIDATION PASSED");
                }
                
                // Print distance report
                eprintln!("\nDistance Report:");
                for v in &validations {
                    if v.is_meshing {
                        eprintln!("  {} <-> {}: {:.2}mm (expected: {:.2}mm, error: {:.3}mm) {}",
                            v.gear1, v.gear2, v.actual_distance, v.expected_distance, v.error,
                            if v.validation_passed { "✓" } else { "✗" }
                        );
                    }
                }
            }
            
            println!("{}", serde_json::to_string_pretty(&result)?);
            Ok(())
        }
        Commands::Export { file, format, view, output } => {
            let input = read_input(&file)?;
            let doc: InputDocument = serde_json::from_str(&input)?;
            
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
            println!(r#"{{
  "version": "0.1.0",
  "entities": ["point", "line", "circle", "arc", "plane"],
  "constraints": [
    "coincident", "distance", "angle", "perpendicular", "parallel",
    "horizontal", "vertical", "equal_length", "equal_radius", "tangent",
    "point_on_line", "point_on_circle", "fixed"
  ],
  "export_formats": ["svg", "dxf", "slvs", "stl"],
  "units": ["mm", "cm", "m", "in", "ft"]
}}"#);
            Ok(())
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