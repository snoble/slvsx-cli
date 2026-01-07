use crate::io::{ErrorWriter, InputReader, OutputWriter};
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
    Isometric,
}

impl From<ViewPlane> for SvgViewPlane {
    fn from(vp: ViewPlane) -> Self {
        match vp {
            ViewPlane::Xy => SvgViewPlane::XY,
            ViewPlane::Xz => SvgViewPlane::XZ,
            ViewPlane::Yz => SvgViewPlane::YZ,
            ViewPlane::Isometric => SvgViewPlane::Isometric,
        }
    }
}

/// Validate command handler
pub fn handle_validate<R: InputReader + ?Sized>(
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
pub fn handle_solve<R: InputReader + ?Sized, W: OutputWriter + ?Sized>(
    reader: &mut R,
    writer: &mut W,
    filename: &str,
) -> Result<()> {
    let input = reader.read()?;
    let doc: InputDocument = parse_json_with_context(&input, filename)?;

    // Validate document before solving to catch errors early
    let validator = slvsx_core::validator::Validator::new();
    validator.validate(&doc)?;

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc)?;

    let output = serde_json::to_string_pretty(&result)?;
    writer.write_str(&output)?;
    Ok(())
}

/// Export command handler
pub fn handle_export<R: InputReader + ?Sized, W: OutputWriter + ?Sized>(
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
pub fn handle_capabilities<W: OutputWriter + ?Sized>(writer: &mut W) -> Result<()> {
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

/// Schema command handler - returns JSON schema for input documents
pub fn handle_schema<W: OutputWriter + ?Sized>(writer: &mut W) -> Result<()> {
    let schema = r##"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "SLVSX Constraint Document",
  "description": "JSON schema for SLVSX geometric constraint solver input",
  "type": "object",
  "required": ["schema", "entities", "constraints"],
  "properties": {
    "schema": {
      "type": "string",
      "const": "slvs-json/1",
      "description": "Schema version identifier"
    },
    "units": {
      "type": "string",
      "enum": ["mm", "cm", "m", "in", "ft"],
      "default": "mm",
      "description": "Unit of measurement for all numeric values"
    },
    "parameters": {
      "type": "object",
      "additionalProperties": { "type": "number" },
      "description": "Named parameters that can be referenced with $name syntax"
    },
    "entities": {
      "type": "array",
      "items": { "$ref": "#/definitions/entity" },
      "description": "Geometric entities (points, lines, circles, etc.)"
    },
    "constraints": {
      "type": "array",
      "items": { "$ref": "#/definitions/constraint" },
      "description": "Constraints between entities"
    }
  },
  "definitions": {
    "entity": {
      "oneOf": [
        {
          "type": "object",
          "required": ["type", "id", "at"],
          "properties": {
            "type": { "const": "point" },
            "id": { "type": "string" },
            "at": { "type": "array", "items": { "type": "number" }, "minItems": 3, "maxItems": 3 },
            "preserve": { "type": "boolean", "default": false }
          }
        },
        {
          "type": "object",
          "required": ["type", "id", "p1", "p2"],
          "properties": {
            "type": { "const": "line" },
            "id": { "type": "string" },
            "p1": { "type": "string", "description": "Reference to start point" },
            "p2": { "type": "string", "description": "Reference to end point" }
          }
        },
        {
          "type": "object",
          "required": ["type", "id", "center", "diameter"],
          "properties": {
            "type": { "const": "circle" },
            "id": { "type": "string" },
            "center": { "oneOf": [
              { "type": "array", "items": { "type": "number" }, "minItems": 3, "maxItems": 3 },
              { "type": "string", "description": "Reference to center point" }
            ]},
            "diameter": { "type": "number" },
            "normal": { "type": "array", "items": { "type": "number" }, "minItems": 3, "maxItems": 3, "default": [0,0,1] }
          }
        },
        {
          "type": "object",
          "required": ["type", "id", "origin", "normal"],
          "properties": {
            "type": { "const": "plane" },
            "id": { "type": "string" },
            "origin": { "type": "array", "items": { "type": "number" }, "minItems": 3, "maxItems": 3 },
            "normal": { "type": "array", "items": { "type": "number" }, "minItems": 3, "maxItems": 3 }
          }
        },
        {
          "type": "object",
          "required": ["type", "id", "at", "workplane"],
          "properties": {
            "type": { "const": "point2_d" },
            "id": { "type": "string" },
            "at": { "type": "array", "items": { "type": "number" }, "minItems": 2, "maxItems": 2 },
            "workplane": { "type": "string", "description": "Reference to plane entity" }
          }
        },
        {
          "type": "object",
          "required": ["type", "id", "p1", "p2", "workplane"],
          "properties": {
            "type": { "const": "line2_d" },
            "id": { "type": "string" },
            "p1": { "type": "string" },
            "p2": { "type": "string" },
            "workplane": { "type": "string", "description": "Reference to plane entity" }
          }
        },
        {
          "type": "object",
          "required": ["type", "id", "center", "start", "end"],
          "properties": {
            "type": { "const": "arc" },
            "id": { "type": "string" },
            "center": { "type": "string", "description": "Reference to center point" },
            "start": { "type": "string", "description": "Reference to start point" },
            "end": { "type": "string", "description": "Reference to end point" },
            "normal": { "type": "array", "items": { "type": "number" }, "minItems": 3, "maxItems": 3 }
          }
        }
      ]
    },
    "constraint": {
      "oneOf": [
        {
          "type": "object",
          "required": ["type", "entity"],
          "properties": {
            "type": { "const": "fixed" },
            "entity": { "type": "string" },
            "workplane": { "type": "string", "description": "Required for 2D points" }
          }
        },
        {
          "type": "object",
          "required": ["type", "between", "value"],
          "properties": {
            "type": { "const": "distance" },
            "between": { "type": "array", "items": { "type": "string" }, "minItems": 2, "maxItems": 2 },
            "value": { "type": "number" }
          }
        },
        {
          "type": "object",
          "required": ["type", "between", "value"],
          "properties": {
            "type": { "const": "angle" },
            "between": { "type": "array", "items": { "type": "string" }, "minItems": 2, "maxItems": 2 },
            "value": { "type": "number", "description": "Angle in degrees" }
          }
        },
        {
          "type": "object",
          "required": ["type", "a", "b"],
          "properties": {
            "type": { "const": "perpendicular" },
            "a": { "type": "string" },
            "b": { "type": "string" }
          }
        },
        {
          "type": "object",
          "required": ["type", "entities"],
          "properties": {
            "type": { "const": "parallel" },
            "entities": { "type": "array", "items": { "type": "string" }, "minItems": 2 }
          }
        },
        {
          "type": "object",
          "required": ["type", "a", "workplane"],
          "properties": {
            "type": { "const": "horizontal" },
            "a": { "type": "string", "description": "Line or point (2D only)" },
            "workplane": { "type": "string", "description": "Required - horizontal/vertical only work in 2D" }
          }
        },
        {
          "type": "object",
          "required": ["type", "a", "workplane"],
          "properties": {
            "type": { "const": "vertical" },
            "a": { "type": "string", "description": "Line or point (2D only)" },
            "workplane": { "type": "string", "description": "Required - horizontal/vertical only work in 2D" }
          }
        },
        {
          "type": "object",
          "required": ["type", "entities"],
          "properties": {
            "type": { "const": "equal_length" },
            "entities": { "type": "array", "items": { "type": "string" }, "minItems": 2 }
          }
        },
        {
          "type": "object",
          "required": ["type", "a", "b"],
          "properties": {
            "type": { "const": "equal_radius" },
            "a": { "type": "string" },
            "b": { "type": "string" }
          }
        },
        {
          "type": "object",
          "required": ["type", "a", "b"],
          "properties": {
            "type": { "const": "tangent" },
            "a": { "type": "string", "description": "Arc or line (NOT circle)" },
            "b": { "type": "string", "description": "Arc or line (NOT circle)" }
          }
        },
        {
          "type": "object",
          "required": ["type", "point", "line"],
          "properties": {
            "type": { "const": "point_on_line" },
            "point": { "type": "string" },
            "line": { "type": "string" }
          }
        },
        {
          "type": "object",
          "required": ["type", "point", "circle"],
          "properties": {
            "type": { "const": "point_on_circle" },
            "point": { "type": "string" },
            "circle": { "type": "string" }
          }
        },
        {
          "type": "object",
          "required": ["type", "a", "b", "workplane"],
          "properties": {
            "type": { "const": "symmetric_horizontal" },
            "a": { "type": "string" },
            "b": { "type": "string" },
            "workplane": { "type": "string" }
          }
        },
        {
          "type": "object",
          "required": ["type", "a", "b", "workplane"],
          "properties": {
            "type": { "const": "symmetric_vertical" },
            "a": { "type": "string" },
            "b": { "type": "string" },
            "workplane": { "type": "string" }
          }
        },
        {
          "type": "object",
          "required": ["type", "point", "of"],
          "properties": {
            "type": { "const": "midpoint" },
            "point": { "type": "string" },
            "of": { "type": "string", "description": "Reference to line" }
          }
        },
        {
          "type": "object",
          "required": ["type", "circle", "value"],
          "properties": {
            "type": { "const": "diameter" },
            "circle": { "type": "string" },
            "value": { "type": "number" }
          }
        }
      ]
    }
  }
}"##;
    writer.write_str(schema)?;
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
    fn test_handle_schema() {
        let mut writer = MemoryWriter::new();
        let result = handle_schema(&mut writer);
        assert!(result.is_ok());
        let output = writer.as_string();
        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&output).expect("Schema should be valid JSON");
        // Verify key schema elements
        assert_eq!(parsed["$schema"], "http://json-schema.org/draft-07/schema#");
        assert_eq!(parsed["title"], "SLVSX Constraint Document");
        assert!(parsed["definitions"]["entity"].is_object());
        assert!(parsed["definitions"]["constraint"].is_object());
        // Verify important constraints are documented
        let constraint_def = &parsed["definitions"]["constraint"]["oneOf"];
        assert!(constraint_def.is_array());
        // Check that horizontal/vertical document workplane requirement
        let schema_str = output.as_str();
        assert!(schema_str.contains("horizontal"));
        assert!(schema_str.contains("workplane"));
    }

    #[test]
    fn test_export_entities_svg() {
        use std::collections::HashMap;
        use slvsx_core::ir::ResolvedEntity;
        let entities = HashMap::new();
        let result = export_entities(&entities, ExportFormat::Svg, ViewPlane::Xy);
        assert!(result.is_ok());
        let result_bytes = result.unwrap();
        let svg = String::from_utf8_lossy(&result_bytes);
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
        assert!(matches!(SvgViewPlane::from(ViewPlane::Xy), SvgViewPlane::XY));
        assert!(matches!(SvgViewPlane::from(ViewPlane::Xz), SvgViewPlane::XZ));
        assert!(matches!(SvgViewPlane::from(ViewPlane::Yz), SvgViewPlane::YZ));
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

    #[test]
    fn test_handle_solve_error_paths() {
        // Test with invalid JSON
        let mut reader = MemoryReader::new("{invalid json}".to_string());
        let mut writer = MemoryWriter::new();
        let result = handle_solve(&mut reader, &mut writer, "test.json");
        assert!(result.is_err());

        // Test with invalid document structure - constraint references nonexistent entity
        let invalid = json!({
            "schema": "slvs-json/1",
            "units": "mm",
            "entities": [
                {"type": "point", "id": "p1", "at": [0, 0, 0]}
            ],
            "constraints": [
                {"type": "fixed", "entity": "nonexistent"}
            ]
        });
        let mut reader = MemoryReader::new(serde_json::to_string(&invalid).unwrap());
        let mut writer = MemoryWriter::new();
        // This should fail validation before solving
        let result = handle_solve(&mut reader, &mut writer, "test.json");
        assert!(result.is_err(), "Should fail validation for nonexistent entity reference");
        match result.unwrap_err().downcast_ref::<slvsx_core::error::Error>() {
            Some(slvsx_core::error::Error::InvalidInput { message, .. }) => {
                assert!(message.contains("unknown entity") || message.contains("nonexistent"));
            }
            _ => {} // Other error types are also acceptable
        }
    }

    #[test]
    fn test_handle_validate_error_paths() {
        // Test with invalid JSON
        let mut reader = MemoryReader::new("{invalid}".to_string());
        let mut error_writer = MemoryErrorWriter::new();
        let result = handle_validate(&mut reader, "test.json", &mut error_writer);
        assert!(result.is_err());

        // Test with missing schema
        let invalid = json!({
            "entities": []
        });
        let mut reader = MemoryReader::new(serde_json::to_string(&invalid).unwrap());
        let mut error_writer = MemoryErrorWriter::new();
        let result = handle_validate(&mut reader, "test.json", &mut error_writer);
        assert!(result.is_err());
    }

    #[test]
    fn test_export_entities_error_handling() {
        use std::collections::HashMap;
        // Test with empty entities (should not crash)
        let entities = HashMap::new();
        for format in [ExportFormat::Svg, ExportFormat::Dxf, ExportFormat::Slvs, ExportFormat::Stl] {
            let result = export_entities(&entities, format, ViewPlane::Xy);
            assert!(result.is_ok(), "Failed for format: {:?}", format);
        }
    }

    #[test]
    fn test_view_plane_all_variants() {
        // Test all view plane conversions
        assert!(matches!(SvgViewPlane::from(ViewPlane::Xy), SvgViewPlane::XY));
        assert!(matches!(SvgViewPlane::from(ViewPlane::Xz), SvgViewPlane::XZ));
        assert!(matches!(SvgViewPlane::from(ViewPlane::Yz), SvgViewPlane::YZ));
    }

    #[test]
    fn test_export_format_all_variants() {
        // Test that all export formats are covered
        let formats = [ExportFormat::Svg, ExportFormat::Dxf, ExportFormat::Slvs, ExportFormat::Stl];
        assert_eq!(formats.len(), 4);
    }
}

