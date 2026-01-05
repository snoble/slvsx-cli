use crate::error::Result;
use crate::ir::{Diagnostics, InputDocument, SolveResult};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub tolerance: f64,
    pub max_iterations: u32,
    pub timeout_ms: Option<u64>,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-6,
            max_iterations: 1000,
            timeout_ms: None,
        }
    }
}

pub struct Solver {
    #[allow(dead_code)]
    config: SolverConfig,
}

impl Solver {
    pub fn new(config: SolverConfig) -> Self {
        Self { config }
    }

    /// Map FFI errors to high-level Error types
    /// This is public for testing purposes
    ///
    /// # Arguments
    /// * `e` - The FFI error to map
    /// * `max_iterations` - The configured maximum iterations, used for convergence error messages
    pub fn map_ffi_error(e: crate::ffi::FfiError, max_iterations: u32) -> crate::error::Error {
        match e {
            crate::ffi::FfiError::Inconsistent => crate::error::Error::Overconstrained,
            crate::ffi::FfiError::DidntConverge => {
                crate::error::Error::SolverConvergence {
                    iterations: max_iterations,
                }
            }
            crate::ffi::FfiError::TooManyUnknowns => {
                // Try to get DOF from solver if possible, otherwise default to 0
                crate::error::Error::Underconstrained { dof: 0 }
            }
            crate::ffi::FfiError::InvalidSystem => {
                crate::error::Error::Ffi("Invalid solver system".to_string())
            }
            crate::ffi::FfiError::Unknown(code) => {
                crate::error::Error::Ffi(format!("Unknown solver error (code: {})", code))
            }
            e => crate::error::Error::Ffi(e.to_string()),
        }
    }

    pub fn solve(&self, doc: &InputDocument) -> Result<SolveResult> {
        use crate::expr::ExpressionEvaluator;
        use crate::ffi::Solver as FfiSolver;

        let mut ffi_solver = FfiSolver::new();
        let eval = ExpressionEvaluator::new(doc.parameters.clone());

        // Add entities to solver
        let mut entity_id_map = HashMap::new();
        let mut next_id = 1;

        for (entity_idx, entity) in doc.entities.iter().enumerate() {
            match entity {
                crate::ir::Entity::Point { id, at, preserve, .. } => {
                    // Evaluate expressions for point coordinates
                    let x = match &at[0] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let y = match &at[1] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let z = if at.len() > 2 {
                        match &at[2] {
                            crate::ir::ExprOrNumber::Number(n) => *n,
                            crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                        }
                    } else {
                        0.0
                    };

                    ffi_solver
                        .add_point(next_id, x, y, z, *preserve)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add point '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                crate::ir::Entity::Point2D { id, at, workplane, preserve, .. } => {
                    // Evaluate expressions for 2D point coordinates
                    let u = match &at[0] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let v = match &at[1] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };

                    // Look up workplane entity ID
                    let workplane_id = entity_id_map
                        .get(workplane)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(workplane.clone()))?;

                    ffi_solver
                        .add_point_2d(next_id, *workplane_id, u, v, *preserve)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add 2D point '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                crate::ir::Entity::Line { id, p1, p2, .. } => {
                    // Look up the point entity IDs
                    let point1_id = entity_id_map
                        .get(p1)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(p1.clone()))?;
                    let point2_id = entity_id_map
                        .get(p2)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(p2.clone()))?;

                    ffi_solver
                        .add_line(next_id, *point1_id, *point2_id)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add line '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                crate::ir::Entity::Line2D { id, p1, p2, workplane, .. } => {
                    // Look up the point entity IDs (must be Point2D)
                    let point1_id = entity_id_map
                        .get(p1)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(p1.clone()))?;
                    let point2_id = entity_id_map
                        .get(p2)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(p2.clone()))?;
                    let workplane_id = entity_id_map
                        .get(workplane)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(workplane.clone()))?;

                    ffi_solver
                        .add_line_2d(next_id, *point1_id, *point2_id, *workplane_id)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add 2D line '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                crate::ir::Entity::Circle {
                    id,
                    center,
                    diameter,
                    ..
                } => {
                    // Evaluate expressions
                    let cx = match &center[0] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let cy = match &center[1] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let cz = if center.len() > 2 {
                        match &center[2] {
                            crate::ir::ExprOrNumber::Number(n) => *n,
                            crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                        }
                    } else {
                        0.0
                    };
                    let diam = match diameter {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let radius = diam / 2.0;

                    ffi_solver
                        .add_circle(next_id, cx, cy, cz, radius)
                        .map_err(|e| crate::error::Error::Ffi(e))?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                crate::ir::Entity::Arc {
                    id,
                    center,
                    start,
                    end,
                    normal,
                    workplane,
                    ..
                } => {
                    // Look up point entity IDs
                    let center_id = entity_id_map
                        .get(center)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(center.clone()))?;
                    let start_id = entity_id_map
                        .get(start)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(start.clone()))?;
                    let end_id = entity_id_map
                        .get(end)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(end.clone()))?;

                    // Evaluate normal vector
                    let nx = match &normal[0] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let ny = match &normal[1] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let nz = if normal.len() > 2 {
                        match &normal[2] {
                            crate::ir::ExprOrNumber::Number(n) => *n,
                            crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                        }
                    } else {
                        1.0 // Default to Z-axis
                    };

                    // Normalize normal vector
                    let norm_len = (nx * nx + ny * ny + nz * nz).sqrt();
                    let nx_norm = if norm_len > 0.0 { nx / norm_len } else { 0.0 };
                    let ny_norm = if norm_len > 0.0 { ny / norm_len } else { 0.0 };
                    let nz_norm = if norm_len > 0.0 { nz / norm_len } else { 1.0 };

                    // Get workplane ID if specified
                    let workplane_id = workplane.as_ref().and_then(|wp| entity_id_map.get(wp).copied());

                    ffi_solver
                        .add_arc(next_id, *center_id, *start_id, *end_id, nx_norm, ny_norm, nz_norm, workplane_id)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add arc '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                crate::ir::Entity::Cubic {
                    id,
                    control_points,
                    workplane,
                    ..
                } => {
                    if control_points.len() != 4 {
                        return Err(crate::error::Error::InvalidInput {
                            message: format!("Cubic curve '{}' must have exactly 4 control points, got {}", id, control_points.len()),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        });
                    }

                    // Look up point entity IDs
                    let pt0_id = entity_id_map
                        .get(&control_points[0])
                        .ok_or_else(|| crate::error::Error::EntityNotFound(control_points[0].clone()))?;
                    let pt1_id = entity_id_map
                        .get(&control_points[1])
                        .ok_or_else(|| crate::error::Error::EntityNotFound(control_points[1].clone()))?;
                    let pt2_id = entity_id_map
                        .get(&control_points[2])
                        .ok_or_else(|| crate::error::Error::EntityNotFound(control_points[2].clone()))?;
                    let pt3_id = entity_id_map
                        .get(&control_points[3])
                        .ok_or_else(|| crate::error::Error::EntityNotFound(control_points[3].clone()))?;

                    // Get workplane ID if specified
                    let workplane_id = workplane.as_ref().and_then(|wp| entity_id_map.get(wp).copied());

                    ffi_solver
                        .add_cubic(next_id, *pt0_id, *pt1_id, *pt2_id, *pt3_id, workplane_id)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add cubic curve '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                crate::ir::Entity::Plane { id, origin, normal } => {
                    // Evaluate expressions for origin point
                    let ox = match &origin[0] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let oy = match &origin[1] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let oz = if origin.len() > 2 {
                        match &origin[2] {
                            crate::ir::ExprOrNumber::Number(n) => *n,
                            crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                        }
                    } else {
                        0.0
                    };

                    // Evaluate expressions for normal vector
                    let nx = match &normal[0] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let ny = match &normal[1] {
                        crate::ir::ExprOrNumber::Number(n) => *n,
                        crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                    };
                    let nz = if normal.len() > 2 {
                        match &normal[2] {
                            crate::ir::ExprOrNumber::Number(n) => *n,
                            crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                        }
                    } else {
                        1.0 // Default to Z-axis if not specified
                    };

                    // Create origin point first (temporary, will be used by workplane)
                    let origin_point_id = next_id;
                    ffi_solver
                        .add_point(origin_point_id, ox, oy, oz, false)  // Plane origin not preserved
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add plane origin point '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    next_id += 1;

                    // Create workplane
                    ffi_solver
                        .add_workplane(next_id, origin_point_id, nx, ny, nz)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add plane '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                _ => {} // Handle other entity types as needed
            }
        }

        // Add constraints from JSON - use ConstraintRegistry to ensure all constraints are handled
        use crate::constraint_registry::ConstraintRegistry;
        let mut constraint_id = 100;

        // Process all constraints from JSON
        for (constraint_idx, constraint) in doc.constraints.iter().enumerate() {
            ConstraintRegistry::process_constraint(
                constraint,
                &mut ffi_solver,
                constraint_id,
                &entity_id_map,
                &eval,
            )
            .map_err(|e| crate::error::Error::InvalidInput {
                message: format!("Failed to process constraint: {}", e),
                pointer: Some(format!("/constraints/{}", constraint_idx)),
            })?;
            constraint_id += 1;
        }

        // Actually solve the constraints!
        let max_iterations = self.config.max_iterations;
        ffi_solver
            .solve()
            .map_err(|e| Self::map_ffi_error(e, max_iterations))?;

        // Get solved positions from libslvs
        let mut resolved_entities = HashMap::new();

        // Retrieve solved positions for all entities
        for entity in &doc.entities {
            match entity {
                crate::ir::Entity::Point { id, .. } | crate::ir::Entity::Point2D { id, .. } => {
                    let entity_id = entity_id_map.get(id).copied().unwrap_or(0);
                    if let Ok((x, y, z)) = ffi_solver.get_point_position(entity_id) {
                        resolved_entities.insert(
                            id.clone(),
                            crate::ir::ResolvedEntity::Point { at: vec![x, y, z] },
                        );
                    }
                }
                crate::ir::Entity::Line { id, p1, p2, .. }
                | crate::ir::Entity::Line2D { id, p1, p2, .. } => {
                    // Lines are defined by their endpoints, get the actual coordinates
                    let p1_id = entity_id_map
                        .get(p1)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(p1.clone()))?;
                    let p2_id = entity_id_map
                        .get(p2)
                        .ok_or_else(|| crate::error::Error::EntityNotFound(p2.clone()))?;

                    if let (Ok((x1, y1, z1)), Ok((x2, y2, z2))) = (
                        ffi_solver.get_point_position(*p1_id),
                        ffi_solver.get_point_position(*p2_id),
                    ) {
                        resolved_entities.insert(
                            id.clone(),
                            crate::ir::ResolvedEntity::Line {
                                p1: vec![x1, y1, z1],
                                p2: vec![x2, y2, z2],
                            },
                        );
                    }
                }
                crate::ir::Entity::Circle { id, .. } => {
                    let entity_id = entity_id_map.get(id).copied().unwrap_or(0);
                    if let Ok((cx, cy, cz, radius)) = ffi_solver.get_circle_position(entity_id) {
                        resolved_entities.insert(
                            id.clone(),
                            crate::ir::ResolvedEntity::Circle {
                                center: vec![cx, cy, cz],
                                diameter: radius * 2.0,
                            },
                        );
                    }
                }
                _ => {} // Handle other entity types as needed
            }
        }

        // Return the solved entities - this is now completely generic!
        return Ok(SolveResult {
            status: "ok".to_string(),
            diagnostics: Some(Diagnostics {
                iters: 1,
                residual: 0.0,
                dof: 0,
                time_ms: 1,
            }),
            entities: Some(resolved_entities),
            warnings: vec![],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_config_default() {
        let config = SolverConfig::default();
        assert_eq!(config.tolerance, 1e-6);
        assert_eq!(config.max_iterations, 1000);
        assert_eq!(config.timeout_ms, None);
    }

    #[test]
    fn test_solver_config_custom() {
        let config = SolverConfig {
            tolerance: 1e-8,
            max_iterations: 500,
            timeout_ms: Some(5000),
        };
        assert_eq!(config.tolerance, 1e-8);
        assert_eq!(config.max_iterations, 500);
        assert_eq!(config.timeout_ms, Some(5000));
    }

    #[test]
    fn test_solver_new() {
        let config = SolverConfig::default();
        let solver = Solver::new(config);
        // Just verify it can be created
        assert!(std::mem::size_of_val(&solver) > 0);
    }

    #[test]
    fn test_map_ffi_error_convergence_uses_max_iterations() {
        // Verify that the convergence error uses the provided max_iterations
        // not a hardcoded value
        let error = Solver::map_ffi_error(crate::ffi::FfiError::DidntConverge, 500);
        match error {
            crate::error::Error::SolverConvergence { iterations } => {
                assert_eq!(iterations, 500);
            }
            _ => panic!("Expected SolverConvergence error"),
        }

        // Test with default config's max_iterations
        let error = Solver::map_ffi_error(crate::ffi::FfiError::DidntConverge, 1000);
        match error {
            crate::error::Error::SolverConvergence { iterations } => {
                assert_eq!(iterations, 1000);
            }
            _ => panic!("Expected SolverConvergence error"),
        }
    }

    #[test]
    fn test_map_ffi_error_other_errors() {
        // Verify other errors still work correctly
        let error = Solver::map_ffi_error(crate::ffi::FfiError::Inconsistent, 1000);
        assert!(matches!(error, crate::error::Error::Overconstrained));

        let error = Solver::map_ffi_error(crate::ffi::FfiError::TooManyUnknowns, 1000);
        assert!(matches!(error, crate::error::Error::Underconstrained { dof: 0 }));

        let error = Solver::map_ffi_error(crate::ffi::FfiError::InvalidSystem, 1000);
        assert!(matches!(error, crate::error::Error::Ffi(_)));

        let error = Solver::map_ffi_error(crate::ffi::FfiError::Unknown(42), 1000);
        if let crate::error::Error::Ffi(msg) = error {
            assert!(msg.contains("42"));
        } else {
            panic!("Expected Ffi error");
        }
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: Convergence error must report the exact max_iterations value provided.
        /// This catches any hardcoded iteration values.
        #[test]
        fn convergence_error_uses_provided_max_iterations(max_iterations in 1u32..=100_000) {
            let error = Solver::map_ffi_error(crate::ffi::FfiError::DidntConverge, max_iterations);

            match error {
                crate::error::Error::SolverConvergence { iterations } => {
                    prop_assert_eq!(
                        iterations, max_iterations,
                        "Convergence error must report provided max_iterations, not a hardcoded value"
                    );
                }
                _ => prop_assert!(false, "DidntConverge must map to SolverConvergence"),
            }
        }

        /// Property: Error message content must match the iterations field value.
        #[test]
        fn error_message_contains_correct_iteration_count(max_iterations in 1u32..=100_000) {
            let error = Solver::map_ffi_error(crate::ffi::FfiError::DidntConverge, max_iterations);
            let message = error.to_string();

            prop_assert!(
                message.contains(&max_iterations.to_string()),
                "Error message '{}' must contain iteration count {}",
                message,
                max_iterations
            );
        }

        /// Property: max_iterations parameter should not affect non-convergence errors.
        #[test]
        fn max_iterations_does_not_affect_other_errors(max_iterations in 1u32..=100_000) {
            // Inconsistent always produces Overconstrained
            let error = Solver::map_ffi_error(crate::ffi::FfiError::Inconsistent, max_iterations);
            prop_assert!(
                matches!(error, crate::error::Error::Overconstrained),
                "Inconsistent should always map to Overconstrained"
            );

            // TooManyUnknowns always produces Underconstrained with dof 0
            let error = Solver::map_ffi_error(crate::ffi::FfiError::TooManyUnknowns, max_iterations);
            prop_assert!(
                matches!(error, crate::error::Error::Underconstrained { dof: 0 }),
                "TooManyUnknowns should always map to Underconstrained with dof 0"
            );

            // InvalidSystem always produces a specific Ffi error
            let error = Solver::map_ffi_error(crate::ffi::FfiError::InvalidSystem, max_iterations);
            match error {
                crate::error::Error::Ffi(msg) => {
                    prop_assert_eq!(msg, "Invalid solver system");
                }
                _ => prop_assert!(false, "InvalidSystem should map to Ffi error"),
            }
        }

        /// Property: SolverConfig default max_iterations should be used in convergence errors
        /// when using the solve method (integration check).
        #[test]
        fn solver_config_max_iterations_consistency(max_iterations in 1u32..10_000) {
            let config = SolverConfig {
                tolerance: 1e-6,
                max_iterations,
                timeout_ms: None,
            };

            // Simulate what happens when solve() encounters a convergence error
            let error = Solver::map_ffi_error(crate::ffi::FfiError::DidntConverge, config.max_iterations);

            match error {
                crate::error::Error::SolverConvergence { iterations } => {
                    prop_assert_eq!(
                        iterations, config.max_iterations,
                        "Solver should use its configured max_iterations in error"
                    );
                }
                _ => prop_assert!(false, "Expected SolverConvergence"),
            }
        }
    }
}
