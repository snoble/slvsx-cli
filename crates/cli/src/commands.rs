use crate::io::{create_input_reader, create_output_writer, ErrorWriter, InputReader, OutputWriter, StderrWriter};
use crate::json_error::parse_json_with_context;
use anyhow::Result;
use slvsx_core::{
    solver::{Solver, SolverConfig},
    InputDocument,
};
use slvsx_exporters::svg::ViewPlane as SvgViewPlane;

/// Export format enum
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExportFormat {
    Svg,
    Dxf,
    Slvs,
    Stl,
}

/// View plane enum
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewPlane {
    Xy,
    Xz,
    Yz,
}

impl From<ViewPlane> for SvgViewPlane {
    fn from(vp: ViewPlane) -> Self {
        match vp {
            ViewPlane::Xy => SvgViewPlane::XY,
            ViewPlane::Xz => SvgViewPlane::XZ,
            ViewPlane::Yz => SvgViewPlane::YZ,
        }
    }
}

/// Validate command handler
pub fn handle_validate<R: InputReader>(
    reader: &mut R,
    filename: &str,
    error_writer: &mut dyn ErrorWriter,
) -> Result<()> {
    let input = reader.read()?;
    let doc: InputDocument = parse_json_with_context(&input, filename)?;

    let validator = slvsx_core::validator::Validator::new();
    validator.validate(&doc)?;

    error_writer.write_error("✓ Document is valid")?;
    Ok(())
}

/// Solve command handler
pub fn handle_solve<R: InputReader, W: OutputWriter>(
    reader: &mut R,
    writer: &mut W,
    filename: &str,
) -> Result<()> {
    let input = reader.read()?;
    let doc: InputDocument = parse_json_with_context(&input, filename)?;

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc)?;

    let output = serde_json::to_string_pretty(&result)?;
    writer.write_str(&output)?;
    Ok(())
}

/// Export command handler
pub fn handle_export<R: InputReader, W: OutputWriter>(
    reader: &mut R,
    writer: &mut W,
    filename: &str,
    format: ExportFormat,
    view: ViewPlane,
) -> Result<()> {
    let input = reader.read()?;
    let doc: InputDocument = parse_json_with_context(&input, filename)?;

    // Solve the constraints
    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc)?;

    // Use the solved entities for export
    let entities = result.entities.unwrap_or_default();

    let output_data = export_entities(&entities, format, view)?;
    writer.write(&output_data)?;
    Ok(())
}

/// Export entities to the specified format
pub fn export_entities(
    entities: &std::collections::HashMap<String, slvsx_core::ir::ResolvedEntity>,
    format: ExportFormat,
    view: ViewPlane,
) -> Result<Vec<u8>> {
    match format {
        ExportFormat::Svg => {
            use slvsx_exporters::svg::SvgExporter;
            let exporter = SvgExporter::new(view.into());
            Ok(exporter.export(entities)?.into_bytes())
        }
        ExportFormat::Dxf => {
            use slvsx_exporters::dxf::DxfExporter;
            let exporter = DxfExporter::new();
            Ok(exporter.export(entities)?.into_bytes())
        }
        ExportFormat::Slvs => {
            use slvsx_exporters::slvs::SlvsExporter;
            let exporter = SlvsExporter::new();
            Ok(exporter.export(entities)?.into_bytes())
        }
        ExportFormat::Stl => {
            use slvsx_exporters::stl::StlExporter;
            let exporter = StlExporter::new(100.0);
            Ok(exporter.export(entities)?)
        }
    }
}

/// Capabilities command handler
pub fn handle_capabilities<W: OutputWriter>(writer: &mut W) -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let capabilities = format!(
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
    writer.write_str(&capabilities)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::tests::{MemoryErrorWriter, MemoryReader, MemoryWriter};
    use serde_json::json;

    #[test]
    fn test_handle_validate_valid() {
        let problem = json!({
            "schema": "slvs-json/1",
            "units": "mm",
            "entities": [
                {"type": "point", "id": "p1", "at": [0, 0, 0]}
            ],
            "constraints": [
                {"type": "fixed", "entity": "p1"}
            ]
        });

        let mut reader = MemoryReader::new(serde_json::to_string(&problem).unwrap());
        let mut error_writer = MemoryErrorWriter::new();

        let result = handle_validate(&mut reader, "test.json", &mut error_writer);
        assert!(result.is_ok());
        assert!(error_writer.messages().iter().any(|m| m.contains("valid")));
    }

    #[test]
    fn test_handle_validate_invalid() {
        let invalid = json!({
            "entities": []
        });

        let mut reader = MemoryReader::new(serde_json::to_string(&invalid).unwrap());
        let mut error_writer = MemoryErrorWriter::new();

        let result = handle_validate(&mut reader, "test.json", &mut error_writer);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_solve() {
        let problem = json!({
            "schema": "slvs-json/1",
            "units": "mm",
            "entities": [
                {"type": "point", "id": "p1", "at": [0, 0, 0]}
            ],
            "constraints": [
                {"type": "fixed", "entity": "p1"}
            ]
        });

        let mut reader = MemoryReader::new(serde_json::to_string(&problem).unwrap());
        let mut writer = MemoryWriter::new();

        let result = handle_solve(&mut reader, &mut writer, "test.json");
        assert!(result.is_ok());
        let output = writer.as_string();
        assert!(output.contains("\"status\""));
        assert!(output.contains("\"p1\""));
    }

    #[test]
    fn test_handle_capabilities() {
        let mut writer = MemoryWriter::new();
        let result = handle_capabilities(&mut writer);
        assert!(result.is_ok());
        let output = writer.as_string();
        assert!(output.contains("\"version\""));
        assert!(output.contains("\"entities\""));
        assert!(output.contains("\"constraints\""));
        assert!(output.contains("\"export_formats\""));
    }

    #[test]
    fn test_export_entities_svg() {
        use std::collections::HashMap;
        use slvsx_core::ir::ResolvedEntity;
        let entities = HashMap::new();
        let result = export_entities(&entities, ExportFormat::Svg, ViewPlane::Xy);
        assert!(result.is_ok());
        let svg = String::from_utf8_lossy(&result.unwrap());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_export_entities_dxf() {
        use std::collections::HashMap;
        let entities = HashMap::new();
        let result = export_entities(&entities, ExportFormat::Dxf, ViewPlane::Xy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_export_entities_slvs() {
        use std::collections::HashMap;
        let entities = HashMap::new();
        let result = export_entities(&entities, ExportFormat::Slvs, ViewPlane::Xy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_export_entities_stl() {
        use std::collections::HashMap;
        let entities = HashMap::new();
        let result = export_entities(&entities, ExportFormat::Stl, ViewPlane::Xy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_view_plane_conversion() {
        assert_eq!(SvgViewPlane::from(ViewPlane::Xy), SvgViewPlane::XY);
        assert_eq!(SvgViewPlane::from(ViewPlane::Xz), SvgViewPlane::XZ);
        assert_eq!(SvgViewPlane::from(ViewPlane::Yz), SvgViewPlane::YZ);
    }

    #[test]
    fn test_handle_export_all_formats() {
        let problem = json!({
            "schema": "slvs-json/1",
            "units": "mm",
            "entities": [
                {"type": "point", "id": "p1", "at": [0, 0, 0]}
            ],
            "constraints": [
                {"type": "fixed", "entity": "p1"}
            ]
        });

        for format in [ExportFormat::Svg, ExportFormat::Dxf, ExportFormat::Slvs] {
            let mut reader = MemoryReader::new(serde_json::to_string(&problem).unwrap());
            let mut writer = MemoryWriter::new();

            let result = handle_export(&mut reader, &mut writer, "test.json", format, ViewPlane::Xy);
            assert!(result.is_ok(), "Failed for format: {:?}", format);
        }
    }

    #[test]
    fn test_handle_export_all_views() {
        let problem = json!({
            "schema": "slvs-json/1",
            "units": "mm",
            "entities": [
                {"type": "point", "id": "p1", "at": [0, 0, 0]}
            ],
            "constraints": [
                {"type": "fixed", "entity": "p1"}
            ]
        });

        for view in [ViewPlane::Xy, ViewPlane::Xz, ViewPlane::Yz] {
            let mut reader = MemoryReader::new(serde_json::to_string(&problem).unwrap());
            let mut writer = MemoryWriter::new();

            let result = handle_export(&mut reader, &mut writer, "test.json", ExportFormat::Svg, view);
            assert!(result.is_ok(), "Failed for view: {:?}", view);
        }
    }
}

