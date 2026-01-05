use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use regex::Regex;
use slvsx_core::ir::ResolvedEntity;
use std::collections::HashMap;
use std::io::Cursor;

/// Normalize a floating point value to avoid -0.0
/// This ensures consistent output across platforms
fn normalize_zero(v: f64) -> f64 {
    if v == 0.0 { 0.0_f64.abs() } else { v }
}

/// Format a float for SVG output, normalizing -0.0 to 0.0
fn fmt_svg(v: f64, precision: usize) -> String {
    format!("{:.p$}", normalize_zero(v), p = precision)
}

/// Sanitize an attribute value, replacing -0.0... patterns with 0.0...
fn sanitize_attr_value(value: &str) -> String {
    // Match -0.0 followed by any number of zeros (e.g., -0.000000)
    let re = Regex::new(r"-0\.0+").unwrap();
    re.replace_all(value, |caps: &regex::Captures| {
        caps[0].replacen("-", "", 1)
    }).to_string()
}

/// Post-process SVG using proper XML parsing to sanitize attribute values
fn sanitize_svg_attributes(svg: &str) -> anyhow::Result<String> {
    let mut reader = Reader::from_str(svg);
    reader.trim_text(false);
    
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    
    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let name = e.name();
                let name_str = std::str::from_utf8(name.as_ref())?;
                let mut elem = BytesStart::new(name_str.to_owned());
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = std::str::from_utf8(attr.key.as_ref())?.to_owned();
                    let value = std::str::from_utf8(&attr.value)?;
                    let sanitized = sanitize_attr_value(value);
                    elem.push_attribute((key.as_str(), sanitized.as_str()));
                }
                writer.write_event(Event::Start(elem))?;
            }
            Ok(Event::Empty(ref e)) => {
                let name = e.name();
                let name_str = std::str::from_utf8(name.as_ref())?;
                let mut elem = BytesStart::new(name_str.to_owned());
                for attr in e.attributes() {
                    let attr = attr?;
                    let key = std::str::from_utf8(attr.key.as_ref())?.to_owned();
                    let value = std::str::from_utf8(&attr.value)?;
                    let sanitized = sanitize_attr_value(value);
                    elem.push_attribute((key.as_str(), sanitized.as_str()));
                }
                writer.write_event(Event::Empty(elem))?;
            }
            Ok(Event::Eof) => break,
            Ok(e) => writer.write_event(e)?,
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
        }
    }
    
    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result)?)
}

pub struct SvgExporter {
    view_plane: ViewPlane,
    precision: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ViewPlane {
    XY,
    XZ,
    YZ,
    Isometric,
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
                ResolvedEntity::Circle { center, diameter, .. } => {
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

        // Sort entities by ID for deterministic output order
        let mut sorted_entities: Vec<_> = entities.iter().collect();
        sorted_entities.sort_by_key(|(id, _)| *id);
        
        for (id, entity) in sorted_entities {
            match entity {
                ResolvedEntity::Point { at } => {
                    let (x, y) = self.project_point(at);
                    svg.push_str(&format!(
                        r#"  <circle id="{}" cx="{}" cy="{}" r="2" fill="black"/>"#,
                        id,
                        fmt_svg(x, self.precision),
                        fmt_svg(y, self.precision),
                    ));
                    svg.push('\n');
                }
                ResolvedEntity::Circle { center, diameter, normal } => {
                    let (cx, cy) = self.project_point(center);
                    // Project circle as ellipse based on normal and viewing angle
                    let (rx, ry, rotation) = self.project_circle_as_ellipse(*diameter / 2.0, normal);
                    
                    if (rx - ry).abs() < 0.001 {
                        // Circle appears as circle (no significant distortion)
                        svg.push_str(&format!(
                            r#"  <circle id="{}" cx="{}" cy="{}" r="{}" fill="none" stroke="black"/>"#,
                            id, fmt_svg(cx, self.precision), fmt_svg(cy, self.precision), fmt_svg(rx, self.precision)
                        ));
                    } else if rx.abs() < 0.001 || ry.abs() < 0.001 {
                        // Circle appears as line (edge-on view)
                        // Draw as a line representing the circle's edge
                        let half_len = rx.max(ry);
                        let angle_rad = rotation.to_radians();
                        let dx = half_len * angle_rad.cos();
                        let dy = half_len * angle_rad.sin();
                        svg.push_str(&format!(
                            r#"  <line id="{}" x1="{}" y1="{}" x2="{}" y2="{}" stroke="black"/>"#,
                            id, fmt_svg(cx - dx, self.precision), fmt_svg(cy - dy, self.precision), 
                            fmt_svg(cx + dx, self.precision), fmt_svg(cy + dy, self.precision)
                        ));
                    } else {
                        // Circle appears as ellipse
                        svg.push_str(&format!(
                            r#"  <ellipse id="{}" cx="{}" cy="{}" rx="{}" ry="{}" transform="rotate({:.1} {} {})" fill="none" stroke="black"/>"#,
                            id, fmt_svg(cx, self.precision), fmt_svg(cy, self.precision), 
                            fmt_svg(rx, self.precision), fmt_svg(ry, self.precision), 
                            normalize_zero(rotation), fmt_svg(cx, self.precision), fmt_svg(cy, self.precision)
                        ));
                    }
                    svg.push('\n');
                }
                ResolvedEntity::Line { p1, p2 } => {
                    let (x1, y1) = self.project_point(p1);
                    let (x2, y2) = self.project_point(p2);
                    svg.push_str(&format!(
                        r#"  <line id="{}" x1="{}" y1="{}" x2="{}" y2="{}" stroke="black"/>"#,
                        id, fmt_svg(x1, self.precision), fmt_svg(y1, self.precision), 
                        fmt_svg(x2, self.precision), fmt_svg(y2, self.precision)
                    ));
                    svg.push('\n');
                }
            }
        }

        svg.push_str("</svg>");
        
        // Post-process to sanitize any remaining -0.0 patterns in attributes
        sanitize_svg_attributes(&svg)
    }

    fn project_point(&self, point: &[f64]) -> (f64, f64) {
        let (x, y) = match self.view_plane {
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
            ViewPlane::Isometric => {
                // Isometric projection: 45-degree view showing all three axes
                // Standard isometric projection formula:
                // x' = x - y
                // y' = (x + y) / 2 - z
                let x = point.get(0).copied().unwrap_or(0.0);
                let y = point.get(1).copied().unwrap_or(0.0);
                let z = point.get(2).copied().unwrap_or(0.0);
                (x - y, (x + y) / 2.0 - z)
            }
        };
        // Normalize to avoid -0.0 which can differ across platforms
        (normalize_zero(x), normalize_zero(y))
    }

    /// Project a 3D circle as an ellipse based on view angle and circle normal.
    /// Returns (rx, ry, rotation_degrees) for the ellipse.
    fn project_circle_as_ellipse(&self, radius: f64, normal: &[f64]) -> (f64, f64, f64) {
        let nx = normal.get(0).copied().unwrap_or(0.0);
        let ny = normal.get(1).copied().unwrap_or(0.0);
        let nz = normal.get(2).copied().unwrap_or(1.0);
        
        // Normalize the normal vector
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        let (nx, ny, nz) = if len > 0.0001 {
            (nx / len, ny / len, nz / len)
        } else {
            (0.0, 0.0, 1.0)
        };

        // Get the view direction based on view plane
        let (view_x, view_y, view_z) = match self.view_plane {
            ViewPlane::XY => (0.0, 0.0, 1.0),   // Looking along +Z
            ViewPlane::XZ => (0.0, 1.0, 0.0),   // Looking along +Y
            ViewPlane::YZ => (1.0, 0.0, 0.0),   // Looking along +X
            ViewPlane::Isometric => {
                // Isometric view direction (normalized)
                let d = 1.0 / 3.0_f64.sqrt();
                (d, d, d)
            }
        };

        // Calculate the dot product between view direction and circle normal
        // This gives us cos(angle) between view and circle plane
        let dot = nx * view_x + ny * view_y + nz * view_z;
        let cos_angle = dot.abs(); // Absolute value since we don't care about facing direction
        
        // The minor axis of the projected ellipse is radius * cos(angle)
        // When viewing perpendicular to circle plane: cos_angle = 0, ellipse is full circle
        // When viewing edge-on: cos_angle = 1, ellipse is a line
        
        // Actually, it's the opposite:
        // - cos_angle = 1 means view is parallel to normal (perpendicular to circle) -> full circle
        // - cos_angle = 0 means view is perpendicular to normal (edge-on) -> line
        
        let rx = radius; // Major axis is always the full radius
        let ry = radius * cos_angle; // Minor axis depends on viewing angle
        
        // Calculate rotation angle for the ellipse in the 2D view
        // This depends on which direction the circle is tilted
        let rotation = self.calculate_ellipse_rotation(nx, ny, nz);
        
        // Normalize to avoid -0.0 which can differ across platforms
        (normalize_zero(rx), normalize_zero(ry), normalize_zero(rotation))
    }

    /// Calculate the rotation angle (in degrees) for the projected ellipse
    fn calculate_ellipse_rotation(&self, nx: f64, ny: f64, nz: f64) -> f64 {
        match self.view_plane {
            ViewPlane::XY => {
                // In XY view, rotation depends on how the normal tilts in XY
                // If normal is [1,0,0], the circle is in YZ plane, appears as vertical line
                // If normal is [0,1,0], the circle is in XZ plane, appears as horizontal line
                ny.atan2(nx).to_degrees()
            }
            ViewPlane::XZ => {
                // In XZ view, rotation depends on X and Z components of normal
                nz.atan2(nx).to_degrees()
            }
            ViewPlane::YZ => {
                // In YZ view, rotation depends on Y and Z components of normal
                nz.atan2(ny).to_degrees()
            }
            ViewPlane::Isometric => {
                // For isometric, calculate based on projection of normal onto view plane
                // This is more complex; for now use a simplified approximation
                let projected_x = nx - ny;
                let projected_y = (nx + ny) / 2.0 - nz;
                projected_y.atan2(projected_x).to_degrees()
            }
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
                normal: vec![0.0, 0.0, 1.0],
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

    #[test]
    fn test_project_point_isometric() {
        let exporter = SvgExporter::new(ViewPlane::Isometric);
        // Test origin
        assert_eq!(exporter.project_point(&[0.0, 0.0, 0.0]), (0.0, 0.0));
        // Test point on X axis
        assert_eq!(exporter.project_point(&[100.0, 0.0, 0.0]), (100.0, 50.0));
        // Test point on Y axis
        assert_eq!(exporter.project_point(&[0.0, 100.0, 0.0]), (-100.0, 50.0));
        // Test point on Z axis
        assert_eq!(exporter.project_point(&[0.0, 0.0, 100.0]), (0.0, -100.0));
        // Test point in 3D space
        let (x, y) = exporter.project_point(&[50.0, 30.0, 20.0]);
        assert_eq!(x, 20.0); // 50 - 30
        assert!((y - 20.0).abs() < 0.001); // (50 + 30) / 2 - 20 = 40 - 20 = 20
    }
}
