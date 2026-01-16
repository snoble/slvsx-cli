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
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
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
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
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
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Fixed { entity: "p2".to_string(), workplane: None },
            Constraint::PointOnLine {
                point: "p3".to_string(),
                line: "line1".to_string(),
                workplane: None,
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
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Fixed { entity: "p2".to_string(), workplane: None },
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
            Constraint::Fixed { entity: "origin".to_string(), workplane: None },
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
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Fixed { entity: "p2".to_string(), workplane: None },
            Constraint::Fixed { entity: "p3".to_string(), workplane: None },
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
fn test_symmetric_horizontal_constraint_with_workplane() {
    // SymmetricHorizontal constraint - makes two points symmetric about a horizontal axis
    // From SolveSpace: av - bv = 0 (equal Y), au + bu = 0 (opposite X)
    // Use 2D points within a workplane
    let doc = create_test_doc(
        vec![
            Entity::Plane {
                id: "wp1".to_string(),
                origin: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            },
            Entity::Point2D {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(-10.0), ExprOrNumber::Number(5.0)],
                workplane: "wp1".to_string(),
                construction: false,
                preserve: false,
            },
            Entity::Point2D {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(15.0), ExprOrNumber::Number(8.0)],
                workplane: "wp1".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        vec![
            // Fix p1 to anchor the solution
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            // SymmetricHorizontal: p1 and p2 should have equal Y and opposite X
            Constraint::SymmetricHorizontal {
                a: "p1".to_string(),
                b: "p2".to_string(),
                workplane: "wp1".to_string(),
            },
        ],
    );

    // This should succeed with the SymmetricHorizontal constraint
    let result = solve_and_get(doc);
    assert!(result.is_ok(), "Solver should succeed with SymmetricHorizontal constraint");
    
    let entities = result.unwrap();
    let p1 = match entities.get("p1") {
        Some(ResolvedEntity::Point { at }) => at.clone(),
        _ => panic!("p1 not found"),
    };
    let p2 = match entities.get("p2") {
        Some(ResolvedEntity::Point { at }) => at.clone(),
        _ => panic!("p2 not found"),
    };
    
    // SymmetricHorizontal: Y coordinates equal, X coordinates opposite
    // With p1 fixed at (-10, 5), p2 should move to (10, 5)
    let y_diff = (p1[1] - p2[1]).abs();
    let x_sum = (p1[0] + p2[0]).abs();  // Opposite X means sum should be 0
    
    assert!(y_diff < 1.0, 
        "SymmetricHorizontal should make Y coordinates equal: p1.y={}, p2.y={}, diff={}", 
        p1[1], p2[1], y_diff);
    assert!(x_sum < 1.0, 
        "SymmetricHorizontal should make X coordinates opposite: p1.x={}, p2.x={}, sum={}", 
        p1[0], p2[0], x_sum);
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
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Fixed { entity: "p2".to_string(), workplane: None },
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