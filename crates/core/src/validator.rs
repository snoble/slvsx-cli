use crate::error::{Error, Result};
use crate::ir::InputDocument;
use std::collections::HashSet;

pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, doc: &InputDocument) -> Result<()> {
        self.validate_schema(doc)?;
        self.validate_entity_ids(doc)?;
        self.validate_units(doc)?;
        self.validate_constraint_references(doc)?;
        self.validate_entity_references(doc)?;
        Ok(())
    }

    fn validate_schema(&self, doc: &InputDocument) -> Result<()> {
        if doc.schema != "slvs-json/1" {
            return Err(Error::SchemaValidation(format!(
                "Unsupported schema version: {}",
                doc.schema
            )));
        }
        Ok(())
    }

    fn validate_entity_ids(&self, doc: &InputDocument) -> Result<()> {
        let mut seen = HashSet::new();
        for entity in &doc.entities {
            let id = entity.id();
            if !seen.insert(id) {
                return Err(Error::InvalidInput {
                    message: format!("Duplicate entity ID: {}", id),
                    pointer: None,
                });
            }
        }
        Ok(())
    }

    fn validate_units(&self, doc: &InputDocument) -> Result<()> {
        const VALID_UNITS: &[&str] = &["mm", "cm", "m", "in", "ft"];
        if !VALID_UNITS.contains(&doc.units.as_str()) {
            return Err(Error::InvalidInput {
                message: format!(
                    "Invalid units: {}. Must be one of: {:?}",
                    doc.units, VALID_UNITS
                ),
                pointer: Some("/units".to_string()),
            });
        }
        Ok(())
    }

    fn validate_constraint_references(&self, doc: &InputDocument) -> Result<()> {
        use crate::ir::{Constraint, CoincidentData};
        let entity_ids: HashSet<&str> = doc.entities.iter().map(|e| e.id()).collect();

        for (idx, constraint) in doc.constraints.iter().enumerate() {
            let refs = match constraint {
                Constraint::Fixed { entity } => vec![entity.as_str()],
                Constraint::Distance { between, .. } => between.iter().map(|s| s.as_str()).collect(),
                Constraint::Angle { between, .. } => between.iter().map(|s| s.as_str()).collect(),
                Constraint::Perpendicular { a, b } | Constraint::EqualRadius { a, b } | Constraint::Tangent { a, b } => {
                    vec![a.as_str(), b.as_str()]
                }
                Constraint::Parallel { entities } | Constraint::EqualLength { entities } => {
                    entities.iter().map(|s| s.as_str()).collect()
                }
                Constraint::Horizontal { a } | Constraint::Vertical { a } => vec![a.as_str()],
                Constraint::PointOnLine { point, line } => vec![point.as_str(), line.as_str()],
                Constraint::PointOnCircle { point, circle } => vec![point.as_str(), circle.as_str()],
                Constraint::Symmetric { a, b, about } => vec![a.as_str(), b.as_str(), about.as_str()],
                Constraint::Midpoint { point, of } => {
                    let mut refs = vec![point.as_str()];
                    refs.push(of.as_str());
                    refs
                }
                Constraint::Coincident { data } => {
                    match data {
                        CoincidentData::PointOnLine { at, of } => {
                            let mut refs = vec![at.as_str()];
                            refs.extend(of.iter().map(|s| s.as_str()));
                            refs
                        }
                        CoincidentData::TwoEntities { entities } => {
                            entities.iter().map(|s| s.as_str()).collect()
                        }
                    }
                }
            };

            for ref_id in refs {
                if !entity_ids.contains(ref_id) {
                    return Err(Error::InvalidInput {
                        message: format!(
                            "Constraint #{} references unknown entity '{}'. Available entities: {}",
                            idx + 1,
                            ref_id,
                            if entity_ids.is_empty() {
                                "(none)".to_string()
                            } else {
                                let mut sorted: Vec<&str> = entity_ids.iter().copied().collect();
                                sorted.sort();
                                sorted.join(", ")
                            }
                        ),
                        pointer: Some(format!("/constraints/{}", idx)),
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_entity_references(&self, doc: &InputDocument) -> Result<()> {
        use crate::ir::Entity;
        let entity_ids: HashSet<&str> = doc.entities.iter().map(|e| e.id()).collect();

        for (idx, entity) in doc.entities.iter().enumerate() {
            match entity {
                Entity::Line { p1, p2, .. } => {
                    if !entity_ids.contains(p1.as_str()) {
                        return Err(Error::InvalidInput {
                            message: format!(
                                "Line entity #{} references unknown point '{}'",
                                idx + 1, p1
                            ),
                            pointer: Some(format!("/entities/{}/p1", idx)),
                        });
                    }
                    if !entity_ids.contains(p2.as_str()) {
                        return Err(Error::InvalidInput {
                            message: format!(
                                "Line entity #{} references unknown point '{}'",
                                idx + 1, p2
                            ),
                            pointer: Some(format!("/entities/{}/p2", idx)),
                        });
                    }
                }
                Entity::Arc { start, end, .. } => {
                    if !entity_ids.contains(start.as_str()) {
                        return Err(Error::InvalidInput {
                            message: format!(
                                "Arc entity #{} references unknown point '{}'",
                                idx + 1, start
                            ),
                            pointer: Some(format!("/entities/{}/start", idx)),
                        });
                    }
                    if !entity_ids.contains(end.as_str()) {
                        return Err(Error::InvalidInput {
                            message: format!(
                                "Arc entity #{} references unknown point '{}'",
                                idx + 1, end
                            ),
                            pointer: Some(format!("/entities/{}/end", idx)),
                        });
                    }
                }
                _ => {} // Points, circles, planes don't reference other entities
            }
        }

        Ok(())
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Entity, ExprOrNumber};
    use std::collections::HashMap;

    #[test]
    fn test_validator_new() {
        let _validator = Validator::new();
    }

    #[test]
    fn test_validate_schema_valid() {
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![],
            constraints: vec![],
        };
        assert!(validator.validate_schema(&doc).is_ok());
    }

    #[test]
    fn test_validate_schema_invalid() {
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/2".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![],
            constraints: vec![],
        };
        let result = validator.validate_schema(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::SchemaValidation(msg) => assert!(msg.contains("Unsupported schema version")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_ids_unique() {
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
            ],
            constraints: vec![],
        };
        assert!(validator.validate_entity_ids(&doc).is_ok());
    }

    #[test]
    fn test_validate_entity_ids_duplicate() {
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                },
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_ids(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, .. } => assert!(message.contains("Duplicate entity ID")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_units_valid() {
        let validator = Validator::new();
        for unit in &["mm", "cm", "m", "in", "ft"] {
            let doc = InputDocument {
                schema: "slvs-json/1".to_string(),
                units: unit.to_string(),
                parameters: HashMap::new(),
                entities: vec![],
                constraints: vec![],
            };
            assert!(validator.validate_units(&doc).is_ok());
        }
    }

    #[test]
    fn test_validate_units_invalid() {
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "yards".to_string(),
            parameters: HashMap::new(),
            entities: vec![],
            constraints: vec![],
        };
        let result = validator.validate_units(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("Invalid units"));
                assert_eq!(pointer, Some("/units".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_complete() {
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0)],
            }],
            constraints: vec![],
        };
        assert!(validator.validate(&doc).is_ok());
    }

    #[test]
    fn test_validate_constraint_references_valid() {
        use crate::ir::Constraint;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
            ],
            constraints: vec![Constraint::Fixed {
                entity: "p1".to_string(),
            }],
        };
        assert!(validator.validate_constraint_references(&doc).is_ok());
    }

    #[test]
    fn test_validate_constraint_references_invalid() {
        use crate::ir::Constraint;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0)],
            }],
            constraints: vec![Constraint::Fixed {
                entity: "nonexistent".to_string(),
            }],
        };
        let result = validator.validate_constraint_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown entity"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/constraints/0".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_valid() {
        use crate::ir::Entity;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "p2".to_string(),
                },
            ],
            constraints: vec![],
        };
        assert!(validator.validate_entity_references(&doc).is_ok());
    }

    #[test]
    fn test_validate_entity_references_invalid() {
        use crate::ir::Entity;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "nonexistent".to_string(), // p2 doesn't exist
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown point"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/entities/1/p2".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_line_missing_p1() {
        use crate::ir::Entity;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "nonexistent".to_string(), // p1 doesn't exist
                    p2: "p2".to_string(),
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown point"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/entities/1/p1".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_arc_missing_start() {
        use crate::ir::Entity;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "end".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
                Entity::Arc {
                    id: "a1".to_string(),
                    center: vec![ExprOrNumber::Number(0.0)],
                    start: "nonexistent".to_string(), // start doesn't exist
                    end: "end".to_string(),
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown point"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/entities/1/start".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_arc_missing_end() {
        use crate::ir::Entity;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "start".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                },
                Entity::Arc {
                    id: "a1".to_string(),
                    center: vec![ExprOrNumber::Number(0.0)],
                    start: "start".to_string(),
                    end: "nonexistent".to_string(), // end doesn't exist
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown point"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/entities/1/end".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_constraint_references_empty_entities() {
        use crate::ir::Constraint;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![], // Empty entities
            constraints: vec![Constraint::Fixed {
                entity: "nonexistent".to_string(),
            }],
        };
        let result = validator.validate_constraint_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown entity"));
                assert!(message.contains("nonexistent"));
                assert!(message.contains("(none)")); // Should show "(none)" when empty
                assert_eq!(pointer, Some("/constraints/0".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_constraint_references_all_types() {
        use crate::ir::{Constraint, CoincidentData, ExprOrNumber};
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
                Entity::Point {
                    id: "p3".to_string(),
                    at: vec![ExprOrNumber::Number(2.0)],
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "p2".to_string(),
                },
                Entity::Line {
                    id: "l2".to_string(),
                    p1: "p2".to_string(),
                    p2: "p3".to_string(),
                },
                Entity::Circle {
                    id: "c1".to_string(),
                    center: vec![ExprOrNumber::Number(0.0)],
                    diameter: ExprOrNumber::Number(10.0),
                },
            ],
            constraints: vec![
                Constraint::Distance {
                    between: vec!["p1".to_string(), "p2".to_string()],
                    value: ExprOrNumber::Number(10.0),
                },
                Constraint::Angle {
                    between: vec!["l1".to_string(), "l2".to_string()],
                    value: ExprOrNumber::Number(90.0),
                },
                Constraint::Perpendicular {
                    a: "l1".to_string(),
                    b: "l2".to_string(),
                },
                Constraint::EqualRadius {
                    a: "c1".to_string(),
                    b: "c1".to_string(), // Same circle (edge case)
                },
                Constraint::Tangent {
                    a: "l1".to_string(),
                    b: "c1".to_string(),
                },
                Constraint::Parallel {
                    entities: vec!["l1".to_string(), "l2".to_string()],
                },
                Constraint::EqualLength {
                    entities: vec!["l1".to_string(), "l2".to_string()],
                },
                Constraint::Horizontal {
                    a: "l1".to_string(),
                },
                Constraint::Vertical {
                    a: "l2".to_string(),
                },
                Constraint::PointOnLine {
                    point: "p3".to_string(),
                    line: "l1".to_string(),
                },
                Constraint::PointOnCircle {
                    point: "p1".to_string(),
                    circle: "c1".to_string(),
                },
                Constraint::Symmetric {
                    a: "p1".to_string(),
                    b: "p2".to_string(),
                    about: "l1".to_string(),
                },
                Constraint::Midpoint {
                    point: "p2".to_string(),
                    of: "l1".to_string(),
                },
                Constraint::Coincident {
                    data: CoincidentData::PointOnLine {
                        at: "p1".to_string(),
                        of: vec!["l1".to_string()],
                    },
                },
                Constraint::Coincident {
                    data: CoincidentData::TwoEntities {
                        entities: vec!["p1".to_string(), "p2".to_string()],
                    },
                },
            ],
        };
        assert!(validator.validate_constraint_references(&doc).is_ok());
    }

    #[test]
    fn test_validate_constraint_references_distance_invalid() {
        use crate::ir::{Constraint, ExprOrNumber};
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0)],
            }],
            constraints: vec![Constraint::Distance {
                between: vec!["p1".to_string(), "nonexistent".to_string()],
                value: ExprOrNumber::Number(10.0),
            }],
        };
        let result = validator.validate_constraint_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown entity"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/constraints/0".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_constraint_references_coincident_point_on_line_invalid() {
        use crate::ir::{Constraint, CoincidentData};
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0)],
            }],
            constraints: vec![Constraint::Coincident {
                data: CoincidentData::PointOnLine {
                    at: "p1".to_string(),
                    of: vec!["nonexistent".to_string()],
                },
            }],
        };
        let result = validator.validate_constraint_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown entity"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/constraints/0".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_constraint_references_coincident_two_entities_invalid() {
        use crate::ir::{Constraint, CoincidentData};
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0)],
            }],
            constraints: vec![Constraint::Coincident {
                data: CoincidentData::TwoEntities {
                    entities: vec!["p1".to_string(), "nonexistent".to_string()],
                },
            }],
        };
        let result = validator.validate_constraint_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown entity"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/constraints/0".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_constraint_references_sorted_entities() {
        use crate::ir::Constraint;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![
                Entity::Point {
                    id: "zebra".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                },
                Entity::Point {
                    id: "apple".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
                Entity::Point {
                    id: "banana".to_string(),
                    at: vec![ExprOrNumber::Number(2.0)],
                },
            ],
            constraints: vec![Constraint::Fixed {
                entity: "nonexistent".to_string(),
            }],
        };
        let result = validator.validate_constraint_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("unknown entity"));
                assert!(message.contains("nonexistent"));
                // Should show sorted entities: apple, banana, zebra
                assert!(message.contains("apple"));
                assert!(message.contains("banana"));
                assert!(message.contains("zebra"));
                // Verify they're in sorted order (apple comes before banana)
                let apple_pos = message.find("apple").unwrap();
                let banana_pos = message.find("banana").unwrap();
                assert!(apple_pos < banana_pos);
                assert_eq!(pointer, Some("/constraints/0".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }
}
