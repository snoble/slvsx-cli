// Integration tests for new entity types: Point2D, Arc, Cubic, Construction geometry

use slvsx_core::ir::{Entity, ExprOrNumber, InputDocument};
use slvsx_core::solver::Solver;

#[test]
fn test_point_2d_entity() {
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            // Create a workplane first
            Entity::Plane {
                id: "front_face".to_string(),
                origin: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            },
            // Create 2D point in workplane
            Entity::Point2D {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(20.0)],
                workplane: "front_face".to_string(),
                construction: false,
            },
        ],
        constraints: vec![],
    };

    let mut solver = Solver::new();
    let result = solver.solve(&doc);
    // Should solve successfully (just a fixed point in a workplane)
    assert!(result.is_ok(), "2D point should solve successfully");
}

#[test]
fn test_proper_arc_entity() {
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            // Create center, start, and end points
            Entity::Point {
                id: "center".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            Entity::Point {
                id: "start".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            Entity::Point {
                id: "end".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            // Create arc
            Entity::Arc {
                id: "arc1".to_string(),
                center: "center".to_string(),
                start: "start".to_string(),
                end: "end".to_string(),
                normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                workplane: None,
                construction: false,
            },
        ],
        constraints: vec![
            slvsx_core::ir::Constraint::Fixed { entity: "center".to_string(), workplane: None },
            slvsx_core::ir::Constraint::Fixed { entity: "start".to_string(), workplane: None },
            slvsx_core::ir::Constraint::Fixed { entity: "end".to_string(), workplane: None },
        ],
    };

    let mut solver = Solver::new();
    let result = solver.solve(&doc);
    // Should solve successfully (all points fixed, arc is determined)
    assert!(result.is_ok(), "Proper arc entity should solve successfully");
}

#[test]
fn test_cubic_bezier_entity() {
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            // Create 4 control points
            Entity::Point {
                id: "p0".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(20.0), ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            Entity::Point {
                id: "p3".to_string(),
                at: vec![ExprOrNumber::Number(30.0), ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            // Create cubic Bezier curve
            Entity::Cubic {
                id: "curve1".to_string(),
                control_points: vec!["p0".to_string(), "p1".to_string(), "p2".to_string(), "p3".to_string()],
                workplane: None,
                construction: false,
            },
        ],
        constraints: vec![
            slvsx_core::ir::Constraint::Fixed { entity: "p0".to_string(), workplane: None },
            slvsx_core::ir::Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            slvsx_core::ir::Constraint::Fixed { entity: "p2".to_string(), workplane: None },
            slvsx_core::ir::Constraint::Fixed { entity: "p3".to_string(), workplane: None },
        ],
    };

    let mut solver = Solver::new();
    let result = solver.solve(&doc);
    // Should solve successfully (all control points fixed, curve is determined)
    assert!(result.is_ok(), "Cubic Bezier curve should solve successfully");
}

#[test]
fn test_construction_geometry() {
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            // Construction line (helper geometry)
            Entity::Line {
                id: "helper".to_string(),
                p1: "p1".to_string(),
                p2: "p2".to_string(),
                construction: true,
            },
        ],
        constraints: vec![
            slvsx_core::ir::Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            slvsx_core::ir::Constraint::Fixed { entity: "p2".to_string(), workplane: None },
        ],
    };

    let mut solver = Solver::new();
    let result = solver.solve(&doc);
    // Should solve successfully (construction flag doesn't affect solving)
    assert!(result.is_ok(), "Construction geometry should solve successfully");
}

#[test]
fn test_arc_with_workplane() {
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            // Create workplane
            Entity::Plane {
                id: "xy_plane".to_string(),
                origin: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            },
            // Create points
            Entity::Point {
                id: "center".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            Entity::Point {
                id: "start".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            Entity::Point {
                id: "end".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0)],
                construction: false,
            },
            // Create 2D arc in workplane
            Entity::Arc {
                id: "arc2d".to_string(),
                center: "center".to_string(),
                start: "start".to_string(),
                end: "end".to_string(),
                normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                workplane: Some("xy_plane".to_string()),
                construction: false,
            },
        ],
        constraints: vec![
            slvsx_core::ir::Constraint::Fixed { entity: "center".to_string(), workplane: None },
            slvsx_core::ir::Constraint::Fixed { entity: "start".to_string(), workplane: None },
            slvsx_core::ir::Constraint::Fixed { entity: "end".to_string(), workplane: None },
        ],
    };

    let mut solver = Solver::new();
    let result = solver.solve(&doc);
    // Should solve successfully
    assert!(result.is_ok(), "Arc with workplane should solve successfully");
}

