// Integration tests for slvsx-core

use slvsx_core::ir::{Constraint, Entity, ExprOrNumber, InputDocument, ResolvedEntity};
use slvsx_core::solver::{Solver, SolverConfig};
use std::collections::HashMap;

#[test]
fn test_point_on_line_constraint() {
    // Create a document with a point that should be constrained to a line
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: HashMap::new(),
        entities: vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(100.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p3".to_string(),
                at: vec![ExprOrNumber::Number(50.0), ExprOrNumber::Number(50.0), ExprOrNumber::Number(0.0)],
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
        constraints: vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
            },
            Constraint::PointOnLine {
                point: "p3".to_string(),
                line: "line1".to_string(),
            },
        ],
    };

    let mut solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).expect("Solve should succeed");

    // Check that p3 is now on the line (y should be 0)
    if let Some(resolved) = result.entities {
        if let Some(p3) = resolved.get("p3") {
            match p3 {
                ResolvedEntity::Point { at } => {
                    assert!(at[1].abs() < 0.001, "Point p3 should be on the line (y ≈ 0), got y = {}", at[1]);
                    assert!((at[0] - 50.0).abs() < 1.0, "Point p3 x coordinate should be near 50, got {}", at[0]);
                }
                _ => panic!("p3 should be a point"),
            }
        } else {
            panic!("p3 not found in resolved entities");
        }
    } else {
        panic!("No resolved entities returned");
    }
}

#[test]
fn test_coincident_constraint_point_on_line() {
    // Test the coincident constraint when used to put a point on a line
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: HashMap::new(),
        entities: vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(100.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p3".to_string(),
                at: vec![ExprOrNumber::Number(50.0), ExprOrNumber::Number(50.0), ExprOrNumber::Number(0.0)],
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
        constraints: vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
            },
            Constraint::Coincident {
                data: slvsx_core::ir::CoincidentData::PointOnLine {
                    at: "p3".to_string(),
                    of: vec!["line1".to_string()],
                }
            },
        ],
    };

    let mut solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).expect("Solve should succeed");

    // Check that p3 is now on the line (y should be 0)
    if let Some(resolved) = result.entities {
        if let Some(p3) = resolved.get("p3") {
            match p3 {
                ResolvedEntity::Point { at } => {
                    assert!(at[1].abs() < 0.001, "Point p3 should be on the line (y ≈ 0), got y = {}", at[1]);
                    assert!((at[0] - 50.0).abs() < 1.0, "Point p3 x coordinate should be near 50, got {}", at[0]);
                }
                _ => panic!("p3 should be a point"),
            }
        } else {
            panic!("p3 not found in resolved entities");
        }
    } else {
        panic!("No resolved entities returned");
    }
}