use slvsx_core::{solve, ir::*};
use serde_json::json;

#[test]
fn test_simple_point_distance_constraint() {
    let input = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![
                    ExprOrNumber::Number(10.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
            },
        ],
        constraints: vec![
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Distance {
                between: vec!["p1".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(15.0),
            },
        ],
    };

    let result = solve(input).expect("Should solve simple distance constraint");
    
    // Should return solved entity positions, not empty
    assert!(!result.entities.is_none(), "Should return entity positions");
    
    let entities = result.entities.unwrap();
    assert!(entities.contains_key("p1"), "Should contain p1");
    assert!(entities.contains_key("p2"), "Should contain p2");
    
    // This test protects against gear-only solving
    assert_eq!(result.status, "ok", "Should solve successfully");
}

#[test] 
fn test_circle_constraint() {
    let input = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: Default::default(),
        entities: vec![
            Entity::Circle {
                id: "c1".to_string(),
                center: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                diameter: ExprOrNumber::Number(20.0),
            },
        ],
        constraints: vec![
            Constraint::Fixed { entity: "c1".to_string(), workplane: None },
        ],
    };

    let result = solve(input).expect("Should solve circle constraint");
    
    // Should return the circle entity, not empty
    assert!(!result.entities.is_none(), "Should return entities");
    assert_eq!(result.status, "ok", "Should solve successfully");
}