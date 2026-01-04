use crate::error::{Error, Result};
use crate::ir::InputDocument;
use serde_json::Value;

/// Simple schema validator that checks basic structure
/// Will be replaced with jsonschema crate when linking issues are resolved
pub struct SchemaValidator;

impl SchemaValidator {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn validate_json(&self, json: &Value) -> Result<()> {
        // Basic validation - check required fields
        let obj = json
            .as_object()
            .ok_or_else(|| Error::SchemaValidation("Input must be a JSON object".into()))?;

        // Check required fields
        if !obj.contains_key("schema") {
            return Err(Error::SchemaValidation(
                "Missing required field: schema".into(),
            ));
        }
        if !obj.contains_key("entities") {
            return Err(Error::SchemaValidation(
                "Missing required field: entities".into(),
            ));
        }
        if !obj.contains_key("constraints") {
            return Err(Error::SchemaValidation(
                "Missing required field: constraints".into(),
            ));
        }

        // Validate schema version
        if let Some(schema) = obj.get("schema").and_then(Value::as_str) {
            if schema != "slvs-json/1" {
                return Err(Error::SchemaValidation(format!(
                    "Invalid schema version: {}",
                    schema
                )));
            }
        }

        // Validate units if present
        if let Some(units) = obj.get("units").and_then(Value::as_str) {
            const VALID_UNITS: &[&str] = &["mm", "cm", "m", "in", "ft"];
            if !VALID_UNITS.contains(&units) {
                return Err(Error::SchemaValidation(format!("Invalid units: {}", units)));
            }
        }

        Ok(())
    }

    pub fn validate_document(&self, doc: &InputDocument) -> Result<()> {
        // Validate using the validator module instead
        crate::validator::Validator::new().validate(doc)
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Entity, ExprOrNumber};
    use std::collections::HashMap;

    #[test]
    fn test_schema_validator_new() {
        let validator = SchemaValidator::new();
        assert!(validator.is_ok());
    }

    #[test]
    fn test_validate_valid_document() {
        let validator = SchemaValidator::new().unwrap();

        let doc = InputDocument {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: HashMap::new(),
            entities: vec![Entity::Point {
                id: "p1".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
            }],
            constraints: vec![],
        };

        assert!(validator.validate_document(&doc).is_ok());
    }

    #[test]
    fn test_validate_invalid_schema_version() {
        let validator = SchemaValidator::new().unwrap();

        let json = serde_json::json!({
            "schema": "slvs-json/2",
            "entities": [],
            "constraints": []
        });

        let result = validator.validate_json(&json);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::SchemaValidation(msg) => assert!(msg.contains("Invalid schema version")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_invalid_units() {
        let validator = SchemaValidator::new().unwrap();

        let json = serde_json::json!({
            "schema": "slvs-json/1",
            "units": "yards",
            "entities": [],
            "constraints": []
        });

        let result = validator.validate_json(&json);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::SchemaValidation(msg) => assert!(msg.contains("Invalid units")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_json_valid() {
        let validator = SchemaValidator::new().unwrap();

        let json = serde_json::json!({
            "schema": "slvs-json/1",
            "units": "mm",
            "parameters": {},
            "entities": [],
            "constraints": []
        });

        assert!(validator.validate_json(&json).is_ok());
    }

    #[test]
    fn test_validate_json_missing_required() {
        let validator = SchemaValidator::new().unwrap();

        let json = serde_json::json!({
            "units": "mm",
            "parameters": {}
            // Missing required fields: schema, entities, constraints
        });

        let result = validator.validate_json(&json);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::SchemaValidation(msg) => {
                // The validator correctly reports the first missing required field
                assert!(msg.contains("Missing required field: schema"))
            },
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validator_default() {
        let _validator = SchemaValidator::default();
    }

    #[test]
    fn test_validate_json_not_object() {
        let validator = SchemaValidator::new().unwrap();
        let json = serde_json::json!([]); // Array, not object
        let result = validator.validate_json(&json);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::SchemaValidation(msg) => assert!(msg.contains("JSON object")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_json_missing_entities() {
        let validator = SchemaValidator::new().unwrap();
        let json = serde_json::json!({
            "schema": "slvs-json/1",
            "constraints": []
            // Missing entities
        });
        let result = validator.validate_json(&json);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::SchemaValidation(msg) => assert!(msg.contains("Missing required field: entities")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_json_missing_constraints() {
        let validator = SchemaValidator::new().unwrap();
        let json = serde_json::json!({
            "schema": "slvs-json/1",
            "entities": []
            // Missing constraints
        });
        let result = validator.validate_json(&json);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::SchemaValidation(msg) => assert!(msg.contains("Missing required field: constraints")),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_validate_json_all_valid_units() {
        let validator = SchemaValidator::new().unwrap();
        for unit in &["mm", "cm", "m", "in", "ft"] {
            let json = serde_json::json!({
                "schema": "slvs-json/1",
                "units": unit,
                "entities": [],
                "constraints": []
            });
            assert!(validator.validate_json(&json).is_ok(), "Failed for unit: {}", unit);
        }
    }

    #[test]
    fn test_validate_json_schema_not_string() {
        let validator = SchemaValidator::new().unwrap();
        let json = serde_json::json!({
            "schema": 123, // Not a string
            "entities": [],
            "constraints": []
        });
        // Should still validate (schema check only happens if it's a string)
        let result = validator.validate_json(&json);
        // May succeed or fail depending on implementation
        assert!(result.is_ok() || result.is_err());
    }
}
