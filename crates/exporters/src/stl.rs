use slvsx_core::ir::ResolvedEntity;
use std::collections::HashMap;
use std::f64::consts::PI;

pub struct StlExporter {
    extrusion_height: f64,
}

impl Default for StlExporter {
    fn default() -> Self {
        Self {
            extrusion_height: 100.0,
        }
    }
}

impl StlExporter {
    pub fn new(extrusion_height: f64) -> Self {
        Self { extrusion_height }
    }

    pub fn export(&self, entities: &HashMap<String, ResolvedEntity>) -> anyhow::Result<Vec<u8>> {
        let mut stl = Vec::new();

        // STL ASCII header
        stl.extend_from_slice(b"solid model\n");

        // Generate cylinder for each circle entity
        for (_id, entity) in entities {
            match entity {
                ResolvedEntity::Circle { center, diameter, .. } => {
                    self.add_cylinder_to_stl(&mut stl, center, *diameter)?;
                }
                _ => {} // Skip other entities
            }
        }

        // STL footer
        stl.extend_from_slice(b"endsolid model\n");

        Ok(stl)
    }

    fn add_cylinder_to_stl(
        &self,
        stl: &mut Vec<u8>,
        center: &[f64],
        diameter: f64,
    ) -> anyhow::Result<()> {
        let cx = center.get(0).copied().unwrap_or(0.0);
        let cy = center.get(1).copied().unwrap_or(0.0);
        let base_z = center.get(2).copied().unwrap_or(0.0);
        let radius = diameter / 2.0;

        // Number of segments for circle approximation
        let segments = 32;
        let angle_step = 2.0 * PI / segments as f64;

        // Generate top and bottom faces
        for z in [base_z, base_z + self.extrusion_height] {
            let normal = if z == base_z { "0 0 -1" } else { "0 0 1" };

            // Triangle fan from center
            for i in 0..segments {
                let angle1 = i as f64 * angle_step;
                let angle2 = (i + 1) as f64 * angle_step;

                let triangle = format!(
                    "  facet normal {}\n    outer loop\n      vertex {:.6} {:.6} {:.6}\n      vertex {:.6} {:.6} {:.6}\n      vertex {:.6} {:.6} {:.6}\n    endloop\n  endfacet\n",
                    normal,
                    cx, cy, z,
                    cx + radius * angle1.cos(), cy + radius * angle1.sin(), z,
                    cx + radius * angle2.cos(), cy + radius * angle2.sin(), z
                );
                stl.extend_from_slice(triangle.as_bytes());
            }
        }

        // Generate side walls
        for i in 0..segments {
            let angle = i as f64 * angle_step;
            let next_angle = (i + 1) as f64 * angle_step;

            self.add_quad_faces(
                stl,
                cx + radius * angle.cos(),
                cy + radius * angle.sin(),
                cx + radius * next_angle.cos(),
                cy + radius * next_angle.sin(),
                base_z,
                base_z + self.extrusion_height,
            );
        }

        Ok(())
    }

    fn add_quad_faces(
        &self,
        stl: &mut Vec<u8>,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        z1: f64,
        z2: f64,
    ) {
        // Calculate normal (pointing outward)
        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();
        let nx = dy / len;
        let ny = -dx / len;

        // First triangle
        let tri1 = format!(
            "  facet normal {:.6} {:.6} 0\n    outer loop\n      vertex {:.6} {:.6} {:.6}\n      vertex {:.6} {:.6} {:.6}\n      vertex {:.6} {:.6} {:.6}\n    endloop\n  endfacet\n",
            nx, ny,
            x1, y1, z1,
            x2, y2, z1,
            x2, y2, z2
        );
        stl.extend_from_slice(tri1.as_bytes());

        // Second triangle
        let tri2 = format!(
            "  facet normal {:.6} {:.6} 0\n    outer loop\n      vertex {:.6} {:.6} {:.6}\n      vertex {:.6} {:.6} {:.6}\n      vertex {:.6} {:.6} {:.6}\n    endloop\n  endfacet\n",
            nx, ny,
            x1, y1, z1,
            x2, y2, z2,
            x1, y1, z2
        );
        stl.extend_from_slice(tri2.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stl_exporter_default() {
        let exporter = StlExporter::default();
        assert_eq!(exporter.extrusion_height, 100.0);
    }

    #[test]
    fn test_stl_exporter_new() {
        let exporter = StlExporter::new(50.0);
        assert_eq!(exporter.extrusion_height, 50.0);
    }

    #[test]
    fn test_export_empty() {
        let exporter = StlExporter::default();
        let entities = HashMap::new();
        let stl = exporter.export(&entities).unwrap();
        let stl_str = String::from_utf8(stl).unwrap();
        assert!(stl_str.starts_with("solid model"));
        assert!(stl_str.ends_with("endsolid model\n"));
    }

    #[test]
    fn test_export_circle_as_cylinder() {
        let exporter = StlExporter::default();
        let mut entities = HashMap::new();
        entities.insert(
            "circle1".to_string(),
            ResolvedEntity::Circle {
                center: vec![0.0, 0.0, 0.0],
                diameter: 48.0,
                normal: vec![0.0, 0.0, 1.0],
            },
        );

        let stl = exporter.export(&entities).unwrap();
        let stl_str = String::from_utf8(stl).unwrap();

        // Check STL structure
        assert!(stl_str.contains("solid model"));
        assert!(stl_str.contains("facet normal"));
        assert!(stl_str.contains("vertex"));
        assert!(stl_str.contains("endsolid model"));
    }

    #[test]
    fn test_export_multiple_circles() {
        let exporter = StlExporter::new(100.0);
        let mut entities = HashMap::new();

        // First circle
        entities.insert(
            "circle1".to_string(),
            ResolvedEntity::Circle {
                center: vec![0.0, 0.0, 0.0],
                diameter: 48.0,
                normal: vec![0.0, 0.0, 1.0],
            },
        );

        // Second circle
        entities.insert(
            "circle2".to_string(),
            ResolvedEntity::Circle {
                center: vec![36.0, 0.0, 0.0],
                diameter: 24.0,
                normal: vec![0.0, 0.0, 1.0],
            },
        );

        let stl = exporter.export(&entities).unwrap();
        let stl_str = String::from_utf8(stl).unwrap();

        // Should have facets for both cylinders
        let facet_count = stl_str.matches("facet normal").count();
        assert!(facet_count > 100); // Should have many facets for 2 cylinders
    }

    #[test]
    fn test_add_quad_faces() {
        let exporter = StlExporter::default();
        let mut stl = Vec::new();

        exporter.add_quad_faces(&mut stl, 0.0, 0.0, 1.0, 0.0, 0.0, 10.0);

        let stl_str = String::from_utf8(stl).unwrap();
        assert!(stl_str.contains("facet normal"));
        assert!(stl_str.contains("vertex"));
        // Should create 2 triangles
        assert_eq!(stl_str.matches("facet normal").count(), 2);
    }

    #[test]
    fn test_add_cylinder_to_stl() {
        let exporter = StlExporter::new(50.0);
        let mut stl = Vec::new();

        let center = vec![10.0, 20.0, 5.0];
        let diameter = 30.0;

        exporter
            .add_cylinder_to_stl(&mut stl, &center, diameter)
            .unwrap();

        let stl_str = String::from_utf8(stl).unwrap();

        // Should have facets for top, bottom, and sides
        let facet_count = stl_str.matches("facet normal").count();
        assert!(facet_count >= 64); // At least 32 segments * 2 (top/bottom) + sides

        // Check that vertices reference the correct center coordinates
        assert!(stl_str.contains("10.000000"));
        assert!(stl_str.contains("20.000000"));
        assert!(stl_str.contains("5.000000"));
        assert!(stl_str.contains("55.000000")); // z + height = 5 + 50
    }

    #[test]
    fn test_cylinder_with_default_z() {
        let exporter = StlExporter::default();
        let mut stl = Vec::new();

        // Test with only x,y coordinates (z should default to 0)
        let center = vec![15.0, 25.0];
        let diameter = 40.0;

        exporter
            .add_cylinder_to_stl(&mut stl, &center, diameter)
            .unwrap();

        let stl_str = String::from_utf8(stl).unwrap();

        // Should contain z=0 and z=100 (default height)
        assert!(stl_str.contains(" 0.000000"));
        assert!(stl_str.contains(" 100.000000"));
    }
}
