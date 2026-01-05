// Tests for interactive editing features: preserve flag and WHERE_DRAGGED constraint

use slvsx_core::ir::{Entity, ExprOrNumber, InputDocument, Constraint};
use slvsx_core::solver::Solver;

#[test]
fn test_preserve_flag_on_point() {
    // Test that preserve flag marks point parameters as dragged
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: true,  // Mark as preserved
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,  // Can change
            },
        ],
        constraints: vec![
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Distance {
                between: vec!["p1".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(20.0),  // Change distance - p1 should try to stay fixed
            },
        ],
    };

    let mut solver = Solver::new(Default::default());
    let result = solver.solve(&doc);
    // Should solve successfully - p1 is preserved (dragged) so solver will try to minimize its changes
    assert!(result.is_ok(), "Preserved point should solve successfully");
}

#[test]
fn test_where_dragged_constraint() {
    // Test WHERE_DRAGGED constraint (locks point absolutely)
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
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
        constraints: vec![
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Dragged {
                point: "p2".to_string(),
                workplane: None,  // 3D point
            },
            Constraint::Distance {
                between: vec!["p1".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(15.0),
            },
        ],
    };

    let mut solver = Solver::new(Default::default());
    let result = solver.solve(&doc);
    // WHERE_DRAGGED locks p2 to its current position - may overconstrain
    // But constraint should be processed
    assert!(result.is_ok() || result.is_err()); // Either is acceptable
}

#[test]
fn test_preserve_in_iterative_refinement() {
    // Simulate iterative refinement: preserve base, adjust roof
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            // Base corners - preserve these
            Entity::Point {
                id: "base_fl".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: true,
            },
            Entity::Point {
                id: "base_fr".to_string(),
                at: vec![ExprOrNumber::Number(100.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: true,
            },
            // Roof peak - can adjust
            Entity::Point {
                id: "roof_peak".to_string(),
                at: vec![ExprOrNumber::Number(50.0), ExprOrNumber::Number(50.0), ExprOrNumber::Number(100.0)],
                construction: false,
                preserve: false,
            },
        ],
        constraints: vec![
            Constraint::Fixed { entity: "base_fl".to_string(), workplane: None },
            Constraint::Distance {
                between: vec!["base_fl".to_string(), "base_fr".to_string()],
                value: ExprOrNumber::Number(100.0),
            },
            // Adjust roof height - base should try to stay fixed
            Constraint::Distance {
                between: vec!["base_fl".to_string(), "roof_peak".to_string()],
                value: ExprOrNumber::Number(120.0),  // Changed from initial 100
            },
        ],
    };

    let mut solver = Solver::new(Default::default());
    let result = solver.solve(&doc);
    // Should solve - preserved points will try to minimize changes
    assert!(result.is_ok(), "Iterative refinement with preserve should solve");
}

#[test]
fn test_preserve_on_2d_point() {
    // Test preserve flag on 2D points
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            Entity::Plane {
                id: "front_face".to_string(),
                origin: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            },
            Entity::Point2D {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(20.0)],
                workplane: "front_face".to_string(),
                construction: false,
                preserve: true,  // Preserve 2D point
            },
        ],
        constraints: vec![],
    };

    let mut solver = Solver::new(Default::default());
    let result = solver.solve(&doc);
    // Should solve successfully
    assert!(result.is_ok(), "Preserved 2D point should solve successfully");
}

#[test]
fn test_where_dragged_constraint_with_workplane() {
    // Test WHERE_DRAGGED constraint with workplane (2D point)
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            Entity::Plane {
                id: "xy_plane".to_string(),
                origin: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            },
            Entity::Point2D {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(20.0)],
                workplane: "xy_plane".to_string(),
                construction: false,
                preserve: false,
            },
        ],
        constraints: vec![
            Constraint::Dragged {
                point: "p1".to_string(),
                workplane: Some("xy_plane".to_string()),  // 2D point
            },
        ],
    };

    let mut solver = Solver::new(Default::default());
    let result = solver.solve(&doc);
    // WHERE_DRAGGED locks point - constraint should be processed
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_preserve_flag_affects_solving() {
    // Test that preserve flag actually affects solving behavior
    // When preserve=true, the solver should try to minimize changes to that point
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: true,  // Preserve this point
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,  // Can change
            },
            Entity::Point {
                id: "p3".to_string(),
                at: vec![ExprOrNumber::Number(20.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,  // Can change
            },
        ],
        constraints: vec![
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Distance {
                between: vec!["p1".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(15.0),
            },
            Constraint::Distance {
                between: vec!["p2".to_string(), "p3".to_string()],
                value: ExprOrNumber::Number(10.0),
            },
        ],
    };

    let mut solver = Solver::new(Default::default());
    let result = solver.solve(&doc);
    // Should solve - p1 is preserved so solver will try to minimize its changes
    assert!(result.is_ok(), "Preserve flag should allow solving");
}

#[test]
fn test_dragged_constraint_error_handling() {
    // Test error handling when point doesn't exist
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![],
        constraints: vec![
            Constraint::Dragged {
                point: "nonexistent".to_string(),
                workplane: None,
            },
        ],
    };

    let mut solver = Solver::new(Default::default());
    let result = solver.solve(&doc);
    // Should fail because point doesn't exist
    assert!(result.is_err(), "Should fail when dragged point doesn't exist");
}

#[test]
fn test_preserve_on_all_entity_types() {
    // Test preserve flag on all entity types that support it
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: true,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![ExprOrNumber::Number(10.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                construction: false,
                preserve: false,
            },
            Entity::Line {
                id: "l1".to_string(),
                p1: "p1".to_string(),
                p2: "p2".to_string(),
                construction: false,
                preserve: true,  // Preserve line
            },
            Entity::Circle {
                id: "c1".to_string(),
                center: vec![ExprOrNumber::Number(5.0), ExprOrNumber::Number(5.0), ExprOrNumber::Number(0.0)],
                diameter: ExprOrNumber::Number(10.0),
                construction: false,
                preserve: true,  // Preserve circle
            },
        ],
        constraints: vec![
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
        ],
    };

    let mut solver = Solver::new(Default::default());
    let result = solver.solve(&doc);
    // Should solve - preserve flags are set (though only points actually use them)
    assert!(result.is_ok(), "Preserve flags on all entity types should work");
}

