//! Tests for circle center point reference feature
//! 
//! These tests verify that circles can reference point entities for their center,
//! and that the circle center correctly tracks the point during solving.

use slvsx_core::ir::{Constraint, Entity, ExprOrNumber, InputDocument, PositionOrRef};
use slvsx_core::solver::{Solver, SolverConfig};

/// Helper to solve a document
fn solve(doc: &InputDocument) -> Result<slvsx_core::SolveResult, slvsx_core::error::Error> {
    let solver = Solver::new(SolverConfig::default());
    solver.solve(doc)
}

/// Helper to create a basic document with common defaults
fn create_doc(entities: Vec<Entity>, constraints: Vec<Constraint>) -> InputDocument {
    InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: std::collections::HashMap::new(),
        entities,
        constraints,
    }
}

/// Helper to make a point
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

/// Helper to make a circle with coordinate center
fn circle_coords(id: &str, x: f64, y: f64, z: f64, diameter: f64) -> Entity {
    Entity::Circle {
        id: id.to_string(),
        center: PositionOrRef::Coordinates(vec![
            ExprOrNumber::Number(x),
            ExprOrNumber::Number(y),
            ExprOrNumber::Number(z),
        ]),
        diameter: ExprOrNumber::Number(diameter),
        normal: vec![
            ExprOrNumber::Number(0.0),
            ExprOrNumber::Number(0.0),
            ExprOrNumber::Number(1.0),
        ],
        construction: false,
        preserve: false,
    }
}

/// Helper to make a circle with point reference center
fn circle_ref(id: &str, point_ref: &str, diameter: f64) -> Entity {
    Entity::Circle {
        id: id.to_string(),
        center: PositionOrRef::Reference(point_ref.to_string()),
        diameter: ExprOrNumber::Number(diameter),
        normal: vec![
            ExprOrNumber::Number(0.0),
            ExprOrNumber::Number(0.0),
            ExprOrNumber::Number(1.0),
        ],
        construction: false,
        preserve: false,
    }
}

#[test]
fn test_circle_with_point_reference_solves_successfully() {
    // A circle referencing a point should solve without errors
    let doc = create_doc(
        vec![
            point("center_pt", 10.0, 20.0, 0.0),
            circle_ref("my_circle", "center_pt", 50.0),
        ],
        vec![],
    );

    let result = solve(&doc);
    assert!(result.is_ok(), "Solve should succeed: {:?}", result.err());
    
    let output = result.unwrap();
    let entities = output.entities.expect("Entities should be present");
    let circle = entities.get("my_circle").expect("Circle should exist");
    
    // Circle center should match the point's position
    match circle {
        slvsx_core::ir::ResolvedEntity::Circle { center, diameter, .. } => {
            assert!((center[0] - 10.0).abs() < 1e-6, "Circle center X should be 10");
            assert!((center[1] - 20.0).abs() < 1e-6, "Circle center Y should be 20");
            assert!((center[2] - 0.0).abs() < 1e-6, "Circle center Z should be 0");
            assert!((diameter - 50.0).abs() < 1e-6, "Diameter should be 50");
        }
        _ => panic!("Expected Circle entity"),
    }
}

#[test]
fn test_circle_center_moves_when_referenced_point_is_constrained() {
    // KEY TEST: Circle center should track the point after solving
    // Create a point and a circle referencing it, then constrain the point to move
    let doc = create_doc(
        vec![
            // The center point starts at (0, 0, 0)
            point("center_pt", 0.0, 0.0, 0.0),
            // Another point that's fixed at (100, 0, 0)
            point("anchor", 100.0, 0.0, 0.0),
            // Circle centered on center_pt
            circle_ref("my_circle", "center_pt", 20.0),
        ],
        vec![
            // Fix the anchor point
            Constraint::Fixed {
                entity: "anchor".to_string(),
                workplane: None,
            },
            // Constrain center_pt to be 50 units from anchor
            // This should move center_pt to (50, 0, 0) or similar
            Constraint::Distance {
                between: vec!["center_pt".to_string(), "anchor".to_string()],
                value: ExprOrNumber::Number(50.0),
            },
        ],
    );

    let result = solve(&doc);
    assert!(result.is_ok(), "Solve should succeed: {:?}", result.err());
    
    let output = result.unwrap();
    let entities = output.entities.expect("Entities should be present");
    
    // Get the solved position of center_pt
    let center_pt = entities.get("center_pt").expect("center_pt should exist");
    let (pt_x, pt_y, pt_z) = match center_pt {
        slvsx_core::ir::ResolvedEntity::Point { at } => {
            (at[0], at[1], at[2])
        }
        _ => panic!("Expected Point entity"),
    };
    
    // The point should have moved (distance constraint should have affected it)
    // Since anchor is fixed at (100, 0, 0) and distance is 50, center_pt should be at (50, 0, 0)
    let dist_to_anchor = ((pt_x - 100.0).powi(2) + pt_y.powi(2) + pt_z.powi(2)).sqrt();
    assert!(
        (dist_to_anchor - 50.0).abs() < 1e-3,
        "center_pt should be 50 units from anchor, got distance: {}",
        dist_to_anchor
    );
    
    // NOW CHECK THE CIRCLE - its center should match the solved point position
    let circle = entities.get("my_circle").expect("Circle should exist");
    match circle {
        slvsx_core::ir::ResolvedEntity::Circle { center, .. } => {
            assert!(
                (center[0] - pt_x).abs() < 1e-3,
                "Circle center X ({}) should match point X ({})",
                center[0], pt_x
            );
            assert!(
                (center[1] - pt_y).abs() < 1e-3,
                "Circle center Y ({}) should match point Y ({})",
                center[1], pt_y
            );
            assert!(
                (center[2] - pt_z).abs() < 1e-3,
                "Circle center Z ({}) should match point Z ({})",
                center[2], pt_z
            );
        }
        _ => panic!("Expected Circle entity"),
    }
}

// Note: point_on_circle constraint with circles that reference points is a known
// limitation and is tracked separately. The key feature (circle center tracking
// the referenced point) works correctly as demonstrated by
// test_circle_center_moves_when_referenced_point_is_constrained.

// ============ Validation Tests ============

#[test]
fn test_circle_reference_to_undefined_point_fails_validation() {
    use slvsx_core::validator::Validator;
    
    let doc = create_doc(
        vec![
            // Circle references a point that doesn't exist
            circle_ref("my_circle", "nonexistent_point", 20.0),
        ],
        vec![],
    );

    let validator = Validator::new();
    let result = validator.validate(&doc);
    
    assert!(result.is_err(), "Validation should fail");
    let err = result.err().unwrap();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("not yet defined") || err_str.contains("nonexistent_point"),
        "Error should mention the undefined point: {}",
        err_str
    );
}

#[test]
fn test_circle_reference_to_non_point_entity_fails_validation() {
    use slvsx_core::validator::Validator;
    
    let doc = create_doc(
        vec![
            // First, create the points for the line
            point("p1", 0.0, 0.0, 0.0),
            point("p2", 10.0, 0.0, 0.0),
            // Then create the line (not a point)
            Entity::Line {
                id: "my_line".to_string(),
                p1: "p1".to_string(),
                p2: "p2".to_string(),
                construction: false,
                preserve: false,
            },
            // Circle tries to reference the line as center - should fail
            circle_ref("my_circle", "my_line", 20.0),
        ],
        vec![],
    );

    let validator = Validator::new();
    let result = validator.validate(&doc);
    
    assert!(result.is_err(), "Validation should fail");
    let err = result.err().unwrap();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("not a Point") || err_str.contains("my_line"),
        "Error should mention that the referenced entity is not a Point: {}",
        err_str
    );
}

#[test]
fn test_circle_reference_to_forward_declared_point_fails_validation() {
    use slvsx_core::validator::Validator;
    
    // Circle comes BEFORE the point it references - forward reference not allowed
    let doc = create_doc(
        vec![
            circle_ref("my_circle", "center_pt", 20.0),
            point("center_pt", 10.0, 20.0, 0.0),
        ],
        vec![],
    );

    let validator = Validator::new();
    let result = validator.validate(&doc);
    
    assert!(result.is_err(), "Validation should fail for forward reference");
    let err = result.err().unwrap();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("not yet defined"),
        "Error should mention forward reference: {}",
        err_str
    );
}

#[test]
fn test_circle_with_coordinate_center_still_works() {
    // Regression test: circles with coordinate centers should still work
    let doc = create_doc(
        vec![
            circle_coords("my_circle", 50.0, 50.0, 0.0, 30.0),
        ],
        vec![],
    );

    let result = solve(&doc);
    assert!(result.is_ok(), "Solve should succeed: {:?}", result.err());
    
    let output = result.unwrap();
    let entities = output.entities.expect("Entities should be present");
    let circle = entities.get("my_circle").expect("Circle should exist");
    
    match circle {
        slvsx_core::ir::ResolvedEntity::Circle { center, diameter, .. } => {
            assert!((center[0] - 50.0).abs() < 1e-6);
            assert!((center[1] - 50.0).abs() < 1e-6);
            assert!((diameter - 30.0).abs() < 1e-6, "Diameter should be 30");
        }
        _ => panic!("Expected Circle"),
    }
}

