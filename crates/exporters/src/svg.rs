use slvsx_core::ir::ResolvedEntity;
use std::collections::HashMap;

pub struct SvgExporter {
    view_plane: ViewPlane,
    precision: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ViewPlane {
    XY,
    XZ,
    YZ,
}

impl Default for SvgExporter {
    fn default() -> Self {
        Self {
            view_plane: ViewPlane::XY,
            precision: 6,
        }
    }
}

impl SvgExporter {
    pub fn new(view_plane: ViewPlane) -> Self {
        Self {
            view_plane,
            precision: 6,
        }
    }

    pub fn export(&self, entities: &HashMap<String, ResolvedEntity>) -> anyhow::Result<String> {
        // Calculate bounding box
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for entity in entities.values() {
            match entity {
                ResolvedEntity::Circle { center, diameter } => {
                    let (cx, cy) = self.project_point(center);
                    let r = diameter / 2.0;
                    min_x = min_x.min(cx - r);
                    max_x = max_x.max(cx + r);
                    min_y = min_y.min(cy - r);
                    max_y = max_y.max(cy + r);
                }
                ResolvedEntity::Point { at } => {
                    let (x, y) = self.project_point(at);
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                }
                ResolvedEntity::Line { p1, p2 } => {
                    let (x1, y1) = self.project_point(p1);
                    let (x2, y2) = self.project_point(p2);
                    min_x = min_x.min(x1.min(x2));
                    max_x = max_x.max(x1.max(x2));
                    min_y = min_y.min(y1.min(y2));
                    max_y = max_y.max(y1.max(y2));
                }
            }
        }

        // Add padding
        let padding = 20.0;
        if min_x.is_finite() && max_x.is_finite() {
            min_x -= padding;
            max_x += padding;
            min_y -= padding;
            max_y += padding;
        } else {
            // Default view if no entities
            min_x = -100.0;
            max_x = 100.0;
            min_y = -100.0;
            max_y = 100.0;
        }

        let width = max_x - min_x;
        let height = max_y - min_y;

        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{:.1} {:.1} {:.1} {:.1}" width="800" height="800">"#,
            min_x, min_y, width, height
        );
        svg.push('\n');

        for (id, entity) in entities {
            match entity {
                ResolvedEntity::Point { at } => {
                    let (x, y) = self.project_point(at);
                    svg.push_str(&format!(
                        r#"  <circle id="{}" cx="{:.p$}" cy="{:.p$}" r="2" fill="black"/>"#,
                        id,
                        x,
                        y,
                        p = self.precision
                    ));
                    svg.push('\n');
                }
                ResolvedEntity::Circle { center, diameter } => {
                    let (cx, cy) = self.project_point(center);
                    svg.push_str(&format!(
                        r#"  <circle id="{}" cx="{:.p$}" cy="{:.p$}" r="{:.p$}" fill="none" stroke="black"/>"#,
                        id, cx, cy, diameter / 2.0, p = self.precision
                    ));
                    svg.push('\n');
                }
                ResolvedEntity::Line { p1, p2 } => {
                    let (x1, y1) = self.project_point(p1);
                    let (x2, y2) = self.project_point(p2);
                    svg.push_str(&format!(
                        r#"  <line id="{}" x1="{:.p$}" y1="{:.p$}" x2="{:.p$}" y2="{:.p$}" stroke="black"/>"#,
                        id, x1, y1, x2, y2, p = self.precision
                    ));
                    svg.push('\n');
                }
            }
        }

        svg.push_str("</svg>");
        Ok(svg)
    }

    fn project_point(&self, point: &[f64]) -> (f64, f64) {
        match self.view_plane {
            ViewPlane::XY => (
                point.get(0).copied().unwrap_or(0.0),
                point.get(1).copied().unwrap_or(0.0),
            ),
            ViewPlane::XZ => (
                point.get(0).copied().unwrap_or(0.0),
                point.get(2).copied().unwrap_or(0.0),
            ),
            ViewPlane::YZ => (
                point.get(1).copied().unwrap_or(0.0),
                point.get(2).copied().unwrap_or(0.0),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_exporter_default() {
        let exporter = SvgExporter::default();
        assert!(matches!(exporter.view_plane, ViewPlane::XY));
        assert_eq!(exporter.precision, 6);
    }

    #[test]
    fn test_svg_exporter_new() {
        let exporter = SvgExporter::new(ViewPlane::XZ);
        assert!(matches!(exporter.view_plane, ViewPlane::XZ));
    }

    #[test]
    fn test_export_empty() {
        let exporter = SvgExporter::default();
        let entities = HashMap::new();
        let svg = exporter.export(&entities).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_export_point() {
        let exporter = SvgExporter::default();
        let mut entities = HashMap::new();
        entities.insert(
            "p1".to_string(),
            ResolvedEntity::Point {
                at: vec![10.0, 20.0, 0.0],
            },
        );

        let svg = exporter.export(&entities).unwrap();
        assert!(svg.contains(r#"id="p1""#));
        assert!(svg.contains(r#"cx="10.000000""#));
        assert!(svg.contains(r#"cy="20.000000""#));
    }

    #[test]
    fn test_export_circle() {
        let exporter = SvgExporter::default();
        let mut entities = HashMap::new();
        entities.insert(
            "c1".to_string(),
            ResolvedEntity::Circle {
                center: vec![0.0, 0.0, 0.0],
                diameter: 50.0,
            },
        );

        let svg = exporter.export(&entities).unwrap();
        assert!(svg.contains(r#"id="c1""#));
        assert!(svg.contains(r#"r="25.000000""#));
    }

    #[test]
    fn test_export_line() {
        let exporter = SvgExporter::default();
        let mut entities = HashMap::new();
        entities.insert(
            "l1".to_string(),
            ResolvedEntity::Line {
                p1: vec![0.0, 0.0, 0.0],
                p2: vec![100.0, 100.0, 0.0],
            },
        );

        let svg = exporter.export(&entities).unwrap();
        assert!(svg.contains(r#"id="l1""#));
        assert!(svg.contains(r#"x2="100.000000""#));
        assert!(svg.contains(r#"y2="100.000000""#));
    }

    #[test]
    fn test_project_point_xy() {
        let exporter = SvgExporter::new(ViewPlane::XY);
        assert_eq!(exporter.project_point(&[1.0, 2.0, 3.0]), (1.0, 2.0));
    }

    #[test]
    fn test_project_point_xz() {
        let exporter = SvgExporter::new(ViewPlane::XZ);
        assert_eq!(exporter.project_point(&[1.0, 2.0, 3.0]), (1.0, 3.0));
    }

    #[test]
    fn test_project_point_yz() {
        let exporter = SvgExporter::new(ViewPlane::YZ);
        assert_eq!(exporter.project_point(&[1.0, 2.0, 3.0]), (2.0, 3.0));
    }

    #[test]
    fn test_project_point_missing_coords() {
        let exporter = SvgExporter::new(ViewPlane::XY);
        assert_eq!(exporter.project_point(&[1.0]), (1.0, 0.0));
        assert_eq!(exporter.project_point(&[]), (0.0, 0.0));
    }
}
