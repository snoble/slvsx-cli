//! Generalized tests for entity processing in solver.rs
//!
//! These tests verify that all entity types are correctly processed,
//! including expression evaluation, ID mapping, and FFI calls.

use slvsx_core::ir::{Constraint, Entity, ExprOrNumber, InputDocument};
use slvsx_core::solver::{Solver, SolverConfig};
use std::collections::HashMap;

/// Helper to create a minimal document with given entities and constraints
fn make_doc(entities: Vec<Entity>, constraints: Vec<Constraint>) -> InputDocument {
    InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: HashMap::new(),
        entities,
        constraints,
    }
}

/// Helper to create a minimal document with parameters
fn make_doc_with_params(
    entities: Vec<Entity>,
    constraints: Vec<Constraint>,
    params: HashMap<String, f64>,
) -> InputDocument {
    InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: params,
        entities,
        constraints,
    }
}

// ============================================================================
// Point Entity Tests
// ============================================================================

#[test]
fn test_point_entity_with_numbers() {
    let doc = make_doc(
        vec![Entity::Point {
            id: "p1".to_string(),
            at: vec![
                ExprOrNumber::Number(10.0),
                ExprOrNumber::Number(20.0),
                ExprOrNumber::Number(30.0),
            ],
            construction: false,
            preserve: false,
        }],
        vec![Constraint::Fixed {
            entity: "p1".to_string(),
            workplane: None,
        }],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).unwrap();

    let entities = result.entities.unwrap();
    assert!(entities.contains_key("p1"));
}

#[test]
fn test_point_entity_with_expressions() {
    let mut params = HashMap::new();
    params.insert("x".to_string(), 100.0);
    params.insert("y".to_string(), 200.0);

    let doc = make_doc_with_params(
        vec![Entity::Point {
            id: "p1".to_string(),
            at: vec![
                ExprOrNumber::Expression("$x".to_string()),
                ExprOrNumber::Expression("$y".to_string()),
                ExprOrNumber::Number(0.0),
            ],
            construction: false,
            preserve: false,
        }],
        vec![Constraint::Fixed {
            entity: "p1".to_string(),
            workplane: None,
        }],
        params,
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).unwrap();

    let entities = result.entities.unwrap();
    assert!(entities.contains_key("p1"));
}

#[test]
fn test_point_entity_with_2d_coordinates() {
    // Point with only 2 coordinates (z defaults to 0)
    let doc = make_doc(
        vec![Entity::Point {
            id: "p1".to_string(),
            at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(20.0)],
            construction: false,
            preserve: false,
        }],
        vec![Constraint::Fixed {
            entity: "p1".to_string(),
            workplane: None,
        }],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).unwrap();
    assert!(result.entities.unwrap().contains_key("p1"));
}

#[test]
fn test_point_preserve_flag() {
    let doc = make_doc(
        vec![Entity::Point {
            id: "p1".to_string(),
            at: vec![
                ExprOrNumber::Number(10.0),
                ExprOrNumber::Number(20.0),
                ExprOrNumber::Number(0.0),
            ],
            construction: false,
            preserve: true, // Mark as preserved
        }],
        vec![Constraint::Fixed {
            entity: "p1".to_string(),
            workplane: None,
        }],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

// ============================================================================
// Line Entity Tests
// ============================================================================

#[test]
fn test_line_entity_between_points() {
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![
                    ExprOrNumber::Number(100.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Line {
                id: "line1".to_string(),
                p1: "p1".to_string(),
                p2: "p2".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
                workplane: None,
            },
        ],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).unwrap();

    let entities = result.entities.unwrap();
    assert!(entities.contains_key("line1"));
}

#[test]
fn test_line_with_missing_p1_fails() {
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "p2".to_string(),
                at: vec![
                    ExprOrNumber::Number(100.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Line {
                id: "line1".to_string(),
                p1: "nonexistent".to_string(), // Does not exist
                p2: "p2".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_err());
}

// ============================================================================
// Circle Entity Tests
// ============================================================================

#[test]
fn test_circle_entity_with_normal() {
    let doc = make_doc(
        vec![Entity::Circle {
            id: "c1".to_string(),
            center: vec![
                ExprOrNumber::Number(50.0),
                ExprOrNumber::Number(50.0),
                ExprOrNumber::Number(0.0),
            ],
            diameter: ExprOrNumber::Number(40.0),
            normal: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(1.0),
            ],
            construction: false,
            preserve: false,
        }],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).unwrap();

    let entities = result.entities.unwrap();
    assert!(entities.contains_key("c1"));
}

#[test]
fn test_circle_with_expression_diameter() {
    let mut params = HashMap::new();
    params.insert("d".to_string(), 60.0);

    let doc = make_doc_with_params(
        vec![Entity::Circle {
            id: "c1".to_string(),
            center: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
            ],
            diameter: ExprOrNumber::Expression("$d".to_string()),
            normal: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(1.0),
            ],
            construction: false,
            preserve: false,
        }],
        vec![],
        params,
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).unwrap();
    assert!(result.entities.unwrap().contains_key("c1"));
}

// ============================================================================
// Plane Entity Tests
// ============================================================================

#[test]
fn test_plane_entity_with_origin_and_normal() {
    let doc = make_doc(
        vec![Entity::Plane {
            id: "xy_plane".to_string(),
            origin: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
            ],
            normal: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(1.0),
            ],
        }],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

#[test]
fn test_plane_normal_normalization() {
    // Non-unit normal [0, 0, 5] should be normalized to [0, 0, 1]
    let doc = make_doc(
        vec![Entity::Plane {
            id: "plane".to_string(),
            origin: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
            ],
            normal: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(5.0), // Should be normalized
            ],
        }],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

#[test]
fn test_plane_with_expressions() {
    let mut params = HashMap::new();
    params.insert("z".to_string(), 100.0);

    let doc = make_doc_with_params(
        vec![Entity::Plane {
            id: "plane".to_string(),
            origin: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Expression("$z".to_string()),
            ],
            normal: vec![
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(0.0),
                ExprOrNumber::Number(1.0),
            ],
        }],
        vec![],
        params,
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

// ============================================================================
// Point2D Entity Tests
// ============================================================================

#[test]
fn test_point2d_entity_with_workplane() {
    let doc = make_doc(
        vec![
            Entity::Plane {
                id: "xy_plane".to_string(),
                origin: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                normal: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(1.0),
                ],
            },
            Entity::Point2D {
                id: "p2d".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(20.0)],
                workplane: "xy_plane".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        vec![Constraint::Fixed {
            entity: "p2d".to_string(),
            workplane: Some("xy_plane".to_string()),
        }],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

#[test]
fn test_point2d_with_missing_workplane_fails() {
    let doc = make_doc(
        vec![Entity::Point2D {
            id: "p2d".to_string(),
            at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(20.0)],
            workplane: "nonexistent".to_string(), // Does not exist
            construction: false,
            preserve: false,
        }],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_err());
}

// ============================================================================
// Line2D Entity Tests
// ============================================================================

#[test]
fn test_line2d_entity() {
    let doc = make_doc(
        vec![
            Entity::Plane {
                id: "xy_plane".to_string(),
                origin: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                normal: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(1.0),
                ],
            },
            Entity::Point2D {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                workplane: "xy_plane".to_string(),
                construction: false,
                preserve: false,
            },
            Entity::Point2D {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(100.0), ExprOrNumber::Number(0.0)],
                workplane: "xy_plane".to_string(),
                construction: false,
                preserve: false,
            },
            Entity::Line2D {
                id: "line".to_string(),
                p1: "p1".to_string(),
                p2: "p2".to_string(),
                workplane: "xy_plane".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
                workplane: Some("xy_plane".to_string()),
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
                workplane: Some("xy_plane".to_string()),
            },
        ],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

// ============================================================================
// Arc Entity Tests
// ============================================================================

#[test]
fn test_arc_entity() {
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "center".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "start".to_string(),
                at: vec![
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "end".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Arc {
                id: "arc1".to_string(),
                center: "center".to_string(),
                start: "start".to_string(),
                end: "end".to_string(),
                normal: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(1.0),
                ],
                workplane: None,
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "center".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "start".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "end".to_string(),
                workplane: None,
            },
        ],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

#[test]
fn test_arc_normal_normalization() {
    // Non-unit normal should be normalized
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "center".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "start".to_string(),
                at: vec![
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "end".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Arc {
                id: "arc1".to_string(),
                center: "center".to_string(),
                start: "start".to_string(),
                end: "end".to_string(),
                normal: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(100.0), // Should normalize to 1.0
                ],
                workplane: None,
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "center".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "start".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "end".to_string(),
                workplane: None,
            },
        ],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

// ============================================================================
// Cubic Entity Tests
// ============================================================================

#[test]
fn test_cubic_entity() {
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "p0".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p1".to_string(),
                at: vec![
                    ExprOrNumber::Number(30.0),
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![
                    ExprOrNumber::Number(70.0),
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p3".to_string(),
                at: vec![
                    ExprOrNumber::Number(100.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Cubic {
                id: "curve".to_string(),
                control_points: vec![
                    "p0".to_string(),
                    "p1".to_string(),
                    "p2".to_string(),
                    "p3".to_string(),
                ],
                workplane: None,
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "p0".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "p1".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "p3".to_string(),
                workplane: None,
            },
        ],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_ok());
}

#[test]
fn test_cubic_with_wrong_control_point_count_fails() {
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "p0".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p1".to_string(),
                at: vec![
                    ExprOrNumber::Number(100.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Cubic {
                id: "curve".to_string(),
                control_points: vec!["p0".to_string(), "p1".to_string()], // Only 2, need 4
                workplane: None,
                construction: false,
                preserve: false,
            },
        ],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_err());
}

// ============================================================================
// Expression Evaluation Tests
// ============================================================================

#[test]
fn test_arithmetic_expressions() {
    let mut params = HashMap::new();
    params.insert("w".to_string(), 100.0);
    params.insert("h".to_string(), 50.0);

    let doc = make_doc_with_params(
        vec![Entity::Point {
            id: "p1".to_string(),
            at: vec![
                ExprOrNumber::Expression("$w / 2".to_string()),
                ExprOrNumber::Expression("$h * 2".to_string()),
                ExprOrNumber::Expression("$w + $h".to_string()),
            ],
            construction: false,
            preserve: false,
        }],
        vec![Constraint::Fixed {
            entity: "p1".to_string(),
            workplane: None,
        }],
        params,
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).unwrap();
    assert!(result.entities.unwrap().contains_key("p1"));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_line_with_missing_p2_fails() {
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Line {
                id: "line1".to_string(),
                p1: "p1".to_string(),
                p2: "nonexistent".to_string(), // Does not exist
                construction: false,
                preserve: false,
            },
        ],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_err());
}

#[test]
fn test_arc_with_missing_center_fails() {
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "start".to_string(),
                at: vec![
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "end".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Arc {
                id: "arc1".to_string(),
                center: "nonexistent".to_string(), // Does not exist
                start: "start".to_string(),
                end: "end".to_string(),
                normal: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(1.0),
                ],
                workplane: None,
                construction: false,
                preserve: false,
            },
        ],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_err());
}

#[test]
fn test_cubic_with_missing_control_point_fails() {
    let doc = make_doc(
        vec![
            Entity::Point {
                id: "p0".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p1".to_string(),
                at: vec![
                    ExprOrNumber::Number(30.0),
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![
                    ExprOrNumber::Number(70.0),
                    ExprOrNumber::Number(50.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            // p3 is missing
            Entity::Cubic {
                id: "curve".to_string(),
                control_points: vec![
                    "p0".to_string(),
                    "p1".to_string(),
                    "p2".to_string(),
                    "nonexistent".to_string(), // Does not exist
                ],
                workplane: None,
                construction: false,
                preserve: false,
            },
        ],
        vec![],
    );

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);
    assert!(result.is_err());
}

