use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct InputDocument {
    #[schemars(regex(pattern = r"^slvs-json/1$"))]
    pub schema: String,
    #[serde(default = "default_units")]
    #[schemars(default = "default_units")]
    pub units: String,
    #[serde(default)]
    pub parameters: HashMap<String, f64>,
    pub entities: Vec<Entity>,
    pub constraints: Vec<Constraint>,
}

fn default_units() -> String {
    "mm".to_string()
}

fn default_circle_normal() -> Vec<ExprOrNumber> {
    vec![
        ExprOrNumber::Number(0.0),
        ExprOrNumber::Number(0.0),
        ExprOrNumber::Number(1.0),
    ]
}

/// Represents a position that can be either coordinates or a reference to a point entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum PositionOrRef {
    /// Reference to a point entity by ID
    Reference(String),
    /// Direct coordinates [x, y, z]
    Coordinates(Vec<ExprOrNumber>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Parameter {
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Entity {
    Point {
        id: String,
        at: Vec<ExprOrNumber>,
        #[serde(default)]
        construction: bool,
        #[serde(default)]
        preserve: bool, // Mark as dragged - minimize changes during solving
    },
    Point2D {
        id: String,
        at: Vec<ExprOrNumber>, // [u, v] - 2D coordinates
        workplane: String,
        #[serde(default)]
        construction: bool,
        #[serde(default)]
        preserve: bool, // Mark as dragged - minimize changes during solving
    },
    Line {
        id: String,
        p1: String,
        p2: String,
        #[serde(default)]
        construction: bool,
        #[serde(default)]
        preserve: bool, // Mark as dragged - minimize changes during solving
    },
    Line2D {
        id: String,
        p1: String, // Point2D entity ID
        p2: String, // Point2D entity ID
        workplane: String, // Workplane/Plane entity ID
        #[serde(default)]
        construction: bool,
        #[serde(default)]
        preserve: bool, // Mark as dragged - minimize changes during solving
    },
    Circle {
        id: String,
        /// Center position - either [x, y, z] coordinates or a point entity reference
        center: PositionOrRef,
        diameter: ExprOrNumber,
        /// Normal vector defining the plane of the circle [nx, ny, nz]
        /// Defaults to [0, 0, 1] (XY plane) if not specified
        #[serde(default = "default_circle_normal")]
        normal: Vec<ExprOrNumber>,
        #[serde(default)]
        construction: bool,
        #[serde(default)]
        preserve: bool, // Mark as dragged - minimize changes during solving
    },
    Arc {
        id: String,
        center: String, // Point entity ID
        start: String,  // Point entity ID
        end: String,    // Point entity ID
        normal: Vec<ExprOrNumber>, // Normal vector [nx, ny, nz] or reference to normal entity
        #[serde(skip_serializing_if = "Option::is_none")]
        workplane: Option<String>, // Optional workplane for 2D arcs
        #[serde(default)]
        construction: bool,
        #[serde(default)]
        preserve: bool, // Mark as dragged - minimize changes during solving
    },
    Cubic {
        id: String,
        control_points: Vec<String>, // 4 point entity IDs: [p0, p1, p2, p3]
        #[serde(skip_serializing_if = "Option::is_none")]
        workplane: Option<String>, // Optional workplane for 2D cubics
        #[serde(default)]
        construction: bool,
        #[serde(default)]
        preserve: bool, // Mark as dragged - minimize changes during solving
    },
    Plane {
        id: String,
        origin: Vec<ExprOrNumber>,
        normal: Vec<ExprOrNumber>,
    },
}

impl Entity {
    pub fn id(&self) -> &str {
        match self {
            Entity::Point { id, .. }
            | Entity::Point2D { id, .. }
            | Entity::Line { id, .. }
            | Entity::Line2D { id, .. }
            | Entity::Circle { id, .. }
            | Entity::Arc { id, .. }
            | Entity::Cubic { id, .. }
            | Entity::Plane { id, .. } => id,
        }
    }
    
    pub fn is_construction(&self) -> bool {
        match self {
            Entity::Point { construction, .. }
            | Entity::Point2D { construction, .. }
            | Entity::Line { construction, .. }
            | Entity::Line2D { construction, .. }
            | Entity::Circle { construction, .. }
            | Entity::Arc { construction, .. }
            | Entity::Cubic { construction, .. } => *construction,
            Entity::Plane { .. } => false,
        }
    }
    
    pub fn should_preserve(&self) -> bool {
        match self {
            Entity::Point { preserve, .. }
            | Entity::Point2D { preserve, .. }
            | Entity::Line { preserve, .. }
            | Entity::Line2D { preserve, .. }
            | Entity::Circle { preserve, .. }
            | Entity::Arc { preserve, .. }
            | Entity::Cubic { preserve, .. } => *preserve,
            Entity::Plane { .. } => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum ExprOrNumber {
    Number(f64),
    Expression(String),
}

impl Default for ExprOrNumber {
    fn default() -> Self {
        ExprOrNumber::Number(0.0)
    }
}

impl ExprOrNumber {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            ExprOrNumber::Number(n) => Some(*n),
            ExprOrNumber::Expression(_) => None,
        }
    }

    pub fn as_expr(&self) -> Option<&str> {
        match self {
            ExprOrNumber::Number(_) => None,
            ExprOrNumber::Expression(e) => Some(e),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum CoincidentData {
    PointOnLine {
        at: String,
        of: Vec<String>,
    },
    TwoEntities {
        entities: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Constraint {
    Coincident {
        #[serde(flatten)]
        data: CoincidentData,
    },
    Distance {
        between: Vec<String>,
        value: ExprOrNumber,
    },
    Angle {
        between: Vec<String>,
        value: ExprOrNumber,
    },
    Perpendicular {
        #[serde(alias = "entities")]
        a: String,
        b: String,
    },
    Parallel {
        entities: Vec<String>,
    },
    Horizontal {
        #[serde(alias = "entity")]
        a: String,
        /// Workplane defining the coordinate system for "horizontal" (required by SolveSpace)
        workplane: String,
    },
    Vertical {
        #[serde(alias = "entity")]
        a: String,
        /// Workplane defining the coordinate system for "vertical" (required by SolveSpace)
        workplane: String,
    },
    EqualLength {
        entities: Vec<String>,
    },
    EqualRadius {
        a: String,
        b: String,
    },
    Tangent {
        a: String,
        b: String,
    },
    PointOnLine {
        point: String,
        line: String,
    },
    PointOnCircle {
        point: String,
        circle: String,
    },
    Fixed {
        entity: String,
        /// Workplane for 2D points (required for Point2D, omit for 3D Point)
        #[serde(skip_serializing_if = "Option::is_none")]
        workplane: Option<String>,
    },
    Symmetric {
        a: String,
        b: String,
        about: String,
    },
    Midpoint {
        point: String,
        of: String,
    },
    PointInPlane {
        point: String,
        plane: String,
    },
    PointPlaneDistance {
        point: String,
        plane: String,
        value: ExprOrNumber,
    },
    PointLineDistance {
        point: String,
        line: String,
        value: ExprOrNumber,
    },
    LengthRatio {
        a: String,
        b: String,
        value: ExprOrNumber,
    },
    EqualAngle {
        lines: Vec<String>, // 4 lines: [line1, line2, line3, line4]
    },
    SymmetricHorizontal {
        a: String,
        b: String,
        /// Workplane for the symmetry constraint (required for 2D constraints)
        workplane: String,
    },
    SymmetricVertical {
        a: String,
        b: String,
        /// Workplane for the symmetry constraint (required for 2D constraints)
        workplane: String,
    },
    Diameter {
        circle: String,
        value: ExprOrNumber,
    },
    SameOrientation {
        a: String,
        b: String,
    },
    ProjectedPointDistance {
        a: String,
        b: String,
        plane: String,
        value: ExprOrNumber,
    },
    LengthDifference {
        a: String,
        b: String,
        value: ExprOrNumber,
    },
    PointOnFace {
        point: String,
        face: String,
    },
    PointFaceDistance {
        point: String,
        face: String,
        value: ExprOrNumber,
    },
    EqualLineArcLength {
        line: String,
        arc: String,
    },
    EqualLengthPointLineDistance {
        line: String,
        point: String,
        reference_line: String,
    },
    EqualPointLineDistances {
        point1: String,
        line1: String,
        point2: String,
        line2: String,
    },
    CubicLineTangent {
        cubic: String,
        line: String,
    },
    ArcArcLengthRatio {
        a: String,
        b: String,
        value: ExprOrNumber,
    },
    ArcLineLengthRatio {
        arc: String,
        line: String,
        value: ExprOrNumber,
    },
    ArcArcLengthDifference {
        a: String,
        b: String,
        value: ExprOrNumber,
    },
    ArcLineLengthDifference {
        arc: String,
        line: String,
        value: ExprOrNumber,
    },
    Dragged {
        point: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        workplane: Option<String>, // Optional workplane for 2D points
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct SolveResult {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<HashMap<String, ResolvedEntity>>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Diagnostics {
    pub iters: u32,
    pub residual: f64,
    pub dof: u32,
    pub time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum ResolvedEntity {
    Point { at: Vec<f64> },
    Circle { center: Vec<f64>, diameter: f64, normal: Vec<f64> },
    Line { p1: Vec<f64>, p2: Vec<f64> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_units() {
        assert_eq!(default_units(), "mm");
    }

    #[test]
    fn test_entity_id() {
        let point = Entity::Point {
            id: "p1".into(),
            at: vec![ExprOrNumber::Number(0.0)],
            construction: false,
            preserve: false,
        };
        assert_eq!(point.id(), "p1");

        let line = Entity::Line {
            id: "l1".into(),
            p1: "p1".into(),
            p2: "p2".into(),
            construction: false,
            preserve: false,
        };
        assert_eq!(line.id(), "l1");

        let circle = Entity::Circle {
            id: "c1".into(),
            center: PositionOrRef::Coordinates(vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)]),
            diameter: ExprOrNumber::Number(10.0),
            normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            construction: false,
            preserve: false,
        };
        assert_eq!(circle.id(), "c1");
    }

    #[test]
    fn test_preserve_flag() {
        let preserved_point = Entity::Point {
            id: "p1".into(),
            at: vec![ExprOrNumber::Number(0.0)],
            construction: false,
            preserve: true,
        };
        assert!(preserved_point.should_preserve());

        let normal_point = Entity::Point {
            id: "p2".into(),
            at: vec![ExprOrNumber::Number(0.0)],
            construction: false,
            preserve: false,
        };
        assert!(!normal_point.should_preserve());

        let preserved_2d_point = Entity::Point2D {
            id: "p3".into(),
            at: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)],
            workplane: "wp1".into(),
            construction: false,
            preserve: true,
        };
        assert!(preserved_2d_point.should_preserve());

        let preserved_line = Entity::Line {
            id: "l1".into(),
            p1: "p1".into(),
            p2: "p2".into(),
            construction: false,
            preserve: true,
        };
        assert!(preserved_line.should_preserve());

        let preserved_circle = Entity::Circle {
            id: "c1".into(),
            center: PositionOrRef::Coordinates(vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)]),
            diameter: ExprOrNumber::Number(10.0),
            normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            construction: false,
            preserve: true,
        };
        assert!(preserved_circle.should_preserve());

        let preserved_arc = Entity::Arc {
            id: "a1".into(),
            center: "center".into(),
            start: "start".into(),
            end: "end".into(),
            normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            workplane: None,
            construction: false,
            preserve: true,
        };
        assert!(preserved_arc.should_preserve());

        let preserved_cubic = Entity::Cubic {
            id: "cubic1".into(),
            control_points: vec!["p0".into(), "p1".into(), "p2".into(), "p3".into()],
            workplane: None,
            construction: false,
            preserve: true,
        };
        assert!(preserved_cubic.should_preserve());

        // Plane doesn't have preserve flag
        let plane = Entity::Plane {
            id: "wp1".into(),
            origin: vec![ExprOrNumber::Number(0.0)],
            normal: vec![ExprOrNumber::Number(0.0)],
        };
        assert!(!plane.should_preserve());
    }

    #[test]
    fn test_preserve_flag_serialization() {
        // Test that preserve flag defaults to false when not specified
        let json = r#"{"type":"point","id":"p1","at":[0.0,0.0,0.0]}"#;
        let entity: Entity = serde_json::from_str(json).unwrap();
        match entity {
            Entity::Point { preserve, .. } => assert!(!preserve, "Preserve should default to false"),
            _ => panic!("Wrong entity type"),
        }

        // Test that preserve flag can be set to true
        let json = r#"{"type":"point","id":"p1","at":[0.0,0.0,0.0],"preserve":true}"#;
        let entity: Entity = serde_json::from_str(json).unwrap();
        match entity {
            Entity::Point { preserve, .. } => assert!(preserve, "Preserve should be true"),
            _ => panic!("Wrong entity type"),
        }
    }

    #[test]
    fn test_expr_or_number() {
        let num = ExprOrNumber::Number(42.0);
        assert_eq!(num.as_f64(), Some(42.0));
        assert_eq!(num.as_expr(), None);

        let expr = ExprOrNumber::Expression("W/2".into());
        assert_eq!(expr.as_f64(), None);
        assert_eq!(expr.as_expr(), Some("W/2"));
    }

    #[test]
    fn test_input_document_deserialize() {
        let json = r#"{
            "schema": "slvs-json/1",
            "units": "mm",
            "parameters": {"W": 100},
            "entities": [
                {"id": "p1", "type": "point", "at": [0, 0, 0]}
            ],
            "constraints": [
                {"type": "horizontal", "a": "l1", "workplane": "wp1"}
            ]
        }"#;

        let doc: InputDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.schema, "slvs-json/1");
        assert_eq!(doc.units, "mm");
        assert_eq!(doc.parameters.get("W"), Some(&100.0));
        assert_eq!(doc.entities.len(), 1);
        assert_eq!(doc.constraints.len(), 1);
    }

    #[test]
    fn test_input_document_default_units() {
        let json = r#"{
            "schema": "slvs-json/1",
            "parameters": {},
            "entities": [],
            "constraints": []
        }"#;

        let doc: InputDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.units, "mm");
    }

    #[test]
    fn test_constraint_serialization() {
        let constraint = Constraint::Perpendicular {
            a: "l1".into(),
            b: "l2".into(),
        };
        let json = serde_json::to_string(&constraint).unwrap();
        assert!(json.contains("\"type\":\"perpendicular\""));
        assert!(json.contains("\"a\":\"l1\""));
        assert!(json.contains("\"b\":\"l2\""));
    }

    #[test]
    fn test_expr_or_number_as_f64() {
        assert_eq!(ExprOrNumber::Number(42.5).as_f64(), Some(42.5));
        assert_eq!(ExprOrNumber::Expression("x".to_string()).as_f64(), None);
    }

    #[test]
    fn test_expr_or_number_as_expr() {
        assert_eq!(ExprOrNumber::Number(42.5).as_expr(), None);
        assert_eq!(ExprOrNumber::Expression("x".to_string()).as_expr(), Some("x"));
    }

    #[test]
    fn test_expr_or_number_default() {
        let default = ExprOrNumber::default();
        assert_eq!(default.as_f64(), Some(0.0));
    }

    #[test]
    fn test_entity_id_all_variants() {
        let point = Entity::Point {
            id: "p1".to_string(),
            at: vec![ExprOrNumber::Number(0.0)],
            construction: false,
            preserve: false,
        };
        assert_eq!(point.id(), "p1");

        let line = Entity::Line {
            id: "l1".to_string(),
            p1: "p1".to_string(),
            p2: "p2".to_string(),
            construction: false,
            preserve: false,
        };
        assert_eq!(line.id(), "l1");

        let circle = Entity::Circle {
            id: "c1".to_string(),
            center: PositionOrRef::Coordinates(vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0)]),
            diameter: ExprOrNumber::Number(10.0),
            normal: vec![ExprOrNumber::Number(0.0), ExprOrNumber::Number(0.0), ExprOrNumber::Number(1.0)],
            construction: false,
            preserve: false,
        };
        assert_eq!(circle.id(), "c1");
    }

    #[test]
    fn test_resolved_entity_serialization() {
        use serde_json;
        let point = ResolvedEntity::Point { at: vec![1.0, 2.0, 3.0] };
        let json = serde_json::to_string(&point).unwrap();
        assert!(json.contains("1"));
        assert!(json.contains("2"));
        assert!(json.contains("3"));
    }

    #[test]
    fn test_solve_result_skip_empty() {
        let result = SolveResult {
            status: "ok".into(),
            diagnostics: None,
            entities: None,
            warnings: vec![],
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(!json.contains("warnings"));
        assert!(!json.contains("diagnostics"));
        assert!(!json.contains("entities"));
    }
}
