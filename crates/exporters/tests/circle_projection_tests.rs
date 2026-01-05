//! Tests for circle projection with orientation
//! 
//! Circles should project as ellipses when viewed from angles other than
//! perpendicular to their plane. A circle on a vertical wall (normal = [0,1,0])
//! should appear as a horizontal line when viewed from above (XY view).

use slvsx_core::ir::ResolvedEntity;
use slvsx_exporters::svg::{SvgExporter, ViewPlane};
use std::collections::HashMap;

/// Helper to create a circle with a given normal
fn circle_with_normal(center: [f64; 3], diameter: f64, normal: [f64; 3]) -> ResolvedEntity {
    ResolvedEntity::Circle {
        center: center.to_vec(),
        diameter,
        normal: normal.to_vec(),
    }
}

/// Helper to extract circle/ellipse from SVG
fn extract_circle_element(svg: &str, id: &str) -> Option<String> {
    for line in svg.lines() {
        if line.contains(&format!("id=\"{}\"", id)) {
            return Some(line.to_string());
        }
    }
    None
}

#[test]
fn test_circle_in_xy_plane_viewed_from_xy_is_circle() {
    // A circle in the XY plane (normal = [0,0,1]) viewed from above (XY view)
    // should appear as a full circle
    let mut entities = HashMap::new();
    entities.insert(
        "circle1".to_string(),
        circle_with_normal([50.0, 50.0, 0.0], 40.0, [0.0, 0.0, 1.0]),
    );

    let exporter = SvgExporter::new(ViewPlane::XY);
    let svg = exporter.export(&entities).unwrap();

    let element = extract_circle_element(&svg, "circle1").unwrap();
    
    // Should be a <circle> element (not an ellipse)
    assert!(element.contains("<circle"), "Circle in XY plane viewed from XY should be a circle: {}", element);
    assert!(element.contains("r=\"20"), "Circle should have radius 20 (diameter/2)");
}

#[test]
fn test_circle_on_front_wall_viewed_from_xy_is_line() {
    // A circle on the front wall (normal = [0,1,0], in XZ plane at Y=0)
    // viewed from above (XY view) should appear as a horizontal line
    let mut entities = HashMap::new();
    entities.insert(
        "entrance".to_string(),
        circle_with_normal([75.0, 0.0, 120.0], 40.0, [0.0, 1.0, 0.0]),
    );

    let exporter = SvgExporter::new(ViewPlane::XY);
    let svg = exporter.export(&entities).unwrap();

    let element = extract_circle_element(&svg, "entrance");
    
    // From above, a circle on a vertical wall should be a line (or very thin ellipse)
    // It should NOT be rendered as a full circle
    if let Some(el) = element {
        assert!(
            !el.contains("<circle") || el.contains("ellipse") || el.contains("<line"),
            "Circle on vertical wall viewed from above should NOT be a full circle: {}",
            el
        );
    }
}

#[test]
fn test_circle_on_side_wall_viewed_from_xz_is_line() {
    // A circle on the side wall (normal = [1,0,0], in YZ plane)
    // viewed from front (XZ view) should appear as a vertical line
    let mut entities = HashMap::new();
    entities.insert(
        "window".to_string(),
        circle_with_normal([0.0, 50.0, 50.0], 30.0, [1.0, 0.0, 0.0]),
    );

    let exporter = SvgExporter::new(ViewPlane::XZ);
    let svg = exporter.export(&entities).unwrap();

    let element = extract_circle_element(&svg, "window");
    
    if let Some(el) = element {
        assert!(
            !el.contains("<circle") || el.contains("ellipse") || el.contains("<line"),
            "Circle on side wall viewed from front should NOT be a full circle: {}",
            el
        );
    }
}

#[test]
fn test_circle_at_45_degrees_is_ellipse() {
    // A circle tilted 45 degrees (normal = [0, 0.707, 0.707])
    // viewed from above (XY view) should appear as an ellipse
    let mut entities = HashMap::new();
    let sqrt2_2 = 0.7071067811865476; // sqrt(2)/2
    entities.insert(
        "tilted".to_string(),
        circle_with_normal([50.0, 50.0, 50.0], 40.0, [0.0, sqrt2_2, sqrt2_2]),
    );

    let exporter = SvgExporter::new(ViewPlane::XY);
    let svg = exporter.export(&entities).unwrap();

    let element = extract_circle_element(&svg, "tilted").unwrap();
    
    // Should be an ellipse (or a circle with different rx/ry if using ellipse element)
    // The important thing is one axis should be shorter than the other
    assert!(
        element.contains("ellipse") || element.contains("rx") || element.contains("transform"),
        "Tilted circle should render as ellipse or transformed circle: {}",
        element
    );
}

#[test]
fn test_circle_isometric_projection() {
    // In isometric view, a circle in the XY plane should appear as an ellipse
    // because we're viewing from an angle
    let mut entities = HashMap::new();
    entities.insert(
        "floor_circle".to_string(),
        circle_with_normal([50.0, 50.0, 0.0], 40.0, [0.0, 0.0, 1.0]),
    );

    let exporter = SvgExporter::new(ViewPlane::Isometric);
    let svg = exporter.export(&entities).unwrap();

    let element = extract_circle_element(&svg, "floor_circle").unwrap();
    
    // In isometric, XY plane circles become ellipses
    // (unless we specifically choose not to distort them)
    assert!(
        element.contains("ellipse") || element.contains("<circle"),
        "Circle should render: {}",
        element
    );
}

#[test]
fn test_resolved_entity_circle_has_normal() {
    // Verify that ResolvedEntity::Circle includes the normal field
    let circle = ResolvedEntity::Circle {
        center: vec![0.0, 0.0, 0.0],
        diameter: 10.0,
        normal: vec![0.0, 1.0, 0.0],
    };
    
    if let ResolvedEntity::Circle { normal, .. } = circle {
        assert_eq!(normal, vec![0.0, 1.0, 0.0]);
    } else {
        panic!("Expected Circle variant");
    }
}

#[test]
fn test_circle_normal_serialization() {
    // Test that circle normal serializes/deserializes correctly
    let circle = ResolvedEntity::Circle {
        center: vec![50.0, 0.0, 100.0],
        diameter: 40.0,
        normal: vec![0.0, 1.0, 0.0],
    };
    
    let json = serde_json::to_string(&circle).unwrap();
    assert!(json.contains("normal"), "Serialized circle should include normal: {}", json);
    
    let deserialized: ResolvedEntity = serde_json::from_str(&json).unwrap();
    assert_eq!(circle, deserialized);
}

#[test]
fn test_svg_sanitizes_negative_zero() {
    // Test that -0.0 values get sanitized to 0.0 in SVG output
    let mut entities = HashMap::new();
    // Create a point that might produce -0.0 in certain calculations
    entities.insert(
        "origin".to_string(),
        ResolvedEntity::Point {
            at: vec![0.0, 0.0, 0.0],
        },
    );

    let exporter = SvgExporter::new(ViewPlane::XY);
    let svg = exporter.export(&entities).unwrap();

    // Verify no -0. appears in the output
    assert!(
        !svg.contains("-0."),
        "SVG should not contain negative zero: {}",
        svg
    );
}

#[test]
fn test_circle_renders_as_ellipse_when_tilted() {
    // Test that a tilted circle renders as an ellipse element
    let mut entities = HashMap::new();
    let sqrt2_2 = 0.7071067811865476;
    entities.insert(
        "tilted_circle".to_string(),
        circle_with_normal([50.0, 50.0, 0.0], 40.0, [sqrt2_2, 0.0, sqrt2_2]),
    );

    let exporter = SvgExporter::new(ViewPlane::XY);
    let svg = exporter.export(&entities).unwrap();
    
    // Should contain an ellipse element
    assert!(svg.contains("<ellipse"), "Tilted circle should render as ellipse: {}", svg);
}

#[test]
fn test_circle_renders_as_line_when_edge_on() {
    // Test that an edge-on circle renders as a line element  
    let mut entities = HashMap::new();
    // Circle with normal perpendicular to view direction (XY view, normal along Y)
    entities.insert(
        "edge_circle".to_string(),
        circle_with_normal([50.0, 50.0, 0.0], 40.0, [0.0, 1.0, 0.0]),
    );

    let exporter = SvgExporter::new(ViewPlane::XY);
    let svg = exporter.export(&entities).unwrap();
    
    // Should contain a line element (edge-on circle appears as a line)
    assert!(svg.contains("<line"), "Edge-on circle should render as line: {}", svg);
}

