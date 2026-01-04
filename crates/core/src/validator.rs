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
    fn test_validate_constraint_references_empty_entities() {
        use crate::ir::Constraint;
        let validator = Validator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![],
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
                assert!(message.contains("(none)"));
                assert_eq!(pointer, Some("/constraints/0".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_constraint_references_with_available_entities() {
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
                entity: "nonexistent".to_string(),
            }],
        };
        let result = validator.validate_constraint_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, .. } => {
                assert!(message.contains("unknown entity"));
                assert!(message.contains("Available entities"));
                assert!(message.contains("p1"));
                assert!(message.contains("p2"));
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
                    p1: "p1".to_string(), // p1 doesn't exist
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
                assert!(message.contains("p1"));
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
                    start: "start".to_string(), // start doesn't exist
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
                assert!(message.contains("start"));
                assert_eq!(pointer, Some("/entities/1/start".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_valid_line() {
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
    fn test_validate_entity_references_valid_arc() {
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
                Entity::Point {
                    id: "end".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                },
                Entity::Arc {
                    id: "a1".to_string(),
                    center: vec![ExprOrNumber::Number(0.0)],
                    start: "start".to_string(),
                    end: "end".to_string(),
                },
            ],
            constraints: vec![],
        };
        assert!(validator.validate_entity_references(&doc).is_ok());
    }

    #[test]
    fn test_validate_constraint_references_all_constraint_types() {
        use crate::ir::{Constraint, ExprOrNumber};
        let validator = Validator::new();
        let mut entities = vec![
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
            Entity::Circle {
                id: "c1".to_string(),
                center: vec![ExprOrNumber::Number(0.0)],
                diameter: ExprOrNumber::Number(10.0),
            },
        ];

        // Test various constraint types
        let constraints = vec![
            Constraint::Fixed { entity: "p1".to_string() },
            Constraint::Distance {
                between: vec!["p1".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(10.0),
            },
            Constraint::Perpendicular {
                a: "l1".to_string(),
                b: "l1".to_string(), // Same line (edge case)
            },
            Constraint::PointOnLine {
                point: "p1".to_string(),
                line: "l1".to_string(),
            },
        ];

        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities,
            constraints,
        };

        assert!(validator.validate_constraint_references(&doc).is_ok());
    }

    #[test]
    fn test_validator_default() {
        let validator = Validator::default();
        assert!(std::mem::size_of_val(&validator) > 0);
    }
}
