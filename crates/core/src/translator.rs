use crate::error::{Error, Result};
use crate::ir::{Constraint, InputDocument};

/// Translates IR to FFI calls
pub struct Translator;

impl Translator {
    pub fn new() -> Self {
        Self
    }

    pub fn translate(&self, doc: &InputDocument) -> Result<()> {
        // Validate references first
        self.validate_references(doc)?;

        // Translation logic will go here
        Ok(())
    }

    fn validate_references(&self, doc: &InputDocument) -> Result<()> {
        let entity_ids: std::collections::HashSet<_> =
            doc.entities.iter().map(|e| e.id()).collect();

        // Check constraints reference valid entities
        for constraint in &doc.constraints {
            self.validate_constraint_refs(constraint, &entity_ids)?;
        }

        Ok(())
    }

    fn validate_constraint_refs(
        &self,
        constraint: &Constraint,
        entity_ids: &std::collections::HashSet<&str>,
    ) -> Result<()> {
        let refs = self.get_constraint_refs(constraint);
        for ref_id in refs {
            if !entity_ids.contains(ref_id.as_str()) {
                return Err(Error::InvalidInput {
                    message: format!("Unknown entity reference '{}'", ref_id),
                    pointer: None,
                });
            }
        }
        Ok(())
    }

    fn get_constraint_refs(&self, constraint: &Constraint) -> Vec<String> {
        match constraint {
            Constraint::Coincident { at, of } => {
                let mut refs = vec![at.clone()];
                refs.extend(of.clone());
                refs
            }
            Constraint::Distance { between, .. } | Constraint::Angle { between, .. } => {
                between.clone()
            }
            Constraint::Perpendicular { a, b }
            | Constraint::Parallel { a, b }
            | Constraint::EqualLength { a, b }
            | Constraint::EqualRadius { a, b }
            | Constraint::Tangent { a, b } => vec![a.clone(), b.clone()],
            Constraint::Horizontal { a }
            | Constraint::Vertical { a }
            | Constraint::Fixed { entity: a } => vec![a.clone()],
            Constraint::PointOnLine { point, line }
            | Constraint::PointOnCircle {
                point,
                circle: line,
            } => vec![point.clone(), line.clone()],
        }
    }
}

impl Default for Translator {
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
    fn test_translator_new() {
        let _translator = Translator::new();
    }

    #[test]
    fn test_validate_references_valid() {
        let translator = Translator::new();
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
                    p2: "p2".to_string(),
                },
            ],
            constraints: vec![Constraint::Horizontal {
                a: "l1".to_string(),
            }],
        };

        assert!(translator.validate_references(&doc).is_ok());
    }

    #[test]
    fn test_validate_references_invalid() {
        let translator = Translator::new();
        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![Entity::Point {
                id: "p1".to_string(),
                at: vec![ExprOrNumber::Number(0.0)],
            }],
            constraints: vec![
                Constraint::Horizontal {
                    a: "l1".to_string(),
                }, // l1 doesn't exist
            ],
        };

        let result = translator.validate_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, .. } => {
                assert!(message.contains("Unknown entity reference"))
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_get_constraint_refs() {
        let translator = Translator::new();

        let constraint = Constraint::Horizontal {
            a: "l1".to_string(),
        };
        assert_eq!(translator.get_constraint_refs(&constraint), vec!["l1"]);

        let constraint = Constraint::Perpendicular {
            a: "l1".to_string(),
            b: "l2".to_string(),
        };
        assert_eq!(
            translator.get_constraint_refs(&constraint),
            vec!["l1", "l2"]
        );

        let constraint = Constraint::Coincident {
            at: "p1".to_string(),
            of: vec!["l1".to_string(), "l2".to_string()],
        };
        assert_eq!(
            translator.get_constraint_refs(&constraint),
            vec!["p1", "l1", "l2"]
        );
    }
}
