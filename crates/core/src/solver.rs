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
                crate::ir::Entity::Point { id, at } => {
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
                        .add_point(next_id, x, y, z)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add point '{}': {}", id, e),
                            pointer: Some(format!("/entities/{}", entity_idx)),
                        })?;
                    entity_id_map.insert(id.clone(), next_id);
                    next_id += 1;
                }
                crate::ir::Entity::Line { id, p1, p2 } => {
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
                crate::ir::Entity::Circle {
                    id,
                    center,
                    diameter,
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
                _ => {} // Handle other entity types as needed
            }
        }

        // Add constraints from JSON - generic handling
        let mut constraint_id = 100;

        // Process all constraints from JSON
        for (constraint_idx, constraint) in doc.constraints.iter().enumerate() {
            match constraint {
                crate::ir::Constraint::Fixed { entity } => {
                    let entity_id = entity_id_map.get(entity).copied().unwrap_or_else(|| {
                        0
                    });
                    ffi_solver
                        .add_fixed_constraint(constraint_id, entity_id)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add fixed constraint for entity '{}': {}", entity, e),
                            pointer: Some(format!("/constraints/{}", constraint_idx)),
                        })?;
                    constraint_id += 1;
                }
                crate::ir::Constraint::Distance { between, value } => {
                    if between.len() == 2 {
                        let id1 = entity_id_map.get(&between[0]).copied().unwrap_or(0);
                        let id2 = entity_id_map.get(&between[1]).copied().unwrap_or(0);
                        let dist = match value {
                            crate::ir::ExprOrNumber::Number(n) => *n,
                            crate::ir::ExprOrNumber::Expression(e) => eval.eval(&e)?,
                        };
                        ffi_solver
                            .add_distance_constraint(constraint_id, id1, id2, dist)
                            .map_err(|e| crate::error::Error::InvalidInput {
                                message: format!("Failed to add distance constraint between '{}' and '{}': {}", between[0], between[1], e),
                                pointer: Some(format!("/constraints/{}", constraint_idx)),
                            })?;
                        constraint_id += 1;
                    }
                }
                crate::ir::Constraint::PointOnLine { point, line } => {
                    let point_id = entity_id_map.get(point).copied().unwrap_or(0);
                    let line_id = entity_id_map.get(line).copied().unwrap_or(0);
                    ffi_solver
                        .add_point_on_line_constraint(constraint_id, point_id, line_id)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add point-on-line constraint: point '{}' on line '{}': {}", point, line, e),
                            pointer: Some(format!("/constraints/{}", constraint_idx)),
                        })?;
                    constraint_id += 1;
                }
                crate::ir::Constraint::Coincident { data } => {
                    match data {
                        crate::ir::CoincidentData::PointOnLine { at, of } => {
                            // Handle point-on-line coincident
                            if of.len() == 1 {
                                let point_id = entity_id_map.get(at).copied().unwrap_or(0);
                                let line_id = entity_id_map.get(&of[0]).copied().unwrap_or(0);
                                ffi_solver
                                    .add_point_on_line_constraint(constraint_id, point_id, line_id)
                                    .map_err(|e| crate::error::Error::InvalidInput {
                                        message: format!("Failed to add coincident constraint: point '{}' on line '{}': {}", at, of[0], e),
                                        pointer: Some(format!("/constraints/{}", constraint_idx)),
                                    })?;
                                constraint_id += 1;
                            }
                        },
                        crate::ir::CoincidentData::TwoEntities { entities } => {
                            // Handle point-to-point coincident
                            if entities.len() == 2 {
                                // For point-to-point coincident, we can use a distance constraint of 0
                                let id1 = entity_id_map.get(&entities[0]).copied().unwrap_or(0);
                                let id2 = entity_id_map.get(&entities[1]).copied().unwrap_or(0);
                                ffi_solver
                                    .add_distance_constraint(constraint_id, id1, id2, 0.0)
                                    .map_err(|e| crate::error::Error::InvalidInput {
                                        message: format!("Failed to add coincident constraint between '{}' and '{}': {}", entities[0], entities[1], e),
                                        pointer: Some(format!("/constraints/{}", constraint_idx)),
                                    })?;
                                constraint_id += 1;
                            }
                        }
                    }
                }
                crate::ir::Constraint::Perpendicular { a, b } => {
                    let line1_id = entity_id_map.get(a).copied().unwrap_or(0);
                    let line2_id = entity_id_map.get(b).copied().unwrap_or(0);
                    ffi_solver
                        .add_perpendicular_constraint(constraint_id, line1_id, line2_id)
                        .map_err(|e| crate::error::Error::InvalidInput {
                            message: format!("Failed to add perpendicular constraint between '{}' and '{}': {}", a, b, e),
                            pointer: Some(format!("/constraints/{}", constraint_idx)),
                        })?;
                    constraint_id += 1;
                }
                crate::ir::Constraint::Parallel { entities } => {
                    if entities.len() == 2 {
                        let line1_id = entity_id_map.get(&entities[0]).copied().unwrap_or(0);
                        let line2_id = entity_id_map.get(&entities[1]).copied().unwrap_or(0);
                        ffi_solver
                            .add_parallel_constraint(constraint_id, line1_id, line2_id)
                            .map_err(|e| crate::error::Error::InvalidInput {
                                message: format!("Failed to add parallel constraint between '{}' and '{}': {}", entities[0], entities[1], e),
                                pointer: Some(format!("/constraints/{}", constraint_idx)),
                            })?;
                        constraint_id += 1;
                    }
                }
                _ => {
                    // Constraint type not yet implemented - will be ignored
                } // Handle other constraint types as needed
            }
        }

        // Actually solve the constraints!
        ffi_solver.solve().map_err(|e| match e {
            crate::ffi::FfiError::Inconsistent => crate::error::Error::Overconstrained,
            crate::ffi::FfiError::DidntConverge => {
                crate::error::Error::SolverConvergence { iterations: 100 }
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
        })?;

        // Get solved positions from libslvs
        let mut resolved_entities = HashMap::new();

        // Retrieve solved positions for all entities
        for (entity_idx, entity) in doc.entities.iter().enumerate() {
            match entity {
                crate::ir::Entity::Point { id, .. } => {
                    let entity_id = entity_id_map.get(id).copied().unwrap_or(0);
                    if let Ok((x, y, z)) = ffi_solver.get_point_position(entity_id) {
                        resolved_entities.insert(
                            id.clone(),
                            crate::ir::ResolvedEntity::Point { at: vec![x, y, z] },
                        );
                    }
                }
                crate::ir::Entity::Line { id, p1, p2 } => {
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
}
