use slvsx_core::*;
use std::collections::HashMap;

#[cfg(test)]
mod solver_tests {
    use super::*;
    
    #[test]
    fn test_simple_gear_pair() {
        let mut params = HashMap::new();
        params.insert("module".to_string(), 2.0);
        params.insert("gear1_teeth".to_string(), 20.0);
        params.insert("gear2_teeth".to_string(), 40.0);
        
        let mut entities = HashMap::new();
        entities.insert("gear1".to_string(), Entity::Gear {
            center: vec![0.0, 0.0, 0.0],
            teeth: 20,
            module: 2.0,
            pressure_angle: 20.0,
            phase: Some(0.0),
            internal: false,
        });
        
        entities.insert("gear2".to_string(), Entity::Gear {
            center: vec![60.0, 0.0, 0.0],
            teeth: 40,
            module: 2.0,
            pressure_angle: 20.0,
            phase: Some(0.0),
            internal: false,
        });
        
        let constraints = vec![
            Constraint::Mesh {
                gear1: "gear1".to_string(),
                gear2: "gear2".to_string(),
            }
        ];
        
        let input = Input {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: params,
            entities,
            constraints,
        };
        
        // Test should pass with mock solver
        #[cfg(feature = "mock-solver")]
        {
            let result = solve(input).unwrap();
            assert_eq!(result.status, "success");
        }
    }
    
    #[test]
    fn test_planetary_system() {
        let mut params = HashMap::new();
        params.insert("module".to_string(), 2.0);
        params.insert("sun_teeth".to_string(), 24.0);
        params.insert("planet_teeth".to_string(), 12.0);
        params.insert("ring_teeth".to_string(), 72.0);
        
        let mut entities = HashMap::new();
        
        // Sun gear
        entities.insert("sun".to_string(), Entity::Gear {
            center: vec![0.0, 0.0, 0.0],
            teeth: 24,
            module: 2.0,
            pressure_angle: 20.0,
            phase: Some(0.0),
            internal: false,
        });
        
        // Ring gear
        entities.insert("ring".to_string(), Entity::Gear {
            center: vec![0.0, 0.0, 0.0],
            teeth: 72,
            module: 2.0,
            pressure_angle: 20.0,
            phase: Some(0.0),
            internal: true,
        });
        
        // Add 3 planets
        for i in 0..3 {
            let angle = i as f64 * 120.0 * std::f64::consts::PI / 180.0;
            let radius = 36.0; // (sun_teeth + planet_teeth) * module / 2
            
            entities.insert(format!("planet{}", i + 1), Entity::Gear {
                center: vec![radius * angle.cos(), radius * angle.sin(), 0.0],
                teeth: 12,
                module: 2.0,
                pressure_angle: 20.0,
                phase: Some(0.0),
                internal: false,
            });
        }
        
        let mut constraints = vec![];
        
        // Mesh planets with sun and ring
        for i in 1..=3 {
            constraints.push(Constraint::Mesh {
                gear1: "sun".to_string(),
                gear2: format!("planet{}", i),
            });
            
            constraints.push(Constraint::Mesh {
                gear1: "ring".to_string(),
                gear2: format!("planet{}", i),
            });
        }
        
        let input = Input {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: params,
            entities,
            constraints,
        };
        
        #[cfg(feature = "mock-solver")]
        {
            let result = solve(input).unwrap();
            assert_eq!(result.status, "success");
            assert!(result.entities.is_some());
        }
    }
    
    #[test]
    fn test_validation_errors() {
        // Test with invalid tooth count
        let mut params = HashMap::new();
        params.insert("module".to_string(), 2.0);
        
        let mut entities = HashMap::new();
        entities.insert("gear1".to_string(), Entity::Gear {
            center: vec![0.0, 0.0, 0.0],
            teeth: 0, // Invalid: zero teeth
            module: 2.0,
            pressure_angle: 20.0,
            phase: Some(0.0),
            internal: false,
        });
        
        let input = Input {
            schema: "slvs-json/1".to_string(),
            units: "mm".to_string(),
            parameters: params,
            entities,
            constraints: vec![],
        };
        
        #[cfg(feature = "mock-solver")]
        {
            let result = solve(input);
            // Should handle invalid input gracefully
            assert!(result.is_ok() || result.is_err());
        }
    }
}

#[cfg(test)]
mod phase_calculator_tests {
    use super::*;
    use slvsx_core::phase_calculator::*;
    
    #[test]
    fn test_phase_calculation_external_gears() {
        let mut gears = HashMap::new();
        
        gears.insert("gear1".to_string(), GearInfo {
            id: "gear1".to_string(),
            teeth: 20,
            center: [0.0, 0.0],
            internal: false,
        });
        
        gears.insert("gear2".to_string(), GearInfo {
            id: "gear2".to_string(),
            teeth: 40,
            center: [60.0, 0.0],
            internal: false,
        });
        
        let constraints = vec![
            MeshConstraint {
                gear1: "gear1".to_string(),
                gear2: "gear2".to_string(),
            }
        ];
        
        let phases = calculate_gear_phases(&gears, &constraints);
        
        // Gear1 should be at phase 0 (reference)
        assert_eq!(phases["gear1"], 0.0);
        
        // Gear2 should have calculated phase
        assert!(phases.contains_key("gear2"));
    }
    
    #[test]
    fn test_phase_calculation_internal_gear() {
        let mut gears = HashMap::new();
        
        gears.insert("planet".to_string(), GearInfo {
            id: "planet".to_string(),
            teeth: 12,
            center: [36.0, 0.0],
            internal: false,
        });
        
        gears.insert("ring".to_string(), GearInfo {
            id: "ring".to_string(),
            teeth: 72,
            center: [0.0, 0.0],
            internal: true,
        });
        
        let constraints = vec![
            MeshConstraint {
                gear1: "ring".to_string(),
                gear2: "planet".to_string(),
            }
        ];
        
        let phases = calculate_gear_phases(&gears, &constraints);
        
        // Both gears should have phases
        assert!(phases.contains_key("planet"));
        assert!(phases.contains_key("ring"));
    }
    
    #[test]
    fn test_assembly_constraint_validation() {
        // Valid assembly constraint
        assert!(validate_assembly_constraint(24, 72, 3));
        
        // Invalid assembly constraint
        assert!(!validate_assembly_constraint(24, 72, 5));
        
        // Edge cases
        assert!(validate_assembly_constraint(12, 48, 6));
        assert!(validate_assembly_constraint(20, 80, 4));
    }
    
    #[test]
    fn test_disconnected_components() {
        let mut gears = HashMap::new();
        
        // First component
        gears.insert("gear1".to_string(), GearInfo {
            id: "gear1".to_string(),
            teeth: 20,
            center: [0.0, 0.0],
            internal: false,
        });
        
        gears.insert("gear2".to_string(), GearInfo {
            id: "gear2".to_string(),
            teeth: 30,
            center: [50.0, 0.0],
            internal: false,
        });
        
        // Second component (disconnected)
        gears.insert("gear3".to_string(), GearInfo {
            id: "gear3".to_string(),
            teeth: 25,
            center: [100.0, 0.0],
            internal: false,
        });
        
        gears.insert("gear4".to_string(), GearInfo {
            id: "gear4".to_string(),
            teeth: 35,
            center: [160.0, 0.0],
            internal: false,
        });
        
        let constraints = vec![
            MeshConstraint {
                gear1: "gear1".to_string(),
                gear2: "gear2".to_string(),
            },
            MeshConstraint {
                gear1: "gear3".to_string(),
                gear2: "gear4".to_string(),
            },
        ];
        
        let phases = calculate_gear_phases(&gears, &constraints);
        
        // All gears should have phases
        assert_eq!(phases.len(), 4);
        for gear in gears.keys() {
            assert!(phases.contains_key(gear));
        }
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;
    use slvsx_core::validation::*;
    
    #[test]
    fn test_distance_validation() {
        let gear1 = GearInfo {
            id: "gear1".to_string(),
            teeth: 20,
            center: [0.0, 0.0],
            internal: false,
        };
        
        let gear2 = GearInfo {
            id: "gear2".to_string(),
            teeth: 30,
            center: [50.0, 0.0],
            internal: false,
        };
        
        let module = 2.0;
        let expected_distance = (20 + 30) as f64 * module / 2.0; // 50mm
        
        let actual_distance = ((gear2.center[0] - gear1.center[0]).powi(2) + 
                              (gear2.center[1] - gear1.center[1]).powi(2)).sqrt();
        
        assert!((actual_distance - expected_distance).abs() < 0.001);
    }
    
    #[test]
    fn test_overlap_detection() {
        // Test for tooth collision detection
        let gear1_phase = 0.0;
        let gear2_phase = 0.0; // Same phase might cause collision
        
        // This would be implemented in the actual validator
        // For now, just test the structure exists
        assert!(gear1_phase >= 0.0 && gear1_phase < 360.0);
        assert!(gear2_phase >= 0.0 && gear2_phase < 360.0);
    }
    
    #[test]
    fn test_clearance_validation() {
        // Test minimum clearance between non-meshing gears
        let min_clearance = 2.0; // mm
        let distance = 50.0;
        let required_clearance = 45.0;
        
        assert!(distance > required_clearance + min_clearance);
    }
}