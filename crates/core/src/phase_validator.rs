use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct GearData {
    pub id: String,
    pub center: [f64; 2],
    pub teeth: u32,
    pub module: f64,
    pub phase: f64,
    pub internal: bool,
}

#[derive(Debug)]
pub struct Overlap {
    pub gear1: String,
    pub gear2: String,
    pub overlap_type: OverlapType,
    pub distance: f64,
    pub min_safe_distance: f64,
}

#[derive(Debug)]
pub enum OverlapType {
    ToothCollision,     // Teeth physically overlap
    InsufficientClearance, // Too close for 3D printing
    MeshingTeethOverlap,  // Gears that should mesh have overlapping teeth
}

/// Validate that Phase 2 solution has NO overlaps
/// Returns Ok(()) if valid, Err with overlap details if invalid
pub fn validate_phase_solution(
    gears: &HashMap<String, GearData>,
    mesh_constraints: &[(String, String)],
) -> Result<(), Vec<Overlap>> {
    let mut overlaps = Vec::new();
    
    // Minimum clearance for 3D printing (in mm)
    const MIN_CLEARANCE_3D_PRINT: f64 = 0.5;
    const SAFE_CLEARANCE_3D_PRINT: f64 = 0.8;
    
    // Check all gear pairs
    let gear_list: Vec<_> = gears.values().collect();
    for i in 0..gear_list.len() {
        for j in i+1..gear_list.len() {
            let g1 = gear_list[i];
            let g2 = gear_list[j];
            
            // Calculate center distance
            let dx = g2.center[0] - g1.center[0];
            let dy = g2.center[1] - g1.center[1];
            let center_dist = (dx * dx + dy * dy).sqrt();
            
            // Skip if both at origin (sun and ring)
            if center_dist < 0.01 {
                continue;
            }
            
            // Skip checking ring against gears that are not meant to mesh with it
            // Only skip if they're clearly inside and not near the ring teeth
            if g1.internal && !g2.internal && g1.id.contains("ring") {
                // Ring is internal, other gear is external
                let ring_pitch_r = (g1.teeth as f64 * g1.module) / 2.0;
                let gear_outer_r = (g2.teeth as f64 * g2.module) / 2.0 + g2.module;
                // Only skip if gear is well inside ring and not meshing
                let is_meshing = mesh_constraints.iter().any(|(a, b)| 
                    (a == &g1.id && b == &g2.id) || (a == &g2.id && b == &g1.id));
                if !is_meshing && (center_dist + gear_outer_r) < (ring_pitch_r - g1.module - 2.0) {
                    continue; // Gear is safely inside, not near ring teeth
                }
            }
            if g2.internal && !g1.internal && g2.id.contains("ring") {
                // Same check but g2 is ring
                let ring_pitch_r = (g2.teeth as f64 * g2.module) / 2.0;
                let gear_outer_r = (g1.teeth as f64 * g1.module) / 2.0 + g1.module;
                let is_meshing = mesh_constraints.iter().any(|(a, b)| 
                    (a == &g1.id && b == &g2.id) || (a == &g2.id && b == &g1.id));
                if !is_meshing && (center_dist + gear_outer_r) < (ring_pitch_r - g2.module - 2.0) {
                    continue;
                }
            }
            
            // Check if these gears should be meshing
            let should_mesh = mesh_constraints.iter().any(|(a, b)| 
                (a == &g1.id && b == &g2.id) || (a == &g2.id && b == &g1.id)
            );
            
            if should_mesh {
                // Validate meshing gears
                if let Some(overlap) = check_meshing_overlap(g1, g2, center_dist) {
                    overlaps.push(overlap);
                }
            } else {
                // Validate non-meshing gears have sufficient clearance
                if let Some(overlap) = check_non_meshing_clearance(g1, g2, center_dist, MIN_CLEARANCE_3D_PRINT) {
                    overlaps.push(overlap);
                }
            }
        }
    }
    
    if overlaps.is_empty() {
        Ok(())
    } else {
        Err(overlaps)
    }
}

/// Check if meshing gears have tooth overlap (phase error)
fn check_meshing_overlap(g1: &GearData, g2: &GearData, center_dist: f64) -> Option<Overlap> {
    // Account for 3D printing clearance
    const TOOTH_CLEARANCE: f64 = 0.7;
    
    // Calculate pitch radii
    let pitch_r1 = (g1.teeth as f64 * g1.module) / 2.0;
    let pitch_r2 = (g2.teeth as f64 * g2.module) / 2.0;
    
    // Calculate actual tooth tip radii with 3D printing clearance
    let tooth_r1 = if g1.internal {
        pitch_r1 - g1.module + TOOTH_CLEARANCE  // Internal teeth shortened inward
    } else {
        pitch_r1 + g1.module - TOOTH_CLEARANCE  // External teeth shortened outward
    };
    
    let tooth_r2 = if g2.internal {
        pitch_r2 - g2.module + TOOTH_CLEARANCE  // Internal teeth shortened inward
    } else {
        pitch_r2 + g2.module - TOOTH_CLEARANCE  // External teeth shortened outward
    };
    
    // Check for overlap based on gear types
    if g1.internal != g2.internal {
        // One internal (ring), one external (planet)
        // For ring-planet: planet must not extend beyond ring's inner teeth
        let (ring_r, planet_r, planet_dist) = if g1.internal {
            (tooth_r1, tooth_r2, center_dist)  // g1 is ring, g2 is planet
        } else {
            (tooth_r2, tooth_r1, center_dist)  // g2 is ring, g1 is planet
        };
        
        // Planet's outer edge is at planet_dist + planet_r from origin
        // Ring's inner teeth are at ring_r from origin
        // There's overlap if planet extends beyond ring
        if planet_dist + planet_r > ring_r {
            return Some(Overlap {
                gear1: g1.id.clone(),
                gear2: g2.id.clone(),
                overlap_type: OverlapType::ToothCollision,
                distance: center_dist,
                min_safe_distance: ring_r - planet_r,  // Planet should be at most this far from origin
            });
        }
    } else {
        // Both external - check normal distance
        let expected_dist = pitch_r1 + pitch_r2;
        let dist_error = (center_dist - expected_dist).abs();
        if dist_error > 1.0 {
            return Some(Overlap {
                gear1: g1.id.clone(),
                gear2: g2.id.clone(),
                overlap_type: OverlapType::ToothCollision,
                distance: center_dist,
                min_safe_distance: expected_dist,
            });
        }
    }
    
    // Check phase alignment - teeth should interleave
    if check_teeth_collision(g1, g2) {
        // Calculate expected distance for the error message
        let expected_dist = if g1.internal != g2.internal {
            (pitch_r1 - pitch_r2).abs()
        } else {
            pitch_r1 + pitch_r2
        };
        
        return Some(Overlap {
            gear1: g1.id.clone(),
            gear2: g2.id.clone(),
            overlap_type: OverlapType::MeshingTeethOverlap,
            distance: center_dist,
            min_safe_distance: expected_dist,
        });
    }
    
    None
}

/// Check if non-meshing gears have sufficient clearance
fn check_non_meshing_clearance(g1: &GearData, g2: &GearData, center_dist: f64, min_clearance: f64) -> Option<Overlap> {
    // Account for 3D printing clearance - teeth are shortened by 0.7mm
    const TOOTH_CLEARANCE: f64 = 0.7;
    
    // Calculate actual outer radii (where shortened teeth tips are)
    let outer_r1 = if g1.internal {
        // Internal teeth point inward, but are shortened
        (g1.teeth as f64 * g1.module) / 2.0 - g1.module + TOOTH_CLEARANCE
    } else {
        // External teeth point outward, but are shortened
        (g1.teeth as f64 * g1.module) / 2.0 + g1.module - TOOTH_CLEARANCE
    };
    
    let outer_r2 = if g2.internal {
        (g2.teeth as f64 * g2.module) / 2.0 - g2.module + TOOTH_CLEARANCE
    } else {
        (g2.teeth as f64 * g2.module) / 2.0 + g2.module - TOOTH_CLEARANCE
    };
    
    // Minimum safe distance is sum of outer radii plus clearance
    let min_safe_dist = outer_r1 + outer_r2 + min_clearance;
    
    if center_dist < min_safe_dist {
        return Some(Overlap {
            gear1: g1.id.clone(),
            gear2: g2.id.clone(),
            overlap_type: if center_dist < outer_r1 + outer_r2 {
                OverlapType::ToothCollision
            } else {
                OverlapType::InsufficientClearance
            },
            distance: center_dist,
            min_safe_distance: min_safe_dist,
        });
    }
    
    None
}

/// Check if teeth of two meshing gears collide based on their phases
fn check_teeth_collision(g1: &GearData, g2: &GearData) -> bool {
    // Calculate angle from g1 to g2
    let dx = g2.center[0] - g1.center[0];
    let dy = g2.center[1] - g1.center[1];
    let angle = dy.atan2(dx);
    
    // Calculate tooth angular width for each gear
    let tooth_angle1 = 2.0 * PI / g1.teeth as f64;
    let tooth_angle2 = 2.0 * PI / g2.teeth as f64;
    
    // For simplicity, assume tooth width is 40% of tooth pitch
    let tooth_width1 = tooth_angle1 * 0.4;
    let tooth_width2 = tooth_angle2 * 0.4;
    
    // Calculate relative phase at mesh point
    let phase1_rad = g1.phase.to_radians();
    let phase2_rad = g2.phase.to_radians();
    
    // Find closest teeth at mesh point
    let mesh_angle1 = (angle + PI - phase1_rad) % tooth_angle1;
    let mesh_angle2 = (angle - phase2_rad) % tooth_angle2;
    
    // Check if teeth overlap
    // Teeth should be offset by at least half a tooth for proper meshing
    let phase_diff = (mesh_angle1 - mesh_angle2).abs();
    let min_phase_diff = (tooth_width1 + tooth_width2) / 2.0;
    
    // If phase difference is too small, teeth collide
    phase_diff < min_phase_diff
}

/// Validate that all gears use the same module
pub fn validate_same_module(gears: &HashMap<String, GearData>) -> Result<(), String> {
    if gears.is_empty() {
        return Ok(());
    }
    
    let first_module = gears.values().next().unwrap().module;
    for gear in gears.values() {
        if (gear.module - first_module).abs() > 0.001 {
            return Err(format!(
                "Module mismatch: {} has module {} but expected {}",
                gear.id, gear.module, first_module
            ));
        }
    }
    Ok(())
}

/// Validate assembly constraint for planetary system
pub fn validate_assembly_constraint(
    gears: &HashMap<String, GearData>,
    mesh_constraints: &[(String, String)],
) -> Result<(), String> {
    // Find sun and ring
    let sun = gears.get("sun");
    let ring = gears.get("ring");
    
    if let (Some(sun), Some(ring)) = (sun, ring) {
        // Count planets (gears that mesh with sun)
        let num_planets = mesh_constraints.iter()
            .filter(|(a, b)| (a == "sun" || b == "sun") && a != "ring" && b != "ring")
            .count() as f64;
        
        if num_planets > 0.0 {
            let assembly_value = (sun.teeth as f64 + ring.teeth as f64) / num_planets;
            let remainder = assembly_value - assembly_value.floor();
            
            if remainder > 0.001 && remainder < 0.999 {
                return Err(format!(
                    "Assembly constraint violated: ({} + {}) / {} = {} (must be integer)",
                    sun.teeth, ring.teeth, num_planets, assembly_value
                ));
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_no_overlap_valid_system() {
        let mut gears = HashMap::new();
        
        // Sun at origin
        gears.insert("sun".to_string(), GearData {
            id: "sun".to_string(),
            center: [0.0, 0.0],
            teeth: 24,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        // Planet at correct distance
        gears.insert("planet1".to_string(), GearData {
            id: "planet1".to_string(),
            center: [36.0, 0.0], // (24+12)*2/2 = 36
            teeth: 12,
            module: 2.0,
            phase: 15.0, // Proper phase offset
            internal: false,
        });
        
        let mesh_constraints = vec![
            ("sun".to_string(), "planet1".to_string()),
        ];
        
        let result = validate_phase_solution(&gears, &mesh_constraints);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_detect_overlap() {
        let mut gears = HashMap::new();
        
        // Two gears too close
        gears.insert("gear1".to_string(), GearData {
            id: "gear1".to_string(),
            center: [0.0, 0.0],
            teeth: 20,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        gears.insert("gear2".to_string(), GearData {
            id: "gear2".to_string(),
            center: [20.0, 0.0], // Too close!
            teeth: 20,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        let mesh_constraints = vec![];
        
        let result = validate_phase_solution(&gears, &mesh_constraints);
        assert!(result.is_err());
        
        if let Err(overlaps) = result {
            assert!(!overlaps.is_empty());
            // Just check that we detected an overlap
        }
    }
}