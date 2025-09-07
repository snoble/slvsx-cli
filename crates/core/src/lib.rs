pub mod constraint_generator;
pub mod distance_validator;
pub mod error;
pub mod expr;
pub mod ir;
pub mod mesh_validator;
pub mod phase_calculator;
pub mod phase_validator;
pub mod planetary_validator;
pub mod schema_validator;
pub mod solver;
pub mod translator;
pub mod validator;

#[cfg(not(feature = "mock-solver"))]
pub mod ffi;

pub use error::{Error, Result};
pub use ir::{Constraint, Entity, InputDocument, Parameter};

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

#[cfg(test)]
#[path = "ring_overlap_test.rs"]
mod ring_overlap_test;

#[cfg(test)]
#[path = "planetary_geometry_test.rs"]
mod planetary_geometry_test;