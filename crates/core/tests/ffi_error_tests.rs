// Tests for FFI error handling and mapping

use slvsx_core::error::Error;
use slvsx_core::ffi::FfiError;
use slvsx_core::ir::{Constraint, Entity, ExprOrNumber, InputDocument};
use slvsx_core::solver::{Solver, SolverConfig};
use std::collections::HashMap;

/// Default max_iterations value for testing error mapping
const DEFAULT_MAX_ITERATIONS: u32 = 1000;

/// Helper function to test error mapping from FfiError to Error
fn test_error_mapping(ffi_error: FfiError, expected_error: Error) {
    test_error_mapping_with_iterations(ffi_error, expected_error, DEFAULT_MAX_ITERATIONS);
}

/// Helper function to test error mapping with a specific max_iterations value
fn test_error_mapping_with_iterations(ffi_error: FfiError, expected_error: Error, max_iterations: u32) {
    let mapped = Solver::map_ffi_error(ffi_error, max_iterations);

    match (&mapped, &expected_error) {
        (Error::Overconstrained, Error::Overconstrained) => {}
        (Error::SolverConvergence { iterations: a }, Error::SolverConvergence { iterations: b }) => {
            assert_eq!(a, b);
        }
        (Error::Underconstrained { dof: a }, Error::Underconstrained { dof: b }) => {
            assert_eq!(a, b);
        }
        (Error::Ffi(a), Error::Ffi(b)) => {
            assert_eq!(a, b);
        }
        (Error::InvalidSystem, Error::InvalidSystem) => {}
        _ => panic!("Error types don't match: {:?} vs {:?}", mapped, expected_error),
    }
}

#[test]
fn test_ffi_error_mapping_inconsistent() {
    test_error_mapping(FfiError::Inconsistent, Error::Overconstrained);
}

#[test]
fn test_ffi_error_mapping_didnt_converge() {
    // With default max_iterations (1000), convergence error should report 1000 iterations
    test_error_mapping(
        FfiError::DidntConverge,
        Error::SolverConvergence { iterations: DEFAULT_MAX_ITERATIONS },
    );
}

#[test]
fn test_ffi_error_mapping_didnt_converge_custom_iterations() {
    // Test that custom max_iterations values are correctly reflected in the error
    let custom_iterations = 500;
    test_error_mapping_with_iterations(
        FfiError::DidntConverge,
        Error::SolverConvergence { iterations: custom_iterations },
        custom_iterations,
    );
}

#[test]
fn test_ffi_error_mapping_too_many_unknowns() {
    test_error_mapping(FfiError::TooManyUnknowns, Error::Underconstrained { dof: 0 });
}

#[test]
fn test_ffi_error_mapping_invalid_system() {
    test_error_mapping(
        FfiError::InvalidSystem,
        Error::InvalidSystem,
    );
}

#[test]
fn test_ffi_error_mapping_unknown() {
    test_error_mapping(
        FfiError::Unknown(99),
        Error::Ffi("Unknown solver error (code: 99)".to_string()),
    );
}

#[test]
fn test_ffi_error_mapping_entity_not_found() {
    test_error_mapping(
        FfiError::EntityNotFound("p1".to_string()),
        Error::Ffi("Entity not found: p1".to_string()),
    );
}

#[test]
fn test_ffi_error_mapping_constraint_failed() {
    test_error_mapping(
        FfiError::ConstraintFailed("test constraint".to_string()),
        Error::Ffi("Constraint operation failed: test constraint".to_string()),
    );
}

// Property-based tests for error mapping consistency
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: For any max_iterations value, the convergence error should report
        /// exactly that iteration count. This ensures we never use hardcoded values.
        #[test]
        fn convergence_error_reports_configured_iterations(max_iterations in 1u32..=100_000) {
            let error = Solver::map_ffi_error(FfiError::DidntConverge, max_iterations);

            match error {
                Error::SolverConvergence { iterations } => {
                    prop_assert_eq!(
                        iterations, max_iterations,
                        "Convergence error should report the configured max_iterations ({}), not a hardcoded value",
                        max_iterations
                    );
                }
                _ => prop_assert!(false, "DidntConverge should map to SolverConvergence error"),
            }
        }

        /// Property: The iteration count in the error message should always equal
        /// the iteration count in the error struct.
        #[test]
        fn convergence_error_message_matches_iterations(max_iterations in 1u32..=100_000) {
            let error = Solver::map_ffi_error(FfiError::DidntConverge, max_iterations);

            let error_message = error.to_string();
            let expected_message = format!("Solver failed to converge after {} iterations", max_iterations);
            prop_assert_eq!(
                error_message, expected_message,
                "Error message should contain the correct iteration count"
            );
        }

        /// Property: Error mapping should be deterministic - same input always produces same output
        #[test]
        fn error_mapping_is_deterministic(max_iterations in 1u32..=100_000) {
            let error1 = Solver::map_ffi_error(FfiError::DidntConverge, max_iterations);
            let error2 = Solver::map_ffi_error(FfiError::DidntConverge, max_iterations);

            prop_assert_eq!(
                error1.to_string(), error2.to_string(),
                "Error mapping should be deterministic"
            );
        }

        /// Property: Overconstrained errors should not be affected by max_iterations
        #[test]
        fn overconstrained_error_independent_of_iterations(max_iterations in 1u32..=100_000) {
            let error = Solver::map_ffi_error(FfiError::Inconsistent, max_iterations);

            match error {
                Error::Overconstrained => {} // Expected
                _ => prop_assert!(false, "Inconsistent should always map to Overconstrained"),
            }
        }

        /// Property: Underconstrained errors should not be affected by max_iterations
        #[test]
        fn underconstrained_error_independent_of_iterations(max_iterations in 1u32..=100_000) {
            let error = Solver::map_ffi_error(FfiError::TooManyUnknowns, max_iterations);

            match error {
                Error::Underconstrained { dof: 0 } => {} // Expected
                _ => prop_assert!(false, "TooManyUnknowns should always map to Underconstrained with dof 0"),
            }
        }

        /// Property: Unknown error codes should preserve the code in the message
        #[test]
        fn unknown_error_preserves_code(code in -1000i32..=1000, max_iterations in 1u32..=100_000) {
            // Skip codes that map to known errors
            prop_assume!(code != 0 && code != 1 && code != 2 && code != 3 && code != -1);

            let error = Solver::map_ffi_error(FfiError::Unknown(code), max_iterations);

            match error {
                Error::Ffi(msg) => {
                    prop_assert!(
                        msg.contains(&code.to_string()),
                        "Unknown error message should contain the error code"
                    );
                }
                _ => prop_assert!(false, "Unknown should map to Ffi error"),
            }
        }
    }
}

/// Test that overconstrained systems return Overconstrained error
#[test]
fn test_overconstrained_error() {
    // Create an overconstrained system (two conflicting distance constraints)
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: HashMap::new(),
        entities: vec![
            Entity::Point {
                id: "p1".to_string(),
                at: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![
                    ExprOrNumber::Number(10.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                construction: false,
                preserve: false,
            },
        ],
        constraints: vec![
            Constraint::Fixed { entity: "p1".to_string(), workplane: None },
            Constraint::Fixed { entity: "p2".to_string(), workplane: None },
            Constraint::Distance {
                between: vec!["p1".to_string(), "p2".to_string()],
                value: ExprOrNumber::Number(50.0), // Conflicting with fixed positions
            },
        ],
    };

    let solver = Solver::new(SolverConfig::default());
    let result = solver.solve(&doc);

    // Should fail with Overconstrained error
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::Overconstrained => {} // Expected
        e => panic!("Expected Overconstrained, got {:?}", e),
    }
}

