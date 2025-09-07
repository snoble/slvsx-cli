use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

/// Calculate circular mean of angles in degrees
fn circular_mean(angles: &[f64]) -> f64 {
    let mut sum_sin = 0.0;
    let mut sum_cos = 0.0;
    
    for &angle in angles {
        let rad = angle.to_radians();
        sum_sin += rad.sin();
        sum_cos += rad.cos();
    }
    
    let mean_rad = sum_sin.atan2(sum_cos);
    let mut mean_deg = mean_rad.to_degrees();
    
    // Normalize to 0-360 range
    while mean_deg < 0.0 {
        mean_deg += 360.0;
    }
    while mean_deg >= 360.0 {
        mean_deg -= 360.0;
    }
    
    mean_deg
}

#[derive(Debug, Clone)]
pub struct GearInfo {
    pub id: String,
    pub teeth: u32,
    pub center: [f64; 2],
    pub internal: bool,
}

#[derive(Debug, Clone)]
pub struct MeshConstraint {
    pub gear1: String,
    pub gear2: String,
}

/// Find connected components in the gear system
fn find_connected_components(
    gears: &HashMap<String, GearInfo>,
    mesh_constraints: &[MeshConstraint],
) -> Vec<Vec<String>> {
    let mut components = Vec::new();
    let mut visited = std::collections::HashSet::new();
    
    // Build adjacency list
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for gear_id in gears.keys() {
        adjacency.insert(gear_id.clone(), Vec::new());
    }
    
    for constraint in mesh_constraints {
        if let Some(neighbors) = adjacency.get_mut(&constraint.gear1) {
            neighbors.push(constraint.gear2.clone());
        }
        if let Some(neighbors) = adjacency.get_mut(&constraint.gear2) {
            neighbors.push(constraint.gear1.clone());
        }
    }
    
    // Find components using DFS
    for gear_id in gears.keys() {
        if !visited.contains(gear_id) {
            let mut component = Vec::new();
            let mut stack = vec![gear_id.clone()];
            
            while let Some(current) = stack.pop() {
                if visited.insert(current.clone()) {
                    component.push(current.clone());
                    if let Some(neighbors) = adjacency.get(&current) {
                        for neighbor in neighbors {
                            if !visited.contains(neighbor) {
                                stack.push(neighbor.clone());
                            }
                        }
                    }
                }
            }
            
            if !component.is_empty() {
                components.push(component);
            }
        }
    }
    
    components
}

/// Calculate phases for all gears based on mesh constraints
/// This should be done AFTER positions are determined by the solver
pub fn calculate_gear_phases(
    gears: &HashMap<String, GearInfo>,
    mesh_constraints: &[MeshConstraint],
) -> HashMap<String, f64> {
    let mut phases = HashMap::new();
    
    // Find connected components
    let components = find_connected_components(gears, mesh_constraints);
    eprintln!("Found {} connected component(s) in gear system", components.len());
    
    for (comp_idx, component) in components.iter().enumerate() {
        eprintln!("Component {}: {:?}", comp_idx, component);
        
        // Choose reference gear for this component
        let reference_gear = if component.contains(&"sun".to_string()) {
            "sun".to_string()
        } else if component.contains(&"ring".to_string()) {
            "ring".to_string()
        } else {
            // Choose gear closest to origin
            component.iter()
                .min_by_key(|id| {
                    if let Some(gear) = gears.get(*id) {
                        let dist = (gear.center[0].powi(2) + gear.center[1].powi(2)) as i64;
                        dist
                    } else {
                        i64::MAX
                    }
                })
                .cloned()
                .unwrap_or_else(|| component[0].clone())
        };
        
        eprintln!("Setting reference gear '{}' for component {} to phase 0", reference_gear, comp_idx);
        phases.insert(reference_gear.clone(), 0.0);
        
        // Process this component
        let mut processed = std::collections::HashSet::new();
        processed.insert(reference_gear.clone());
        
        // Process mesh constraints for this component only
        let max_iterations = 100;
        let mut iteration = 0;
        
        while processed.len() < component.len() && iteration < max_iterations {
            iteration += 1;
            let mut new_phases = Vec::new();
            
            for constraint in mesh_constraints {
                // Skip constraints not in this component
                if !component.contains(&constraint.gear1) || !component.contains(&constraint.gear2) {
                    continue;
                }
                
                // Check if we can process this constraint
                let gear1_has_phase = phases.contains_key(&constraint.gear1);
                let gear2_has_phase = phases.contains_key(&constraint.gear2);
                
                if gear1_has_phase && !gear2_has_phase {
                    // Calculate phase for gear2 based on gear1
                    if let (Some(g1), Some(g2)) = (gears.get(&constraint.gear1), gears.get(&constraint.gear2)) {
                        let phase1 = phases[&constraint.gear1];
                        let phase2 = calculate_mesh_phase(g1, g2, phase1);
                        eprintln!("  Calculating phase for {} from {} (phase={:.1}°): {:.1}°", 
                            constraint.gear2, constraint.gear1, phase1, phase2);
                        new_phases.push((constraint.gear2.clone(), phase2));
                    }
                } else if !gear1_has_phase && gear2_has_phase {
                    // Calculate phase for gear1 based on gear2
                    if let (Some(g1), Some(g2)) = (gears.get(&constraint.gear1), gears.get(&constraint.gear2)) {
                        let phase2 = phases[&constraint.gear2];
                        let phase1 = calculate_mesh_phase_reverse(g1, g2, phase2);
                        eprintln!("  Calculating phase for {} from {} (phase={:.1}°): {:.1}°", 
                            constraint.gear1, constraint.gear2, phase2, phase1);
                        new_phases.push((constraint.gear1.clone(), phase1));
                    }
                }
            }
            
            // Group phase demands by gear for averaging
            let mut phase_demands: HashMap<String, Vec<f64>> = HashMap::new();
            for (gear_id, phase) in new_phases {
                phase_demands.entry(gear_id).or_insert_with(Vec::new).push(phase);
            }
            
            // Process phase demands
            for (gear_id, demands) in phase_demands {
                if let Some(&existing_phase) = phases.get(&gear_id) {
                    // Gear already has a phase - check if new demands are consistent
                    for &phase in &demands {
                        let phase_diff = (phase - existing_phase).abs();
                        let normalized_diff = phase_diff % 360.0;
                        let min_diff = normalized_diff.min(360.0 - normalized_diff);
                        
                        if min_diff > 1.0 {  // Allow 1 degree tolerance
                            eprintln!("  WARNING: Phase conflict for {}: existing={:.1}°, calculated={:.1}° (diff={:.1}°)",
                                gear_id, existing_phase, phase, min_diff);
                        }
                    }
                } else if demands.len() == 1 {
                    // Single demand - use it directly
                    phases.insert(gear_id.clone(), demands[0]);
                    processed.insert(gear_id);
                } else {
                    // Multiple demands - average them (accounting for circular mean)
                    let avg_phase = circular_mean(&demands);
                    eprintln!("  Averaging {} phase demands for {}: {:?} -> {:.1}°", 
                        demands.len(), gear_id, demands, avg_phase);
                    phases.insert(gear_id.clone(), avg_phase);
                    processed.insert(gear_id);
                }
            }
            
            // Break if no progress
            if processed.len() == component.len() {
                break;
            }
        }
        
        // Check if all gears in component have phases
        for gear_id in component {
            if !phases.contains_key(gear_id) {
                eprintln!("  WARNING: Could not calculate phase for {} in component {}", gear_id, comp_idx);
                phases.insert(gear_id.clone(), 0.0);
            }
        }
    }
    
    phases
}

/// Calculate phase for gear2 when meshing with gear1 at known phase
fn calculate_mesh_phase(gear1: &GearInfo, gear2: &GearInfo, phase1: f64) -> f64 {
    // Calculate angle from gear1 center to gear2 center
    let dx = gear2.center[0] - gear1.center[0];
    let dy = gear2.center[1] - gear1.center[1];
    let angle = dy.atan2(dx);
    
    // Calculate gear ratio
    let ratio = gear1.teeth as f64 / gear2.teeth as f64;
    
    // Calculate phase for proper meshing
    let _tooth_angle1 = 2.0 * PI / gear1.teeth as f64;
    let tooth_angle2 = 2.0 * PI / gear2.teeth as f64;
    
    // For proper meshing, teeth need to interleave
    let mut phase2 = if gear1.internal != gear2.internal {
        // One internal, one external
        if gear2.internal {
            // gear2 is internal (ring), gear1 is external (planet)
            // Ring teeth point inward, need half-tooth offset
            phase1 - angle.to_degrees() * ratio + tooth_angle2.to_degrees() / 2.0
        } else {
            // gear1 is internal (ring), gear2 is external (planet)
            // Ring teeth point inward, need half-tooth offset
            phase1 + angle.to_degrees() * ratio + tooth_angle2.to_degrees() / 2.0
        }
    } else {
        // Both external or both internal
        // For external-external, the geometry determines the meshing
        // No additional offset needed
        phase1 + angle.to_degrees() * ratio
    };
    
    // Add correction based on position angle
    // This ensures teeth mesh properly based on center-to-center angle
    let position_correction = (angle.to_degrees() / (360.0 / gear2.teeth as f64)).fract() * (360.0 / gear2.teeth as f64);
    phase2 += position_correction;
    
    // Normalize to 0-360 range
    while phase2 < 0.0 {
        phase2 += 360.0;
    }
    while phase2 >= 360.0 {
        phase2 -= 360.0;
    }
    
    phase2
}

/// Calculate phase for gear1 when gear2 has known phase (reverse calculation)
fn calculate_mesh_phase_reverse(gear1: &GearInfo, gear2: &GearInfo, phase2: f64) -> f64 {
    // Calculate angle from gear2 center to gear1 center
    let dx = gear1.center[0] - gear2.center[0];
    let dy = gear1.center[1] - gear2.center[1];
    let angle = dy.atan2(dx);
    
    // Calculate gear ratio
    let ratio = gear2.teeth as f64 / gear1.teeth as f64;
    
    // Calculate phase for proper meshing (inverse of forward calculation)
    let tooth_angle1 = 2.0 * PI / gear1.teeth as f64;
    
    let mut phase1 = if gear1.internal != gear2.internal {
        // One internal, one external
        if gear1.internal {
            phase2 + angle.to_degrees() * ratio
        } else {
            phase2 - angle.to_degrees() * ratio
        }
    } else {
        // Both external or both internal
        phase2 - angle.to_degrees() * ratio - (tooth_angle1 / 2.0).to_degrees()
    };
    
    // Add correction based on position angle
    let position_correction = (angle.to_degrees() / (360.0 / gear1.teeth as f64)).fract() * (360.0 / gear1.teeth as f64);
    phase1 -= position_correction;
    
    // Normalize to 0-360 range
    while phase1 < 0.0 {
        phase1 += 360.0;
    }
    while phase1 >= 360.0 {
        phase1 -= 360.0;
    }
    
    phase1
}

/// Validate that the assembly constraint is satisfied
/// Returns true if (S + R) / n is an integer
pub fn validate_assembly_constraint(sun_teeth: u32, ring_teeth: u32, num_planets: u32) -> bool {
    let sum = (sun_teeth + ring_teeth) as f64;
    let ratio = sum / num_planets as f64;
    let remainder = ratio - ratio.floor();
    
    // Check if close to integer (within floating point tolerance)
    remainder < 0.001 || remainder > 0.999
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_assembly_constraint() {
        // Valid: (24 + 72) / 6 = 16
        assert!(validate_assembly_constraint(24, 72, 6));
        
        // Invalid: (24 + 72) / 5 = 19.2
        assert!(!validate_assembly_constraint(24, 72, 5));
        
        // Valid: (20 + 80) / 4 = 25
        assert!(validate_assembly_constraint(20, 80, 4));
    }
    
    #[test]
    fn test_phase_calculation() {
        let mut gears = HashMap::new();
        
        // Sun gear at origin
        gears.insert("sun".to_string(), GearInfo {
            id: "sun".to_string(),
            teeth: 24,
            center: [0.0, 0.0],
            internal: false,
        });
        
        // Planet at 36mm on X axis
        gears.insert("planet1".to_string(), GearInfo {
            id: "planet1".to_string(),
            teeth: 12,
            center: [36.0, 0.0],
            internal: false,
        });
        
        let constraints = vec![
            MeshConstraint {
                gear1: "sun".to_string(),
                gear2: "planet1".to_string(),
            }
        ];
        
        let phases = calculate_gear_phases(&gears, &constraints);
        
        // Sun should be at 0
        assert_eq!(phases["sun"], 0.0);
        
        // Planet should have calculated phase
        assert!(phases.contains_key("planet1"));
        let planet_phase = phases["planet1"];
        
        // Phase should be positive and less than 360
        assert!(planet_phase >= 0.0 && planet_phase < 360.0);
    }
}