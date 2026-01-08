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
        self.validate_constraint_entity_types(doc)?;
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
                Constraint::Fixed { entity, .. } => vec![entity.as_str()],
                Constraint::Distance { between, .. } => between.iter().map(|s| s.as_str()).collect(),
                Constraint::Angle { between, .. } => between.iter().map(|s| s.as_str()).collect(),
                Constraint::Perpendicular { a, b } | Constraint::EqualRadius { a, b } | Constraint::Tangent { a, b } | 
                Constraint::SameOrientation { a, b } | Constraint::CubicLineTangent { cubic: a, line: b } => {
                    vec![a.as_str(), b.as_str()]
                }
                Constraint::Parallel { entities } | Constraint::EqualLength { entities } => {
                    entities.iter().map(|s| s.as_str()).collect()
                }
                Constraint::EqualAngle { lines } => {
                    lines.iter().map(|s| s.as_str()).collect()
                }
                Constraint::Horizontal { a, workplane } | Constraint::Vertical { a, workplane } => {
                    vec![a.as_str(), workplane.as_str()]
                }
                Constraint::Diameter { circle: a, .. } => {
                    vec![a.as_str()]
                }
                Constraint::PointOnLine { point, line } | Constraint::PointLineDistance { point, line, .. } | 
                Constraint::EqualLengthPointLineDistance { point, line, .. } => {
                    vec![point.as_str(), line.as_str()]
                }
                Constraint::PointOnCircle { point, circle } => vec![point.as_str(), circle.as_str()],
                Constraint::Symmetric { a, b, about } => vec![a.as_str(), b.as_str(), about.as_str()],
                Constraint::SymmetricHorizontal { a, b, workplane } | Constraint::SymmetricVertical { a, b, workplane } => {
                    vec![a.as_str(), b.as_str(), workplane.as_str()]
                }
                Constraint::Midpoint { point, of } => {
                    let mut refs = vec![point.as_str()];
                    refs.push(of.as_str());
                    refs
                }
                Constraint::PointInPlane { point, plane } | Constraint::PointPlaneDistance { point, plane, .. } => {
                    vec![point.as_str(), plane.as_str()]
                }
                Constraint::LengthRatio { a, b, .. } | Constraint::LengthDifference { a, b, .. } => {
                    vec![a.as_str(), b.as_str()]
                }
                Constraint::ProjectedPointDistance { a, b, plane, .. } => {
                    vec![a.as_str(), b.as_str(), plane.as_str()]
                }
                Constraint::PointOnFace { point, face } | Constraint::PointFaceDistance { point, face, .. } => {
                    vec![point.as_str(), face.as_str()]
                }
                Constraint::EqualLineArcLength { line, arc } => {
                    vec![line.as_str(), arc.as_str()]
                }
                Constraint::EqualPointLineDistances { point1, line1, point2, line2 } => {
                    vec![point1.as_str(), line1.as_str(), point2.as_str(), line2.as_str()]
                }
                Constraint::ArcArcLengthRatio { a, b, .. } | Constraint::ArcArcLengthDifference { a, b, .. } => {
                    vec![a.as_str(), b.as_str()]
                }
                Constraint::ArcLineLengthRatio { arc, line, .. } | Constraint::ArcLineLengthDifference { arc, line, .. } => {
                    vec![arc.as_str(), line.as_str()]
                }
                Constraint::Dragged { point, workplane } => {
                    let mut refs = vec![point.as_str()];
                    if let Some(wp) = workplane {
                        refs.push(wp.as_str());
                    }
                    refs
                }
                Constraint::Collinear { points } => {
                    points.iter().map(|s| s.as_str()).collect()
                }
                Constraint::EqualAngles { lines, .. } => {
                    lines.iter().map(|s| s.as_str()).collect()
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

    fn validate_constraint_entity_types(&self, doc: &InputDocument) -> Result<()> {
        use crate::ir::{Constraint, Entity};
        use std::collections::HashMap;

        // Build entity type map
        let entity_types: HashMap<&str, &str> = doc.entities.iter().map(|e| {
            let type_name = match e {
                Entity::Point { .. } | Entity::Point2D { .. } => "point",
                Entity::Line { .. } | Entity::Line2D { .. } => "line",
                Entity::Circle { .. } => "circle",
                Entity::Arc { .. } => "arc",
                Entity::Plane { .. } => "plane",
                Entity::Cubic { .. } => "cubic",
            };
            (e.id(), type_name)
        }).collect();

        for (idx, constraint) in doc.constraints.iter().enumerate() {
            // Symmetric constraint about a line doesn't work in 3D
            if let Constraint::Symmetric { a, b, about } = constraint {
                return Err(Error::InvalidInput {
                    message: format!(
                        "The 'symmetric' constraint (about line '{}') is not supported in 3D mode. \
                        Use 'symmetric_horizontal' or 'symmetric_vertical' with a workplane instead. \
                        Example: {{\"type\": \"symmetric_horizontal\", \"a\": \"{}\", \"b\": \"{}\", \"workplane\": \"your_plane\"}}",
                        about, a, b
                    ),
                    pointer: Some(format!("/constraints/{}", idx)),
                });
            }

            // Tangent constraint only works with arc, cubic, and line - NOT circle
            if let Constraint::Tangent { a, b } = constraint {
                let type_a = entity_types.get(a.as_str()).copied().unwrap_or("unknown");
                let type_b = entity_types.get(b.as_str()).copied().unwrap_or("unknown");
                
                const VALID_TANGENT_TYPES: &[&str] = &["arc", "cubic", "line"];
                
                if !VALID_TANGENT_TYPES.contains(&type_a) {
                    return Err(Error::InvalidInput {
                        message: format!(
                            "Tangent constraint cannot be applied to entity '{}' (type: {}). \
                            Tangent constraints only work with arc, cubic, or line entities. \
                            Circles are not supported - use an Arc entity instead.",
                            a, type_a
                        ),
                        pointer: Some(format!("/constraints/{}", idx)),
                    });
                }
                if !VALID_TANGENT_TYPES.contains(&type_b) {
                    return Err(Error::InvalidInput {
                        message: format!(
                            "Tangent constraint cannot be applied to entity '{}' (type: {}). \
                            Tangent constraints only work with arc, cubic, or line entities. \
                            Circles are not supported - use an Arc entity instead.",
                            b, type_b
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
        // Build set incrementally to prevent forward references
        // This matches how the solver processes entities sequentially
        // Only track Point entities since Line.p1/p2 and Arc.start/end must reference Points
        let mut seen_point_ids = HashSet::new();
        // Track all entity IDs to provide better error messages
        let mut seen_entity_ids = HashSet::new();

        for (idx, entity) in doc.entities.iter().enumerate() {
            match entity {
                Entity::Line { p1, p2, .. } => {
                    if !seen_point_ids.contains(p1.as_str()) {
                        if !seen_entity_ids.contains(p1.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Line entity #{} references point '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, p1
                                ),
                                pointer: Some(format!("/entities/{}/p1", idx)),
                            });
                        } else {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Line entity #{} references '{}' which is not a Point entity. Line endpoints must reference Point entities.",
                                    idx + 1, p1
                                ),
                                pointer: Some(format!("/entities/{}/p1", idx)),
                            });
                        }
                    }
                    if !seen_point_ids.contains(p2.as_str()) {
                        if !seen_entity_ids.contains(p2.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Line entity #{} references point '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, p2
                                ),
                                pointer: Some(format!("/entities/{}/p2", idx)),
                            });
                        } else {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Line entity #{} references '{}' which is not a Point entity. Line endpoints must reference Point entities.",
                                    idx + 1, p2
                                ),
                                pointer: Some(format!("/entities/{}/p2", idx)),
                            });
                        }
                    }
                }
                Entity::Line2D { p1, p2, workplane, .. } => {
                    // Validate that p1 and p2 reference Point2D entities
                    if !seen_point_ids.contains(p1.as_str()) {
                        if !seen_entity_ids.contains(p1.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Line2D entity #{} references point '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, p1
                                ),
                                pointer: Some(format!("/entities/{}/p1", idx)),
                            });
                        } else {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Line2D entity #{} references '{}' which is not a Point2D entity. Line2D endpoints must reference Point2D entities.",
                                    idx + 1, p1
                                ),
                                pointer: Some(format!("/entities/{}/p1", idx)),
                            });
                        }
                    }
                    if !seen_point_ids.contains(p2.as_str()) {
                        if !seen_entity_ids.contains(p2.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Line2D entity #{} references point '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, p2
                                ),
                                pointer: Some(format!("/entities/{}/p2", idx)),
                            });
                        } else {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Line2D entity #{} references '{}' which is not a Point2D entity. Line2D endpoints must reference Point2D entities.",
                                    idx + 1, p2
                                ),
                                pointer: Some(format!("/entities/{}/p2", idx)),
                            });
                        }
                    }
                    // Validate workplane reference
                    if !seen_entity_ids.contains(workplane.as_str()) {
                        return Err(Error::InvalidInput {
                            message: format!(
                                "Line2D entity #{} references workplane '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                idx + 1, workplane
                            ),
                            pointer: Some(format!("/entities/{}/workplane", idx)),
                        });
                    }
                }
                Entity::Arc { center, start, end, workplane, .. } => {
                    // Validate center point reference
                    if !seen_point_ids.contains(center.as_str()) {
                        if !seen_entity_ids.contains(center.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Arc entity #{} references point '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, center
                                ),
                                pointer: Some(format!("/entities/{}/center", idx)),
                            });
                        } else {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Arc entity #{} references '{}' which is not a Point entity. Arc center must reference a Point entity.",
                                    idx + 1, center
                                ),
                                pointer: Some(format!("/entities/{}/center", idx)),
                            });
                        }
                    }
                    // Validate start point reference
                    if !seen_point_ids.contains(start.as_str()) {
                        if !seen_entity_ids.contains(start.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Arc entity #{} references point '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, start
                                ),
                                pointer: Some(format!("/entities/{}/start", idx)),
                            });
                        } else {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Arc entity #{} references '{}' which is not a Point entity. Arc start/end points must reference Point entities.",
                                    idx + 1, start
                                ),
                                pointer: Some(format!("/entities/{}/start", idx)),
                            });
                        }
                    }
                    // Validate end point reference
                    if !seen_point_ids.contains(end.as_str()) {
                        if !seen_entity_ids.contains(end.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Arc entity #{} references point '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, end
                                ),
                                pointer: Some(format!("/entities/{}/end", idx)),
                            });
                        } else {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Arc entity #{} references '{}' which is not a Point entity. Arc start/end points must reference Point entities.",
                                    idx + 1, end
                                ),
                                pointer: Some(format!("/entities/{}/end", idx)),
                            });
                        }
                    }
                    // Validate workplane if specified
                    if let Some(wp_id) = workplane {
                        if !seen_entity_ids.contains(wp_id.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Arc entity #{} references workplane '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, wp_id
                                ),
                                pointer: Some(format!("/entities/{}/workplane", idx)),
                            });
                        }
                    }
                }
                Entity::Cubic { control_points, workplane, .. } => {
                    // Validate all control points
                    for (pt_idx, pt_id) in control_points.iter().enumerate() {
                        if !seen_point_ids.contains(pt_id.as_str()) {
                            if !seen_entity_ids.contains(pt_id.as_str()) {
                                return Err(Error::InvalidInput {
                                    message: format!(
                                        "Cubic entity #{} control point #{} '{}' is not yet defined. Entities must be defined before they are referenced.",
                                        idx + 1, pt_idx + 1, pt_id
                                    ),
                                    pointer: Some(format!("/entities/{}/control_points/{}", idx, pt_idx)),
                                });
                            } else {
                                return Err(Error::InvalidInput {
                                    message: format!(
                                        "Cubic entity #{} control point #{} '{}' is not a Point entity. Cubic control points must reference Point entities.",
                                        idx + 1, pt_idx + 1, pt_id
                                    ),
                                    pointer: Some(format!("/entities/{}/control_points/{}", idx, pt_idx)),
                                });
                            }
                        }
                    }
                    // Validate workplane if specified
                    if let Some(wp_id) = workplane {
                        if !seen_entity_ids.contains(wp_id.as_str()) {
                            return Err(Error::InvalidInput {
                                message: format!(
                                    "Cubic entity #{} references workplane '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                    idx + 1, wp_id
                                ),
                                pointer: Some(format!("/entities/{}/workplane", idx)),
                            });
                        }
                    }
                }
                Entity::Point2D { workplane, .. } => {
                    // Validate workplane reference
                    if !seen_entity_ids.contains(workplane.as_str()) {
                        return Err(Error::InvalidInput {
                            message: format!(
                                "Point2D entity #{} references workplane '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                idx + 1, workplane
                            ),
                            pointer: Some(format!("/entities/{}/workplane", idx)),
                        });
                    }
                    // Track Point2D as a Point for reference validation
                    seen_point_ids.insert(entity.id());
                }
                Entity::Point { .. } => {
                    // Track Point entities separately
                    seen_point_ids.insert(entity.id());
                }
                Entity::Circle { id, center, .. } => {
                    // Validate center point reference if it's a reference (not coordinates)
                    if let crate::ir::PositionOrRef::Reference(point_id) = center {
                        if !seen_point_ids.contains(point_id.as_str()) {
                            if !seen_entity_ids.contains(point_id.as_str()) {
                                return Err(Error::InvalidInput {
                                    message: format!(
                                        "Circle entity '{}' references point '{}' that is not yet defined. Entities must be defined before they are referenced.",
                                        id, point_id
                                    ),
                                    pointer: Some(format!("/entities/{}/center", idx)),
                                });
                            } else {
                                return Err(Error::InvalidInput {
                                    message: format!(
                                        "Circle entity '{}' references '{}' which is not a Point entity. Circle center must reference a Point or Point2D entity.",
                                        id, point_id
                                    ),
                                    pointer: Some(format!("/entities/{}/center", idx)),
                                });
                            }
                        }
                    }
                }
                Entity::Plane { .. } => {
                    // Planes don't reference other entities
                }
            }
            // Add this entity's ID to the set after checking its references
            seen_entity_ids.insert(entity.id());
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
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
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
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
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
                construction: false,
                preserve: false,
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
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![Constraint::Fixed { entity: "p1".to_string(), workplane: None }],
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
                construction: false,
                preserve: false,
            }],
            constraints: vec![Constraint::Fixed { entity: "nonexistent".to_string(), workplane: None }],
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
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "p2".to_string(),
                    construction: false,
                    preserve: false,
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
                    construction: false,
                    preserve: false,
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "nonexistent".to_string(), // p2 doesn't exist
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("not yet defined"));
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
                    construction: false,
                    preserve: false,
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "nonexistent".to_string(), // p1 doesn't exist
                    p2: "p2".to_string(),
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("not yet defined"));
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
                    construction: false,
                    preserve: false,
                },
                Entity::Arc {
                    id: "a1".to_string(),
                    center: "end".to_string(),
                    start: "nonexistent".to_string(), // start doesn't exist
                    end: "end".to_string(),
                    normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                    workplane: None,
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("not yet defined"));
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
                    construction: false,
                    preserve: false,
                },
                Entity::Arc {
                    id: "a1".to_string(),
                    center: "start".to_string(),
                    start: "start".to_string(),
                    end: "nonexistent".to_string(), // end doesn't exist
                    normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                    workplane: None,
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("not yet defined"));
                assert!(message.contains("nonexistent"));
                assert_eq!(pointer, Some("/entities/1/end".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_forward_reference_rejected() {
        use crate::ir::Entity;
        let validator = Validator::new();
        // Test forward reference: Line references p2 before p2 is defined
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
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "p2".to_string(), // p2 is defined later - forward reference!
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err(), "Forward references should be rejected");
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("not yet defined"));
                assert!(message.contains("p2"));
                assert_eq!(pointer, Some("/entities/1/p2".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_line_references_circle() {
        use crate::ir::Entity;
        let validator = Validator::new();
        // Test that Line cannot reference a Circle entity
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
                Entity::Circle {
                    id: "c1".to_string(),
                    center: crate::ir::PositionOrRef::Coordinates(vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)]),
                    diameter: ExprOrNumber::Number(10.0),
                    normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "c1".to_string(), // c1 is a Circle, not a Point!
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err(), "Line should not be able to reference Circle");
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("not a Point entity"));
                assert!(message.contains("c1"));
                assert!(message.contains("Line endpoints must reference Point entities"));
                assert_eq!(pointer, Some("/entities/2/p2".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_arc_references_circle() {
        use crate::ir::Entity;
        let validator = Validator::new();
        // Test that Arc cannot reference a Circle entity
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
                Entity::Circle {
                    id: "c1".to_string(),
                    center: crate::ir::PositionOrRef::Coordinates(vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)]),
                    diameter: ExprOrNumber::Number(10.0),
                    normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
                },
                Entity::Arc {
                    id: "a1".to_string(),
                    center: "p1".to_string(),
                    start: "p1".to_string(),
                    end: "c1".to_string(), // c1 is a Circle, not a Point!
                    normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                    workplane: None,
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err(), "Arc should not be able to reference Circle");
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("not a Point entity"));
                assert!(message.contains("c1"));
                assert!(message.contains("Arc start/end points must reference Point entities"));
                assert_eq!(pointer, Some("/entities/2/end".to_string()));
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_entity_references_line_references_line() {
        use crate::ir::Entity;
        let validator = Validator::new();
        // Test that Line cannot reference another Line entity
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
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "p1".to_string(), // Self-reference is valid
                    construction: false,
                    preserve: false,
                },
                Entity::Line {
                    id: "l2".to_string(),
                    p1: "p1".to_string(),
                    p2: "l1".to_string(), // l1 is a Line, not a Point!
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![],
        };
        let result = validator.validate_entity_references(&doc);
        assert!(result.is_err(), "Line should not be able to reference Line");
        match result.unwrap_err() {
            Error::InvalidInput { message, pointer } => {
                assert!(message.contains("not a Point entity"));
                assert!(message.contains("l1"));
                assert!(message.contains("Line endpoints must reference Point entities"));
                assert_eq!(pointer, Some("/entities/2/p2".to_string()));
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
            constraints: vec![Constraint::Fixed { entity: "nonexistent".to_string(), workplane: None }],
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
                Entity::Plane {
                    id: "wp1".to_string(),
                    origin: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
                    normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                },
                Entity::Point {
                    id: "p1".to_string(),
                    at: vec![ExprOrNumber::Number(0.0)],
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "p2".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "p3".to_string(),
                    at: vec![ExprOrNumber::Number(2.0)],
                    construction: false,
                    preserve: false,
                },
                Entity::Line {
                    id: "l1".to_string(),
                    p1: "p1".to_string(),
                    p2: "p2".to_string(),
                    construction: false,
                    preserve: false,
                },
                Entity::Line {
                    id: "l2".to_string(),
                    p1: "p2".to_string(),
                    p2: "p3".to_string(),
                    construction: false,
                    preserve: false,
                },
                Entity::Circle {
                    id: "c1".to_string(),
                    center: crate::ir::PositionOrRef::Coordinates(vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)]),
                    diameter: ExprOrNumber::Number(10.0),
                    normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
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
                    workplane: "wp1".to_string(),
                },
                Constraint::Vertical {
                    a: "l2".to_string(),
                    workplane: "wp1".to_string(),
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
                construction: false,
                preserve: false,
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
                construction: false,
                preserve: false,
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
                construction: false,
                preserve: false,
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
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "apple".to_string(),
                    at: vec![ExprOrNumber::Number(1.0)],
                    construction: false,
                    preserve: false,
                },
                Entity::Point {
                    id: "banana".to_string(),
                    at: vec![ExprOrNumber::Number(2.0)],
                    construction: false,
                    preserve: false,
                },
            ],
            constraints: vec![Constraint::Fixed { entity: "nonexistent".to_string(), workplane: None }],
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
