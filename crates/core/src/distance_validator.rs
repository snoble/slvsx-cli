use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DistanceValidation {
    pub gear1: String,
    pub gear2: String,
    pub actual_distance: f64,
    pub expected_distance: f64,
    pub error: f64,
    pub is_meshing: bool,
    pub validation_passed: bool,
}

/// Validates all gear distances in a solution
/// This runs on EVERY solution to ensure geometric correctness
pub fn validate_all_distances(
    gears: &HashMap<String, crate::phase_validator::GearData>,
    mesh_constraints: &[(String, String)],
    tolerance: f64,
) -> Vec<DistanceValidation> {
    let mut validations = Vec::new();
    
    // Check all gear pairs
    let gear_list: Vec<_> = gears.values().collect();
    for i in 0..gear_list.len() {
        for j in i+1..gear_list.len() {
            let g1 = gear_list[i];
            let g2 = gear_list[j];
            
            // Calculate actual center distance
            let dx = g2.center[0] - g1.center[0];
            let dy = g2.center[1] - g1.center[1];
            let actual_dist = (dx * dx + dy * dy).sqrt();
            
            // Skip if both at origin (sun and ring case)
            if actual_dist < 0.01 {
                continue;
            }
            
            // Check if these gears should be meshing
            let is_meshing = mesh_constraints.iter().any(|(a, b)| 
                (a == &g1.id && b == &g2.id) || (a == &g2.id && b == &g1.id)
            );
            
            // Calculate expected distance based on pitch radii
            let pitch_r1 = (g1.teeth as f64 * g1.module) / 2.0;
            let pitch_r2 = (g2.teeth as f64 * g2.module) / 2.0;
            
            let expected_dist = if is_meshing {
                // For meshing gears, calculate proper center distance
                if g1.internal != g2.internal {
                    // One internal, one external (e.g., ring-planet)
                    (pitch_r1 - pitch_r2).abs()
                } else {
                    // Both external or both internal (e.g., sun-planet)
                    pitch_r1 + pitch_r2
                }
            } else {
                // For non-meshing gears, there's no specific expected distance
                // but we can still record the actual distance
                actual_dist
            };
            
            let error = (actual_dist - expected_dist).abs();
            let validation_passed = error <= tolerance;
            
            validations.push(DistanceValidation {
                gear1: g1.id.clone(),
                gear2: g2.id.clone(),
                actual_distance: actual_dist,
                expected_distance: expected_dist,
                error,
                is_meshing,
                validation_passed,
            });
        }
    }
    
    validations
}

/// Checks if all critical distances are valid
pub fn check_critical_distances(
    validations: &[DistanceValidation],
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    for v in validations {
        if v.is_meshing && !v.validation_passed {
            errors.push(format!(
                "{} <-> {}: Distance error {:.2}mm (actual: {:.2}mm, expected: {:.2}mm)",
                v.gear1, v.gear2, v.error, v.actual_distance, v.expected_distance
            ));
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phase_validator::GearData;
    
    #[test]
    fn test_distance_validation() {
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
            center: [36.0, 0.0],  // 24 + 12 = 36mm (correct)
            teeth: 12,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        // Planet at wrong distance
        gears.insert("planet2".to_string(), GearData {
            id: "planet2".to_string(),
            center: [35.0, 0.0],  // Should be 36mm
            teeth: 12,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        let mesh_constraints = vec![
            ("sun".to_string(), "planet1".to_string()),
            ("sun".to_string(), "planet2".to_string()),
        ];
        
        let validations = validate_all_distances(&gears, &mesh_constraints, 0.1);
        
        // Find validation for sun-planet1
        let sun_planet1 = validations.iter()
            .find(|v| (v.gear1 == "sun" && v.gear2 == "planet1") || 
                      (v.gear1 == "planet1" && v.gear2 == "sun"))
            .unwrap();
        
        assert!(sun_planet1.validation_passed, "Sun-planet1 should pass");
        assert_eq!(sun_planet1.expected_distance, 36.0);
        
        // Find validation for sun-planet2
        let sun_planet2 = validations.iter()
            .find(|v| (v.gear1 == "sun" && v.gear2 == "planet2") || 
                      (v.gear1 == "planet2" && v.gear2 == "sun"))
            .unwrap();
        
        assert!(!sun_planet2.validation_passed, "Sun-planet2 should fail");
        assert_eq!(sun_planet2.expected_distance, 36.0);
        assert!((sun_planet2.actual_distance - 35.0).abs() < 0.01);
        
        // Check critical distances
        let result = check_critical_distances(&validations);
        assert!(result.is_err(), "Should detect distance error");
        
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            assert!(errors[0].contains("planet2"));
        }
    }
    
    #[test]
    fn test_ring_planet_distance() {
        let mut gears = HashMap::new();
        
        // Ring at origin
        gears.insert("ring".to_string(), GearData {
            id: "ring".to_string(),
            center: [0.0, 0.0],
            teeth: 48,
            module: 2.0,
            phase: 0.0,
            internal: true,
        });
        
        // Planet at correct distance for ring mesh
        gears.insert("planet".to_string(), GearData {
            id: "planet".to_string(),
            center: [36.0, 0.0],  // |48 - 12| = 36mm (correct)
            teeth: 12,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        let mesh_constraints = vec![
            ("ring".to_string(), "planet".to_string()),
        ];
        
        let validations = validate_all_distances(&gears, &mesh_constraints, 0.1);
        
        let ring_planet = validations.iter()
            .find(|v| v.is_meshing)
            .unwrap();
        
        assert!(ring_planet.validation_passed, "Ring-planet mesh should pass");
        assert_eq!(ring_planet.expected_distance, 36.0);
    }
}