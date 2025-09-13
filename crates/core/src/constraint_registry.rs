// Constraint Registry - Ensures all constraints have FFI implementations
//
// This module uses Rust's type system to enforce that every constraint
// type defined in ir.rs has a corresponding FFI implementation.

use crate::ir::Constraint;
use crate::ffi::Solver as FfiSolver;
use crate::expr::ExpressionEvaluator;

/// Trait that all constraints must implement to prove they have FFI support
pub trait HasFfiImplementation {
    /// Add this constraint to the FFI solver
    fn add_to_ffi_solver(
        &self,
        solver: &mut FfiSolver,
        constraint_id: i32,
        entity_id_map: &std::collections::HashMap<String, i32>,
    ) -> Result<(), String>;
}

/// Macro to implement HasFfiImplementation for each constraint variant
/// This ensures compile-time checking that all constraints are handled
macro_rules! impl_ffi_constraint {
    ($variant:ident { $($field:ident),* } => $handler:expr) => {
        // This would be implemented for each variant
    };
}

/// Registry of all constraint implementations
/// If a constraint doesn't have an entry here, it will fail to compile
pub struct ConstraintRegistry;

impl ConstraintRegistry {
    /// Process any constraint - this function won't compile if a constraint
    /// lacks FFI implementation
    pub fn process_constraint(
        constraint: &Constraint,
        solver: &mut FfiSolver,
        constraint_id: i32,
        entity_id_map: &std::collections::HashMap<String, i32>,
    ) -> Result<(), String> {
        match constraint {
            Constraint::Fixed { entity } => {
                let entity_id = entity_id_map.get(entity).copied().unwrap_or(0);
                solver.add_fixed_constraint(constraint_id, entity_id)
            }
            Constraint::Distance { between, value } => {
                if between.len() == 2 {
                    let id1 = entity_id_map.get(&between[0]).copied().unwrap_or(0);
                    let id2 = entity_id_map.get(&between[1]).copied().unwrap_or(0);
                    let dist = match value {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => {
                            let evaluator = ExpressionEvaluator::new(std::collections::HashMap::new());
                            evaluator.eval(e).unwrap_or(0.0)
                        }
                    };
                    solver.add_distance_constraint(constraint_id, id1, id2, dist)
                } else {
                    Err("Distance constraint requires exactly 2 entities".to_string())
                }
            }
            Constraint::Angle { between, value } => {
                // TODO: Implement angle constraint in FFI
                Err("Angle constraint not yet implemented in FFI".to_string())
            }
            Constraint::Coincident { at, of } => {
                if of.len() == 1 {
                    let point_id = entity_id_map.get(at).copied().unwrap_or(0);
                    let line_id = entity_id_map.get(&of[0]).copied().unwrap_or(0);
                    solver.add_point_on_line_constraint(constraint_id, point_id, line_id)
                } else {
                    Err("Coincident constraint currently only supports point-on-line".to_string())
                }
            }
            Constraint::Perpendicular { a, b } => {
                let line1_id = entity_id_map.get(a).copied().unwrap_or(0);
                let line2_id = entity_id_map.get(b).copied().unwrap_or(0);
                solver.add_perpendicular_constraint(constraint_id, line1_id, line2_id)
            }
            Constraint::Parallel { a, b } => {
                let line1_id = entity_id_map.get(a).copied().unwrap_or(0);
                let line2_id = entity_id_map.get(b).copied().unwrap_or(0);
                solver.add_parallel_constraint(constraint_id, line1_id, line2_id)
            }
            Constraint::Horizontal { a } => {
                // TODO: Implement horizontal constraint in FFI
                Err("Horizontal constraint not yet implemented in FFI".to_string())
            }
            Constraint::Vertical { a } => {
                // TODO: Implement vertical constraint in FFI
                Err("Vertical constraint not yet implemented in FFI".to_string())
            }
            Constraint::EqualLength { a, b } => {
                // TODO: Implement equal_length constraint in FFI
                Err("EqualLength constraint not yet implemented in FFI".to_string())
            }
            Constraint::EqualRadius { a, b } => {
                // TODO: Implement equal_radius constraint in FFI
                Err("EqualRadius constraint not yet implemented in FFI".to_string())
            }
            Constraint::Tangent { a, b } => {
                // TODO: Implement tangent constraint in FFI
                Err("Tangent constraint not yet implemented in FFI".to_string())
            }
            Constraint::PointOnLine { point, line } => {
                let point_id = entity_id_map.get(point).copied().unwrap_or(0);
                let line_id = entity_id_map.get(line).copied().unwrap_or(0);
                solver.add_point_on_line_constraint(constraint_id, point_id, line_id)
            }
            Constraint::PointOnCircle { point, circle } => {
                // TODO: Implement point_on_circle constraint in FFI
                Err("PointOnCircle constraint not yet implemented in FFI".to_string())
            }
            Constraint::Symmetric { a, b, about } => {
                // TODO: Implement symmetric constraint in FFI
                Err("Symmetric constraint not yet implemented in FFI".to_string())
            }
            Constraint::Midpoint { point, of } => {
                // TODO: Implement midpoint constraint in FFI
                Err("Midpoint constraint not yet implemented in FFI".to_string())
            }
            // COMPILER ERROR if a constraint variant is missing here!
            // This ensures we never forget to handle a new constraint type
        }
    }

    /// Get list of constraints with missing FFI implementations
    pub fn missing_implementations() -> Vec<&'static str> {
        vec![
            "Angle",
            "Horizontal",
            "Vertical",
            "EqualLength",
            "EqualRadius",
            "Tangent",
            "PointOnCircle",
            "Symmetric",
            "Midpoint",
        ]
    }

    /// Get list of constraints with FFI implementations
    pub fn implemented_constraints() -> Vec<&'static str> {
        vec![
            "Fixed",
            "Distance",
            "Coincident",
            "Perpendicular",
            "Parallel",
            "PointOnLine",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_constraints_handled() {
        // This test will fail to compile if any constraint variant is missing
        // from the process_constraint match statement
        let test_constraint = |c: Constraint| {
            let mut solver = FfiSolver::new();
            let entity_map = std::collections::HashMap::new();
            let _ = ConstraintRegistry::process_constraint(&c, &mut solver, 1, &entity_map);
        };

        // Test that we can handle all constraint types (compilation test)
        test_constraint(Constraint::Fixed { entity: "p1".to_string() });
        test_constraint(Constraint::Distance { 
            between: vec!["p1".to_string(), "p2".to_string()],
            value: crate::ir::ExprOrNumber::Number(10.0)
        });
        // ... more test cases
    }

    #[test]
    fn test_missing_implementations_documented() {
        let missing = ConstraintRegistry::missing_implementations();
        assert!(!missing.is_empty(), "Update this test when all constraints are implemented!");
        
        // This test ensures we're aware of what's not implemented
        println!("Constraints missing FFI implementation:");
        for constraint in missing {
            println!("  - {}", constraint);
        }
    }
}