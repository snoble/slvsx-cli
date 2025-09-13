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
    },
    Line {
        id: String,
        p1: String,
        p2: String,
    },
    Circle {
        id: String,
        center: Vec<ExprOrNumber>,
        diameter: ExprOrNumber,
    },
    Arc {
        id: String,
        center: Vec<ExprOrNumber>,
        start: String,
        end: String,
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
            | Entity::Line { id, .. }
            | Entity::Circle { id, .. }
            | Entity::Arc { id, .. }
            | Entity::Plane { id, .. } => id,
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
    },
    Vertical {
        #[serde(alias = "entity")]
        a: String,
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
    Circle { center: Vec<f64>, diameter: f64 },
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
        };
        assert_eq!(point.id(), "p1");

        let line = Entity::Line {
            id: "l1".into(),
            p1: "p1".into(),
            p2: "p2".into(),
        };
        assert_eq!(line.id(), "l1");

        let circle = Entity::Circle {
            id: "c1".into(),
            center: vec![ExprOrNumber::Number(0.0)],
            diameter: ExprOrNumber::Number(10.0),
        };
        assert_eq!(circle.id(), "c1");
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
                {"type": "horizontal", "a": "l1"}
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
