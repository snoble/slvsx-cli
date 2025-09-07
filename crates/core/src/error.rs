use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid input: {message}")]
    InvalidInput { message: String, pointer: Option<String> },
    
    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),
    
    #[error("Expression evaluation error: {0}")]
    ExpressionEval(String),
    
    #[error("Solver failed to converge after {iterations} iterations")]
    SolverConvergence { iterations: u32 },
    
    #[error("System is overconstrained")]
    Overconstrained,
    
    #[error("System is underconstrained (DOF: {dof})")]
    Underconstrained { dof: u32 },
    
    #[error("Entity not found: {0}")]
    EntityNotFound(String),
    
    #[error("FFI error: {0}")]
    Ffi(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[cfg(feature = "yaml")]
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::InvalidInput { .. } | Error::SchemaValidation(_) => 2,
            Error::SolverConvergence { .. } => 3,
            Error::Overconstrained => 4,
            Error::Underconstrained { .. } => 5,
            Error::Ffi(_) => 6,
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_codes() {
        assert_eq!(Error::InvalidInput { message: "test".into(), pointer: None }.exit_code(), 2);
        assert_eq!(Error::SchemaValidation("test".into()).exit_code(), 2);
        assert_eq!(Error::SolverConvergence { iterations: 100 }.exit_code(), 3);
        assert_eq!(Error::Overconstrained.exit_code(), 4);
        assert_eq!(Error::Underconstrained { dof: 3 }.exit_code(), 5);
        assert_eq!(Error::Ffi("test".into()).exit_code(), 6);
        assert_eq!(Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test")).exit_code(), 1);
        assert_eq!(Error::Json(serde_json::from_str::<String>("invalid").unwrap_err()).exit_code(), 1);
    }
    
    #[test]
    fn test_error_display() {
        let err = Error::InvalidInput { message: "bad input".into(), pointer: Some("/entities/0".into()) };
        assert_eq!(err.to_string(), "Invalid input: bad input");
        
        let err = Error::SolverConvergence { iterations: 42 };
        assert_eq!(err.to_string(), "Solver failed to converge after 42 iterations");
        
        let err = Error::Underconstrained { dof: 2 };
        assert_eq!(err.to_string(), "System is underconstrained (DOF: 2)");
    }
    
    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert_eq!(err.exit_code(), 1);
    }
    
    #[test]
    fn test_from_json_error() {
        let json_err = serde_json::from_str::<String>("invalid").unwrap_err();
        let err: Error = json_err.into();
        assert_eq!(err.exit_code(), 1);
    }
    
    #[cfg(feature = "yaml")]
    #[test]
    fn test_from_yaml_error() {
        let yaml_err = serde_yaml::from_str::<String>("invalid: [").unwrap_err();
        let err: Error = yaml_err.into();
        assert_eq!(err.exit_code(), 1);
    }
}