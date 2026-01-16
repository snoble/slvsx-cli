use slvsx_core::ir::ResolvedEntity;
use std::collections::HashMap;

pub struct DxfExporter {
    precision: usize,
}

impl Default for DxfExporter {
    fn default() -> Self {
        Self { precision: 6 }
    }
}

impl DxfExporter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn export(&self, entities: &HashMap<String, ResolvedEntity>) -> anyhow::Result<String> {
        let mut dxf = String::new();

        // DXF header
        dxf.push_str("0\nSECTION\n2\nHEADER\n0\nENDSEC\n");

        // Entities section
        dxf.push_str("0\nSECTION\n2\nENTITIES\n");

        for (_id, entity) in entities {
            match entity {
                ResolvedEntity::Point { at } => {
                    // DXF POINT entity
                    dxf.push_str(&format!(
                        "0\nPOINT\n8\n0\n10\n{:.p$}\n20\n{:.p$}\n30\n{:.p$}\n",
                        at.get(0).copied().unwrap_or(0.0),
                        at.get(1).copied().unwrap_or(0.0),
                        at.get(2).copied().unwrap_or(0.0),
                        p = self.precision
                    ));
                }
                ResolvedEntity::Circle { center, diameter, .. } => {
                    // DXF CIRCLE entity
                    dxf.push_str(&format!(
                        "0\nCIRCLE\n8\n0\n10\n{:.p$}\n20\n{:.p$}\n30\n{:.p$}\n40\n{:.p$}\n",
                        center.get(0).copied().unwrap_or(0.0),
                        center.get(1).copied().unwrap_or(0.0),
                        center.get(2).copied().unwrap_or(0.0),
                        diameter / 2.0,
                        p = self.precision
                    ));
                }
                ResolvedEntity::Line { p1, p2 } => {
                    // DXF LINE entity
                    dxf.push_str(&format!(
                        "0\nLINE\n8\n0\n10\n{:.p$}\n20\n{:.p$}\n30\n{:.p$}\n11\n{:.p$}\n21\n{:.p$}\n31\n{:.p$}\n",
                        p1.get(0).copied().unwrap_or(0.0),
                        p1.get(1).copied().unwrap_or(0.0),
                        p1.get(2).copied().unwrap_or(0.0),
                        p2.get(0).copied().unwrap_or(0.0),
                        p2.get(1).copied().unwrap_or(0.0),
                        p2.get(2).copied().unwrap_or(0.0),
                        p = self.precision
                    ));
                }
                ResolvedEntity::Arc { center, start, end, .. } => {
                    // DXF ARC entity - approximate as line from start to end for now
                    // TODO: Implement proper DXF ARC with angles
                    dxf.push_str(&format!(
                        "0\nLINE\n8\n0\n10\n{:.p$}\n20\n{:.p$}\n30\n{:.p$}\n11\n{:.p$}\n21\n{:.p$}\n31\n{:.p$}\n",
                        start.get(0).copied().unwrap_or(0.0),
                        start.get(1).copied().unwrap_or(0.0),
                        start.get(2).copied().unwrap_or(0.0),
                        end.get(0).copied().unwrap_or(0.0),
                        end.get(1).copied().unwrap_or(0.0),
                        end.get(2).copied().unwrap_or(0.0),
                        p = self.precision
                    ));
                    let _ = center; // Suppress unused warning
                }
                ResolvedEntity::Cubic { start, control1: _, control2: _, end } => {
                    // DXF doesn't have native cubic bezier - approximate as line from start to end
                    // TODO: Implement as SPLINE or polyline approximation
                    dxf.push_str(&format!(
                        "0\nLINE\n8\n0\n10\n{:.p$}\n20\n{:.p$}\n30\n{:.p$}\n11\n{:.p$}\n21\n{:.p$}\n31\n{:.p$}\n",
                        start.get(0).copied().unwrap_or(0.0),
                        start.get(1).copied().unwrap_or(0.0),
                        start.get(2).copied().unwrap_or(0.0),
                        end.get(0).copied().unwrap_or(0.0),
                        end.get(1).copied().unwrap_or(0.0),
                        end.get(2).copied().unwrap_or(0.0),
                        p = self.precision
                    ));
                }
            }
        }

        dxf.push_str("0\nENDSEC\n0\nEOF\n");
        Ok(dxf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dxf_exporter_default() {
        let exporter = DxfExporter::default();
        assert_eq!(exporter.precision, 6);
    }

    #[test]
    fn test_dxf_exporter_new() {
        let exporter = DxfExporter::new();
        assert_eq!(exporter.precision, 6);
    }

    #[test]
    fn test_export_empty() {
        let exporter = DxfExporter::new();
        let entities = HashMap::new();
        let dxf = exporter.export(&entities).unwrap();
        assert!(dxf.contains("SECTION"));
        assert!(dxf.contains("ENTITIES"));
        assert!(dxf.contains("EOF"));
    }

    #[test]
    fn test_export_point() {
        let exporter = DxfExporter::new();
        let mut entities = HashMap::new();
        entities.insert(
            "p1".to_string(),
            ResolvedEntity::Point {
                at: vec![10.0, 20.0, 30.0],
            },
        );

        let dxf = exporter.export(&entities).unwrap();
        assert!(dxf.contains("POINT"));
        assert!(dxf.contains("10\n10.000000"));
        assert!(dxf.contains("20\n20.000000"));
        assert!(dxf.contains("30\n30.000000"));
    }

    #[test]
    fn test_export_circle() {
        let exporter = DxfExporter::new();
        let mut entities = HashMap::new();
        entities.insert(
            "c1".to_string(),
            ResolvedEntity::Circle {
                center: vec![0.0, 0.0, 0.0],
                diameter: 100.0,
                normal: vec![0.0, 0.0, 1.0],
            },
        );

        let dxf = exporter.export(&entities).unwrap();
        assert!(dxf.contains("CIRCLE"));
        assert!(dxf.contains("40\n50.000000")); // radius = diameter/2
    }

    #[test]
    fn test_export_line() {
        let exporter = DxfExporter::new();
        let mut entities = HashMap::new();
        entities.insert(
            "l1".to_string(),
            ResolvedEntity::Line {
                p1: vec![0.0, 0.0, 0.0],
                p2: vec![100.0, 100.0, 0.0],
            },
        );

        let dxf = exporter.export(&entities).unwrap();
        assert!(dxf.contains("LINE"));
        assert!(dxf.contains("11\n100.000000"));
        assert!(dxf.contains("21\n100.000000"));
    }

    #[test]
    fn test_missing_coordinates() {
        let exporter = DxfExporter::new();
        let mut entities = HashMap::new();
        entities.insert("p1".to_string(), ResolvedEntity::Point { at: vec![10.0] });

        let dxf = exporter.export(&entities).unwrap();
        assert!(dxf.contains("10\n10.000000"));
        assert!(dxf.contains("20\n0.000000")); // Missing coords default to 0
        assert!(dxf.contains("30\n0.000000"));
    }
}
