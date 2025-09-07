pub mod error;
pub mod expr;
pub mod ir;
pub mod schema_validator;
pub mod solver;
pub mod translator;
pub mod validator;

pub mod ffi;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use error::{Error, Result};
pub use ir::{Constraint, Entity, InputDocument, Parameter, SolveResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify that core types are exported
        let _ = std::mem::size_of::<Error>();
        let _ = std::mem::size_of::<Entity>();
        let _ = std::mem::size_of::<Constraint>();
    }
}
