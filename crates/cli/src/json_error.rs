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
fn format_json_error(err: JsonError, input: &str, filename: &str) -> String {
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
fn get_helpful_hint(err: &JsonError) -> &str {
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
    struct TestStruct {
        name: String,
        value: i32,
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
}