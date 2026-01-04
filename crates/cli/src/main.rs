use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

mod commands;
mod io;
mod json_error;

use commands::{handle_capabilities, handle_export, handle_solve, handle_validate};
use io::{create_input_reader, create_output_writer};
use io::StderrWriter;

#[derive(Parser)]
#[command(name = "slvsx")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Re-export for clap ValueEnum
#[derive(Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum ExportFormat {
    Svg,
    Dxf,
    Slvs,
    Stl,
}

#[derive(Clone, Debug, PartialEq, clap::ValueEnum)]
pub enum ViewPlane {
    Xy,
    Xz,
    Yz,
}

impl From<ExportFormat> for commands::ExportFormat {
    fn from(f: ExportFormat) -> Self {
        match f {
            ExportFormat::Svg => commands::ExportFormat::Svg,
            ExportFormat::Dxf => commands::ExportFormat::Dxf,
            ExportFormat::Slvs => commands::ExportFormat::Slvs,
            ExportFormat::Stl => commands::ExportFormat::Stl,
        }
    }
}

impl From<ViewPlane> for commands::ViewPlane {
    fn from(v: ViewPlane) -> Self {
        match v {
            ViewPlane::Xy => commands::ViewPlane::Xy,
            ViewPlane::Xz => commands::ViewPlane::Xz,
            ViewPlane::Yz => commands::ViewPlane::Yz,
        }
    }
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { file } => {
            let mut reader = create_input_reader(&file);
            let mut error_writer = StderrWriter;
            handle_validate(reader.as_mut(), &file, &mut error_writer)
        }
        Commands::Solve { file } => {
            let mut reader = create_input_reader(&file);
            let mut writer = create_output_writer(None);
            handle_solve(reader.as_mut(), writer.as_mut(), &file)
        }
        Commands::Export {
            file,
            format,
            view,
            output,
        } => {
            let mut reader = create_input_reader(&file);
            let mut writer = create_output_writer(output.as_deref());
            handle_export(reader.as_mut(), writer.as_mut(), &file, format.into(), view.into())
        }
        Commands::Capabilities => {
            let mut writer = create_output_writer(None);
            handle_capabilities(writer.as_mut())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_capabilities() {
        let cli = Cli::parse_from(["slvsx", "capabilities"]);
        matches!(cli.command, Commands::Capabilities);
    }

    #[test]
    fn test_cli_parse_validate() {
        let cli = Cli::parse_from(["slvsx", "validate", "file.json"]);
        match cli.command {
            Commands::Validate { file } => assert_eq!(file, "file.json"),
            _ => panic!("Expected Validate command"),
        }
    }

    #[test]
    fn test_cli_parse_solve() {
        let cli = Cli::parse_from(["slvsx", "solve", "-"]);
        match cli.command {
            Commands::Solve { file } => assert_eq!(file, "-"),
            _ => panic!("Expected Solve command"),
        }
    }

    #[test]
    fn test_cli_parse_export_defaults() {
        let cli = Cli::parse_from(["slvsx", "export", "file.json"]);
        match cli.command {
            Commands::Export { format, view, .. } => {
                assert_eq!(format, ExportFormat::Svg);
                assert_eq!(view, ViewPlane::Xy);
            }
            _ => panic!("Expected Export command"),
        }
    }

    #[test]
    fn test_cli_parse_export_with_format() {
        let cli = Cli::parse_from(["slvsx", "export", "-f", "dxf", "file.json"]);
        match cli.command {
            Commands::Export { format, .. } => {
                assert_eq!(format, ExportFormat::Dxf);
            }
            _ => panic!("Expected Export command"),
        }
    }

    #[test]
    fn test_cli_parse_export_with_view() {
        let cli = Cli::parse_from(["slvsx", "export", "-v", "xz", "file.json"]);
        match cli.command {
            Commands::Export { view, .. } => {
                assert_eq!(view, ViewPlane::Xz);
            }
            _ => panic!("Expected Export command"),
        }
    }

    #[test]
    fn test_cli_parse_export_with_output() {
        let cli = Cli::parse_from(["slvsx", "export", "--output", "out.svg", "file.json"]);
        match cli.command {
            Commands::Export { output, .. } => {
                assert_eq!(output, Some("out.svg".to_string()));
            }
            _ => panic!("Expected Export command"),
        }
    }
}
