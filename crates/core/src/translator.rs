use crate::error::{Error, Result};
use crate::ir::{Constraint, CoincidentData, InputDocument};

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
            Constraint::Coincident { data } => {
                match data {
                    CoincidentData::PointOnLine { at, of } => {
                        let mut refs = vec![at.clone()];
                        refs.extend(of.clone());
                        refs
                    },
                    CoincidentData::TwoEntities { entities } => entities.clone(),
                }
            }
            Constraint::Distance { between, .. } | Constraint::Angle { between, .. } => {
                between.clone()
            }
            Constraint::Perpendicular { a, b }
            | Constraint::EqualRadius { a, b }
            | Constraint::Tangent { a, b }
            | Constraint::SameOrientation { a, b }
            | Constraint::CubicLineTangent { cubic: a, line: b } => vec![a.clone(), b.clone()],
            Constraint::Parallel { entities } | Constraint::EqualLength { entities } => entities.clone(),
            Constraint::EqualAngle { lines } => lines.clone(),
            Constraint::Horizontal { a, workplane } | Constraint::Vertical { a, workplane } => {
                vec![a.clone(), workplane.clone()]
            }
            Constraint::Fixed { entity, workplane } => {
                let mut refs = vec![entity.clone()];
                if let Some(wp) = workplane {
                    refs.push(wp.clone());
                }
                refs
            }
            Constraint::Diameter { circle: a, .. } => vec![a.clone()],
            Constraint::PointOnLine { point, line }
            | Constraint::PointLineDistance { point, line, .. }
            | Constraint::EqualLengthPointLineDistance { point, line, .. }
            | Constraint::PointOnCircle {
                point,
                circle: line,
            } => vec![point.clone(), line.clone()],
            Constraint::Symmetric { a, b, about } => vec![a.clone(), b.clone(), about.clone()],
            Constraint::SymmetricHorizontal { a, b, workplane } | Constraint::SymmetricVertical { a, b, workplane } => {
                vec![a.clone(), b.clone(), workplane.clone()]
            }
            Constraint::Midpoint { point, of } => vec![point.clone(), of.clone()],
            Constraint::PointInPlane { point, plane } | Constraint::PointPlaneDistance { point, plane, .. } => {
                vec![point.clone(), plane.clone()]
            }
            Constraint::LengthRatio { a, b, .. } | Constraint::LengthDifference { a, b, .. } => {
                vec![a.clone(), b.clone()]
            }
            Constraint::ProjectedPointDistance { a, b, plane, .. } => {
                vec![a.clone(), b.clone(), plane.clone()]
            }
            Constraint::PointOnFace { point, face } | Constraint::PointFaceDistance { point, face, .. } => {
                vec![point.clone(), face.clone()]
            }
            Constraint::EqualLineArcLength { line, arc } => {
                vec![line.clone(), arc.clone()]
            }
            Constraint::EqualPointLineDistances { point1, line1, point2, line2 } => {
                vec![point1.clone(), line1.clone(), point2.clone(), line2.clone()]
            }
            Constraint::ArcArcLengthRatio { a, b, .. } | Constraint::ArcArcLengthDifference { a, b, .. } => {
                vec![a.clone(), b.clone()]
            }
            Constraint::ArcLineLengthRatio { arc, line, .. } | Constraint::ArcLineLengthDifference { arc, line, .. } => {
                vec![arc.clone(), line.clone()]
            }
            Constraint::Dragged { point, workplane } => {
                let mut refs = vec![point.clone()];
                if let Some(wp) = workplane {
                    refs.push(wp.clone());
                }
                refs
            }
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
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(10.0)],
                    construction: false,
                    preserve: false,
                },
                Entity::Plane {
                    id: "wp1".to_string(),
                    origin: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                    normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "p2".to_string(),
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![Constraint::Horizontal {
                a: "l1".to_string(),
                workplane: "wp1".to_string(),
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
                construction: false,
                preserve: false,
                at: vec![ExprOrNumber::Number(0.0)],
            }],
            constraints: vec![
                Constraint::Horizontal {
                    a: "l1".to_string(),
                    workplane: "wp1".to_string(),
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
            workplane: "wp1".to_string(),
        };
        assert_eq!(translator.get_constraint_refs(&constraint), vec!["l1", "wp1"]);

        let constraint = Constraint::Perpendicular {
            a: "l1".to_string(),
            b: "l2".to_string(),
        };
        assert_eq!(
            translator.get_constraint_refs(&constraint),
            vec!["l1", "l2"]
        );

        let constraint = Constraint::Coincident {
            data: CoincidentData::PointOnLine {
                at: "p1".to_string(),
                of: vec!["l1".to_string(), "l2".to_string()],
            }
        };
        assert_eq!(
            translator.get_constraint_refs(&constraint),
            vec!["p1", "l1", "l2"]
        );
    }

    #[test]
    fn test_get_constraint_refs_all_variants() {
        use crate::ir::ExprOrNumber;
        let translator = Translator::new();

        // Test all constraint variants
        let constraints = vec![
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
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
            Constraint::Parallel {
                entities: vec!["l1".to_string(), "l2".to_string()],
            },
            Constraint::Horizontal { a: "l1".to_string(), workplane: "wp1".to_string() },
            Constraint::Vertical { a: "l1".to_string(), workplane: "wp1".to_string() },
            Constraint::EqualLength {
                entities: vec!["l1".to_string(), "l2".to_string()],
            },
            Constraint::EqualRadius {
                a: "c1".to_string(),
                b: "c2".to_string(),
            },
            Constraint::Tangent {
                a: "c1".to_string(),
                b: "c2".to_string(),
            },
            Constraint::PointOnLine {
                point: "p1".to_string(),
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
                point: "p1".to_string(),
                of: "p2".to_string(),
            },
            Constraint::Coincident {
                data: CoincidentData::TwoEntities {
                    entities: vec!["p1".to_string(), "p2".to_string()],
                },
            },
        ];

        for constraint in constraints {
            let refs = translator.get_constraint_refs(&constraint);
            assert!(!refs.is_empty(), "Constraint should have at least one reference");
        }
    }

    #[test]
    fn test_translator_default() {
        let translator = Translator::default();
        // Translator is a unit struct, so size is 0, but we can verify it can be created
        let _ = translator;
    }
}
