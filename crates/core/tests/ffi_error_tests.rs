// Tests for FFI error handling and mapping

use slvsx_core::error::Error;
use slvsx_core::ffi::FfiError;
use slvsx_core::ir::{Constraint, Entity, ExprOrNumber, InputDocument};
use slvsx_core::solver::{Solver, SolverConfig};
use std::collections::HashMap;

/// Helper function to test error mapping from FfiError to Error
fn test_error_mapping(ffi_error: FfiError, expected_error: Error) {
    let mapped = Solver::map_ffi_error(ffi_error);

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
        _ => panic!("Error types don't match: {:?} vs {:?}", mapped, expected_error),
    }
}

#[test]
fn test_ffi_error_mapping_inconsistent() {
    test_error_mapping(FfiError::Inconsistent, Error::Overconstrained);
}

#[test]
fn test_ffi_error_mapping_didnt_converge() {
    test_error_mapping(
        FfiError::DidntConverge,
        Error::SolverConvergence { iterations: 100 },
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
        Error::Ffi("Invalid solver system".to_string()),
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
            },
            Entity::Point {
                id: "p2".to_string(),
                at: vec![
                    ExprOrNumber::Number(10.0),
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
            },
        ],
        constraints: vec![
            Constraint::Fixed {
                entity: "p1".to_string(),
            },
            Constraint::Fixed {
                entity: "p2".to_string(),
            },
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

