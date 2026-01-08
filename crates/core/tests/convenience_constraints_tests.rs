//! Tests for convenience constraints (Collinear, EqualAngles)

use slvsx_core::ir::{Constraint, Entity, ExprOrNumber, InputDocument};
use slvsx_core::solver::{Solver, SolverConfig};

/// Helper to solve a document
fn solve(doc: &InputDocument) -> Result<slvsx_core::SolveResult, slvsx_core::error::Error> {
    let solver = Solver::new(SolverConfig::default());
    solver.solve(doc)
}

fn point(id: &str, x: f64, y: f64, z: f64) -> Entity {
    Entity::Point {
        id: id.to_string(),
        at: vec![
            ExprOrNumber::Number(x),
            ExprOrNumber::Number(y),
            ExprOrNumber::Number(z),
        ],
        construction: false,
        preserve: false,
    }
}

fn line(id: &str, p1: &str, p2: &str) -> Entity {
    Entity::Line {
        id: id.to_string(),
        p1: p1.to_string(),
        p2: p2.to_string(),
        construction: false,
        preserve: false,
    }
}

fn create_doc(entities: Vec<Entity>, constraints: Vec<Constraint>) -> InputDocument {
    InputDocument {
        schema: "slvs-json/1".to_string(),
        entities,
        constraints,
        parameters: std::collections::HashMap::new(),
        units: "mm".to_string(),
    }
}

// ============ Collinear Constraint Tests ============

#[test]
fn test_collinear_three_points() {
    // Three points: p1 and p2 on a line, p3 should be moved to be collinear
    let doc = create_doc(
        vec![
            point("p1", 0.0, 0.0, 0.0),
            point("p2", 10.0, 0.0, 0.0),
            point("p3", 5.0, 5.0, 0.0), // Off the line initially
        ],
        vec![
            // Fix the first two points
            Constraint::Fixed {
                entity: "p1".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
                workplane: None,
            },
            // Collinear constraint should move p3 onto the line
            Constraint::Collinear {
                points: vec!["p1".to_string(), "p2".to_string(), "p3".to_string()],
            },
        ],
    );

    let result = solve(&doc);
    assert!(result.is_ok(), "Solve should succeed: {:?}", result.err());

    let output = result.unwrap();
    let entities = output.entities.expect("Entities should be present");

    // Get p3's solved position
    let p3 = entities.get("p3").expect("p3 should exist");
    let (x, y, z) = match p3 {
        slvsx_core::ir::ResolvedEntity::Point { at } => (at[0], at[1], at[2]),
        _ => panic!("Expected Point"),
    };

    // p3 should be on the line y=0 (since p1 and p2 are on y=0)
    assert!(
        y.abs() < 1e-3,
        "p3 should be on the line (y should be 0), but y = {}",
        y
    );
    assert!(
        z.abs() < 1e-3,
        "p3 should be on the line (z should be 0), but z = {}",
        z
    );
}

#[test]
fn test_collinear_four_points() {
    // Four points should all be collinear
    let doc = create_doc(
        vec![
            point("p1", 0.0, 0.0, 0.0),
            point("p2", 10.0, 10.0, 0.0),
            point("p3", 3.0, 5.0, 0.0), // Off the line
            point("p4", 7.0, 3.0, 0.0), // Off the line
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
            Constraint::Collinear {
                points: vec![
                    "p1".to_string(),
                    "p2".to_string(),
                    "p3".to_string(),
                    "p4".to_string(),
                ],
            },
        ],
    );

    let result = solve(&doc);
    assert!(result.is_ok(), "Solve should succeed: {:?}", result.err());

    let output = result.unwrap();
    let entities = output.entities.expect("Entities should be present");

    // All points should be on the line y = x
    for point_id in ["p3", "p4"] {
        let pt = entities.get(point_id).expect(&format!("{} should exist", point_id));
        let (x, y, _z) = match pt {
            slvsx_core::ir::ResolvedEntity::Point { at } => (at[0], at[1], at[2]),
            _ => panic!("Expected Point"),
        };
        assert!(
            (x - y).abs() < 1e-3,
            "{} should be on line y=x, but x={}, y={}",
            point_id, x, y
        );
    }
}

#[test]
fn test_collinear_requires_three_points() {
    // Collinear with only 2 points should fail
    let doc = create_doc(
        vec![
            point("p1", 0.0, 0.0, 0.0),
            point("p2", 10.0, 0.0, 0.0),
        ],
        vec![Constraint::Collinear {
            points: vec!["p1".to_string(), "p2".to_string()],
        }],
    );

    let result = solve(&doc);
    assert!(result.is_err(), "Should fail with only 2 points");
    let err = result.err().unwrap();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("at least 3 points"),
        "Error should mention minimum 3 points: {}",
        err_str
    );
}

// ============ EqualAngles Constraint Tests ============

#[test]
fn test_equal_angles_three_spokes() {
    // Three lines from a common center - test that equal_angles constraint is applied
    // Use equal angles without a specific value to make angles equal to each other
    let doc = create_doc(
        vec![
            point("center", 0.0, 0.0, 0.0),
            point("tip1", 50.0, 0.0, 0.0),
            point("tip2", 30.0, 40.0, 0.0),
            point("tip3", -30.0, 40.0, 0.0),
            line("spoke1", "center", "tip1"),
            line("spoke2", "center", "tip2"),
            line("spoke3", "center", "tip3"),
        ],
        vec![
            Constraint::Fixed {
                entity: "center".to_string(),
                workplane: None,
            },
            // Fix tip1 position as reference
            Constraint::Fixed {
                entity: "tip1".to_string(),
                workplane: None,
            },
            // Fix distances to prevent tips from collapsing to center
            Constraint::Distance {
                between: vec!["center".to_string(), "tip2".to_string()],
                value: ExprOrNumber::Number(50.0),
            },
            Constraint::Distance {
                between: vec!["center".to_string(), "tip3".to_string()],
                value: ExprOrNumber::Number(50.0),
            },
            // Equal angles without specified value - makes angle(spoke1,spoke2) == angle(spoke2,spoke3)
            Constraint::EqualAngles {
                lines: vec!["spoke1".to_string(), "spoke2".to_string(), "spoke3".to_string()],
                value: None,
            },
        ],
    );

    let result = solve(&doc);
    assert!(result.is_ok(), "Solve should succeed: {:?}", result.err());
}

#[test]
fn test_equal_angles_with_specified_value() {
    // Two lines with a specific angle between them
    let doc = create_doc(
        vec![
            point("center", 0.0, 0.0, 0.0),
            point("tip1", 50.0, 0.0, 0.0),
            point("tip2", 40.0, 30.0, 0.0), // Will be moved to 45° from tip1
            line("spoke1", "center", "tip1"),
            line("spoke2", "center", "tip2"),
        ],
        vec![
            Constraint::Fixed {
                entity: "center".to_string(),
                workplane: None,
            },
            Constraint::Fixed {
                entity: "tip1".to_string(),
                workplane: None,
            },
            Constraint::Distance {
                between: vec!["center".to_string(), "tip2".to_string()],
                value: ExprOrNumber::Number(50.0),
            },
            Constraint::EqualAngles {
                lines: vec!["spoke1".to_string(), "spoke2".to_string()],
                value: Some(ExprOrNumber::Number(45.0)),
            },
        ],
    );

    let result = solve(&doc);
    assert!(result.is_ok(), "Solve should succeed: {:?}", result.err());

    let output = result.unwrap();
    let entities = output.entities.expect("Entities should be present");

    // tip2 should be at 45° from the x-axis
    let tip2 = entities.get("tip2").expect("tip2 should exist");
    let (x, y, _z) = match tip2 {
        slvsx_core::ir::ResolvedEntity::Point { at } => (at[0], at[1], at[2]),
        _ => panic!("Expected Point"),
    };

    // At 45° with radius 50: x ≈ 35.35, y ≈ 35.35
    let angle = y.atan2(x).to_degrees();
    assert!(
        (angle - 45.0).abs() < 1.0,
        "Angle should be ~45°, but is {}°",
        angle
    );
}

#[test]
fn test_equal_angles_requires_two_lines() {
    // EqualAngles with only 1 line should fail
    let doc = create_doc(
        vec![
            point("center", 0.0, 0.0, 0.0),
            point("tip1", 50.0, 0.0, 0.0),
            line("spoke1", "center", "tip1"),
        ],
        vec![Constraint::EqualAngles {
            lines: vec!["spoke1".to_string()],
            value: None,
        }],
    );

    let result = solve(&doc);
    assert!(result.is_err(), "Should fail with only 1 line");
    let err = result.err().unwrap();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("at least 2 lines"),
        "Error should mention minimum 2 lines: {}",
        err_str
    );
}
