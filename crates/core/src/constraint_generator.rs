use crate::ir::{Constraint, Entity, ExprOrNumber, InputDocument};
use std::collections::HashMap;

/// Generate Phase 2 constraints from Phase 1 solved positions
/// This takes the solved positions and generates new constraints for phase alignment
pub fn generate_phase_constraints(
    phase1_doc: &InputDocument,
    solved_positions: &HashMap<String, Vec<f64>>,
) -> InputDocument {
    let mut phase2_doc = phase1_doc.clone();
    
    // Clear existing constraints - we'll generate new ones for phase solving
    phase2_doc.constraints.clear();
    
    // Update entities with fixed positions from Phase 1
    let mut new_entities = Vec::new();
    for entity in &phase1_doc.entities {
        match entity {
            Entity::Gear { id, teeth, module, pressure_angle, phase, internal, .. } => {
                // Use solved position from Phase 1
                if let Some(pos) = solved_positions.get(id) {
                    new_entities.push(Entity::Gear {
                        id: id.clone(),
                        center: vec![
                            ExprOrNumber::Number(pos[0]),
                            ExprOrNumber::Number(pos[1]),
                            ExprOrNumber::Number(pos.get(2).copied().unwrap_or(0.0)),
                        ],
                        teeth: teeth.clone(),
                        module: module.clone(),
                        pressure_angle: pressure_angle.clone(),
                        phase: phase.clone(), // Will be solved in Phase 2
                        internal: *internal,
                    });
                }
            }
            _ => new_entities.push(entity.clone()),
        }
    }
    phase2_doc.entities = new_entities;
    
    // Generate phase alignment constraints for meshing gears
    for constraint in &phase1_doc.constraints {
        if let Constraint::Mesh { gear1, gear2 } = constraint {
            // For meshing gears, we need phase alignment constraint
            // The phase difference should ensure teeth interleave properly
            
            // Add a phase relationship constraint
            // For external-external: phase2 = phase1 + angle + half_tooth
            // For internal-external: phase2 = phase1 - angle
            
            // First, add the original mesh constraint to maintain distance
            phase2_doc.constraints.push(Constraint::Mesh {
                gear1: gear1.clone(),
                gear2: gear2.clone(),
            });
            
            // Then add phase alignment constraint
            // This would be a new constraint type that libslvs understands
            // For now, we'll use a comment to indicate what's needed
            
            // TODO: Add phase constraint type to IR and libslvs
            // phase2_doc.constraints.push(Constraint::PhaseAlignment {
            //     gear1: gear1.clone(),
            //     gear2: gear2.clone(),
            // });
        }
    }
    
    // Add assembly constraint validation
    if let (Some(sun_params), Some(ring_params)) = (
        get_gear_params(&phase2_doc, "sun"),
        get_gear_params(&phase2_doc, "ring")
    ) {
        let num_planets = phase2_doc.constraints.iter()
            .filter(|c| matches!(c, Constraint::Mesh { gear1, gear2 } 
                if (gear1 == "sun" || gear2 == "sun") && gear1 != "ring" && gear2 != "ring"))
            .count() as u32;
        
        if num_planets > 0 {
            let assembly_value = (sun_params.0 + ring_params.0) as f64 / num_planets as f64;
            if (assembly_value - assembly_value.floor()).abs() > 0.001 {
                eprintln!("WARNING: Assembly constraint not satisfied!");
                eprintln!("({} + {}) / {} = {} (must be integer)", 
                    sun_params.0, ring_params.0, num_planets, assembly_value);
            }
        }
    }
    
    phase2_doc
}

/// Extract gear parameters from document
fn get_gear_params(doc: &InputDocument, gear_id: &str) -> Option<(u32, f64, bool)> {
    for entity in &doc.entities {
        if let Entity::Gear { id, teeth, module, internal, .. } = entity {
            if id == gear_id {
                let teeth_val = match teeth {
                    ExprOrNumber::Number(n) => *n as u32,
                    ExprOrNumber::Expression(expr) => {
                        // Would need expression evaluator here
                        if let Some(&param_val) = doc.parameters.get(expr.trim_start_matches('$')) {
                            param_val as u32
                        } else {
                            0
                        }
                    }
                };
                let module_val = match module {
                    ExprOrNumber::Number(n) => *n,
                    ExprOrNumber::Expression(expr) => {
                        if let Some(&param_val) = doc.parameters.get(expr.trim_start_matches('$')) {
                            param_val
                        } else {
                            0.0
                        }
                    }
                };
                return Some((teeth_val, module_val, *internal));
            }
        }
    }
    None
}

/// Generate angular position constraints for equal spacing
pub fn add_equal_spacing_constraints(doc: &mut InputDocument, orbit_name: &str, num_gears: usize) {
    // For gears on the same orbit, add angular spacing constraints
    let angle_between = 360.0 / num_gears as f64;
    
    // Find all gears with this orbit prefix
    let orbit_gears: Vec<String> = doc.entities.iter()
        .filter_map(|e| match e {
            Entity::Gear { id, .. } if id.starts_with(orbit_name) => Some(id.clone()),
            _ => None,
        })
        .collect();
    
    if orbit_gears.len() != num_gears {
        eprintln!("WARNING: Expected {} gears in orbit '{}', found {}", 
            num_gears, orbit_name, orbit_gears.len());
        return;
    }
    
    // Add angular constraints between consecutive gears
    for i in 0..orbit_gears.len() {
        let next = (i + 1) % orbit_gears.len();
        
        // Add angular spacing constraint
        // This ensures gears are equally spaced around the orbit
        // TODO: Add AngularSpacing constraint type to IR
        // doc.constraints.push(Constraint::AngularSpacing {
        //     entity1: orbit_gears[i].clone(),
        //     entity2: orbit_gears[next].clone(),
        //     angle: ExprOrNumber::Number(angle_between),
        // });
    }
}