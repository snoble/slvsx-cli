// Comprehensive constraint testing
// This file tests all implemented constraints to ensure they work correctly

use slvsx_core::ir::{Constraint, Entity, ExprOrNumber, InputDocument, ResolvedEntity};
use slvsx_core::solver::{Solver, SolverConfig};
use std::collections::HashMap;

/// Helper to create a test document
fn create_test_doc(entities: Vec<Entity>, constraints: Vec<Constraint>) -> InputDocument {
    InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: HashMap::new(),
        entities,
        constraints,
    }
}

/// Helper to solve and get result
fn solve_and_get(doc: InputDocument) -> Result<HashMap<String, ResolvedEntity>, String> {
    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc).map_err(|e| format!("{:?}", e))?;
    result.entities.ok_or_else(|| "No entities in result".to_string())
}

#[test]
fn test_fixed_constraint() {
    let doc = create_test_doc(
        vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(20.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
            },
        ],
    );

    let entities = solve_and_get(doc).expect("Solve should succeed");
    
    if let Some(ResolvedEntity::Point { at }) = entities.get("p1") {
        assert!((at[0] - 10.0).abs() < 0.001, "X should remain 10");
        assert!((at[1] - 20.0).abs() < 0.001, "Y should remain 20");
        assert!(at[2].abs() < 0.001, "Z should remain 0");
    } else {
        panic!("p1 not found or wrong type");
    }
}

#[test]
fn test_distance_constraint() {
    let doc = create_test_doc(
        vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
            },
            Constraint::Distance {
                between: vec!["p1".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(50.0),
            },
        ],
    );

    let entities = solve_and_get(doc).expect("Solve should succeed");
    
    if let (Some(ResolvedEntity::Point { at: at1 }), Some(ResolvedEntity::Point { at: at2 })) = 
        (entities.get("p1"), entities.get("p2")) {
        let dx = at2[0] - at1[0];
        let dy = at2[1] - at1[1];
        let dz = at2[2] - at1[2];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        assert!((distance - 50.0).abs() < 0.1, "Distance should be 50, got {}", distance);
    } else {
        panic!("Points not found or wrong type");
    }
}

#[test]
fn test_point_on_line_constraint() {
    let doc = create_test_doc(
        vec![
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
        vec![
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
    );

    let entities = solve_and_get(doc).expect("Solve should succeed");
    
    if let Some(ResolvedEntity::Point { at }) = entities.get("p3") {
        // Point should be on the line from (0,0,0) to (100,0,0)
        // So Y should be 0
        assert!(at[1].abs() < 0.001, "Y should be 0 for point on horizontal line, got {}", at[1]);
        // X should be between 0 and 100
        assert!(at[0] >= -0.001 && at[0] <= 100.001, "X should be between 0 and 100, got {}", at[0]);
    } else {
        panic!("p3 not found or wrong type");
    }
}

#[test]
fn test_coincident_constraint() {
    let doc = create_test_doc(
        vec![
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
        vec![
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
    );

    let entities = solve_and_get(doc).expect("Solve should succeed");
    
    if let Some(ResolvedEntity::Point { at }) = entities.get("p3") {
        // Point should be on the line from (0,0,0) to (100,0,0)
        assert!(at[1].abs() < 0.001, "Y should be 0 for coincident with horizontal line, got {}", at[1]);
    } else {
        panic!("p3 not found or wrong type");
    }
}

#[test]
fn test_perpendicular_constraint() {
    let doc = create_test_doc(
        vec![
            Entity::Point {
                id: "origin".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(100.0), ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(100.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Line {
                id: "line1".to_string(),
                p1: "origin".to_string(),
                p2: "p1".to_string(),
                construction: false,
                preserve: false,
            },
            Entity::Line {
                id: "line2".to_string(),
                p1: "origin".to_string(),
                p2: "p2".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "origin".to_string(),
            },
            Constraint::Distance {
                between: vec!["origin".to_string(), "p1".to_string()],
                value: ExprOrNumber::Number(100.0),
            },
            Constraint::Distance {
                between: vec!["origin".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(100.0),
            },
            Constraint::Perpendicular {
                a: "line1".to_string(),
                b: "line2".to_string(),
            },
        ],
    );

    let entities = solve_and_get(doc).expect("Solve should succeed");
    
    if let (Some(ResolvedEntity::Point { at: at1 }), Some(ResolvedEntity::Point { at: at2 })) = 
        (entities.get("p1"), entities.get("p2")) {
        // Dot product of the two vectors should be close to 0 for perpendicular lines
        let dot_product = at1[0] * at2[0] + at1[1] * at2[1] + at1[2] * at2[2];
        assert!(dot_product.abs() < 0.1, "Dot product should be near 0 for perpendicular lines, got {}", dot_product);
    } else {
        panic!("Points not found or wrong type");
    }
}

#[test]
fn test_parallel_constraint() {
    let doc = create_test_doc(
        vec![
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
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(50.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p4".to_string(),
                at: vec![ExprOrNumber::Number(100.0), ExprOrNumber::Number(60.0), ExprOrNumber::Number(0.0)],
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
            Entity::Line {
                id: "line2".to_string(),
                p1: "p3".to_string(),
                p2: "p4".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
            },
            Constraint::Fixed {
                entity: "p3".to_string(),
            },
            Constraint::Parallel {
                entities: vec!["line1".to_string(), "line2".to_string()],
            },
        ],
    );

    let entities = solve_and_get(doc).expect("Solve should succeed");
    
    if let Some(ResolvedEntity::Point { at }) = entities.get("p4") {
        // For parallel lines, p4.y should equal p3.y (both lines horizontal)
        assert!((at[1] - 50.0).abs() < 0.1, "Y of p4 should be 50 for parallel lines, got {}", at[1]);
    } else {
        panic!("p4 not found or wrong type");
    }
}

#[test]
fn test_unimplemented_constraint_ignored() {
    // Unimplemented constraints are currently just ignored with a warning
    // They don't cause the solver to fail
    let doc = create_test_doc(
        vec![
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
            Entity::Line {
                id: "line1".to_string(),
                p1: "p1".to_string(),
                p2: "p2".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        vec![
            Constraint::Horizontal {
                a: "line1".to_string(),
            },
        ],
    );

    // This should succeed but ignore the Horizontal constraint
    let result = solve_and_get(doc);
    assert!(result.is_ok(), "Solver should succeed even with unimplemented constraints");
}

/// Test that over-constrained systems fail appropriately
#[test]
fn test_over_constrained_system_fails() {
    let doc = create_test_doc(
        vec![
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
        ],
        vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
            },
            Constraint::Distance {
                between: vec!["p1".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(50.0), // Inconsistent with fixed positions!
            },
        ],
    );

    // This should fail due to inconsistent constraints
    let result = solve_and_get(doc);
    assert!(result.is_err(), "Over-constrained system should fail");
}