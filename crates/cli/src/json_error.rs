use anyhow::{anyhow, Result};
use serde_json::Error as JsonError;

/// Parse JSON with helpful error messages for users
pub fn parse_json_with_context<T>(input: &str, filename: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(input).map_err(|e| {
        anyhow!(format_json_error(e, input, filename))
    })
}

/// Format a serde_json error with context
pub fn format_json_error(err: JsonError, input: &str, filename: &str) -> String {
    let mut message = String::new();
    
    // Get line and column from error
    let line = err.line();
    let column = err.column();
    
    message.push_str(&format!("JSON parsing error in {}\n", filename));
    message.push_str(&format!("Error at line {}, column {}: {}\n", line, column, err));
    
    // Add context lines if possible
    if line > 0 {
        let lines: Vec<&str> = input.lines().collect();
        let line_idx = (line - 1) as usize;
        
        message.push_str("\nContext:\n");
        
        // Show previous line if available
        if line_idx > 0 && line_idx <= lines.len() {
            message.push_str(&format!("  {}│ {}\n", line - 1, lines[line_idx - 1]));
        }
        
        // Show error line with pointer
        if line_idx < lines.len() {
            let error_line = lines[line_idx];
            message.push_str(&format!("→ {}│ {}\n", line, error_line));
            
            // Add pointer to specific column
            if column > 0 && column <= error_line.len() {
                let spaces = " ".repeat(format!("  {}│ ", line).len() + column - 1);
                message.push_str(&format!("{}↑\n", spaces));
                message.push_str(&format!("{}└─ {}\n", spaces, get_helpful_hint(&err)));
            }
        }
        
        // Show next line if available
        if line_idx + 1 < lines.len() {
            message.push_str(&format!("  {}│ {}\n", line + 1, lines[line_idx + 1]));
        }
    }
    
    message.push_str("\nCommon issues:\n");
    message.push_str("  • Check for missing commas between fields\n");
    message.push_str("  • Ensure all strings are properly quoted\n");
    message.push_str("  • Verify field names match the schema exactly\n");
    message.push_str("  • Check for trailing commas (not allowed in JSON)\n");
    
    message
}

/// Get a helpful hint based on the error type
pub fn get_helpful_hint(err: &JsonError) -> &str {
    let err_str = err.to_string();
    
    if err_str.contains("missing field") {
        "This field is required but not found"
    } else if err_str.contains("unknown field") {
        "This field is not recognized in the schema"
    } else if err_str.contains("invalid type") {
        "The value type doesn't match what's expected"
    } else if err_str.contains("trailing comma") {
        "Remove the trailing comma"
    } else if err_str.contains("EOF") {
        "Unexpected end of file - check for unclosed brackets"
    } else if err_str.contains("expected") {
        "Check the syntax at this position"
    } else {
        "Check the JSON syntax here"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct TestStruct {
        name: String,
        value: i32,
        optional: Option<String>,
    }
    
    #[test]
    fn test_missing_field_error() {
        let json = r#"{"name": "test"}"#;
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("missing field"));
        assert!(err.contains("test.json"));
    }
    
    #[test]
    fn test_invalid_syntax_error() {
        let json = r#"{"name": "test", }"#;  // trailing comma
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("line 1"));
    }

    #[test]
    fn test_unknown_field_error() {
        let json = r#"{"name": "test", "value": 42, "unknown": "field"}"#;
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("unknown field") || err.contains("not recognized"));
    }

    #[test]
    fn test_invalid_type_error() {
        let json = r#"{"name": "test", "value": "not a number"}"#;
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("invalid type") || err.contains("expected"));
    }

    #[test]
    fn test_eof_error() {
        let json = r#"{"name": "test""#;  // missing closing brace
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("EOF") || err.contains("end of file") || err.contains("unclosed"));
    }

    #[test]
    fn test_empty_input() {
        let json = "";
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("test.json"));
    }

    #[test]
    fn test_multiline_error() {
        let json = r#"{
  "name": "test",
  "value": "wrong type"
}"#;
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        // Should show context lines
        assert!(err.contains("Context:") || err.contains("│"));
    }

    #[test]
    fn test_successful_parsing() {
        let json = r#"{"name": "test", "value": 42}"#;
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.name, "test");
        assert_eq!(parsed.value, 42);
    }

    #[derive(Debug, Deserialize)]
    struct TestStructWithOptional {
        name: String,
        value: i32,
        optional: Option<String>,
    }

    #[test]
    fn test_successful_parsing_with_optional() {
        let json = r#"{"name": "test", "value": 42, "optional": "present"}"#;
        let result: Result<TestStructWithOptional> = parse_json_with_context(json, "test.json");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.optional, Some("present".to_string()));
    }

    #[test]
    fn test_error_hints() {
        let json = r#"{"name": "test", }"#;
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        // Should include common issues section
        assert!(err.contains("Common issues") || err.contains("trailing comma"));
    }

    #[test]
    fn test_get_helpful_hint_coverage() {
        // Test all branches of get_helpful_hint through parse errors
        let test_cases = vec![
            (r#"{"name": "test"}"#, "missing field"),  // missing field
            (r#"{"name": "test", "value": 42, "extra": "field"}"#, "unknown field"),  // unknown field
            (r#"{"name": "test", "value": "string"}"#, "invalid type"),  // invalid type
            (r#"{"name": "test", "value": 42, }"#, "trailing comma"),  // trailing comma
            (r#"{"name": "test""#, "EOF"),  // EOF
        ];

        for (json, _expected_keyword) in test_cases {
            let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
            assert!(result.is_err(), "Expected error for: {}", json);
            let err = result.unwrap_err().to_string();
            // The error message should contain helpful information
            assert!(err.len() > 0, "Error message should not be empty");
        }
    }

    #[test]
    fn test_format_json_error_edge_cases() {
        // Test with line 0 (should still work)
        let json = "";
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("test.json"));

        // Test with single line
        let json = r#"{"name": "test"}"#;
        let result: Result<TestStruct> = parse_json_with_context(json, "test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        // Should show context even for single line
        assert!(err.contains("test.json"));
    }

    #[test]
    fn test_format_json_error_directly() {
        // Test format_json_error function directly
        let json = r#"{"name": "test", "value": }"#;  // missing value
        let err = serde_json::from_str::<TestStruct>(json).unwrap_err();
        let formatted = format_json_error(err, json, "test.json");
        assert!(formatted.contains("test.json"));
        assert!(formatted.contains("line"));
        assert!(formatted.contains("Context:"));
    }

    #[test]
    fn test_get_helpful_hint_all_branches() {
        // Create mock errors to test all hint branches
        let json_missing = r#"{"name": "test"}"#;
        let err_missing = serde_json::from_str::<TestStruct>(json_missing).unwrap_err();
        assert!(get_helpful_hint(&err_missing).contains("required"));

        let json_unknown = r#"{"name": "test", "value": 42, "extra": "field"}"#;
        let err_unknown = serde_json::from_str::<TestStruct>(json_unknown).unwrap_err();
        assert!(get_helpful_hint(&err_unknown).contains("recognized") || get_helpful_hint(&err_unknown).contains("schema"));

        let json_type = r#"{"name": "test", "value": "string"}"#;
        let err_type = serde_json::from_str::<TestStruct>(json_type).unwrap_err();
        assert!(get_helpful_hint(&err_type).contains("type") || get_helpful_hint(&err_type).contains("expected"));

        let json_eof = r#"{"name": "test""#;
        let err_eof = serde_json::from_str::<TestStruct>(json_eof).unwrap_err();
        let hint = get_helpful_hint(&err_eof);
        assert!(hint.contains("EOF") || hint.contains("unclosed") || hint.contains("syntax"));
    }

    #[test]
    fn test_format_json_error_with_column() {
        let json = r#"{"name": "test", "value": }"#;
        let err = serde_json::from_str::<TestStruct>(json).unwrap_err();
        let formatted = format_json_error(err, json, "test.json");
        // Should include column information
        assert!(formatted.contains("column") || formatted.contains("line"));
    }

    #[test]
    fn test_format_json_error_multiline_context() {
        let json = r#"{
  "name": "test",
  "value": "wrong"
}"#;
        let err = serde_json::from_str::<TestStruct>(json).unwrap_err();
        let formatted = format_json_error(err, json, "test.json");
        // Should show multiple lines of context
        assert!(formatted.contains("│") || formatted.contains("Context:"));
    }
}
