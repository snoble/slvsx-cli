#[cfg(test)]
mod tests {
    use crate::planetary_validator::{PlanetarySystemBuilder, calculate_stub_factor};
    use crate::phase_validator::{validate_phase_solution, GearData};
    use crate::distance_validator::{validate_all_distances, check_critical_distances};
    use std::collections::HashMap;

    /// This test captures the exact bug that was discovered:
    /// Standard planetary gear formulas give correct pitch circles
    /// but with standard tooth profiles there's inherent overlap
    #[test]
    fn test_prevent_regression_standard_teeth_overlap() {
        // Create a planetary system with standard teeth
        let mut gears = HashMap::new();
        
        // Standard module and teeth
        let module = 2.0;
        let sun_teeth = 24;
        let planet_teeth = 12;
        let ring_teeth = 48;
        
        // Verify the fundamental formula
        assert_eq!(
            sun_teeth + 2 * planet_teeth, 
            ring_teeth,
            "S + 2P = R must hold"
        );
        
        // Calculate positions
        let sun_pitch_r = (sun_teeth as f64 * module) / 2.0;
        let planet_pitch_r = (planet_teeth as f64 * module) / 2.0;
        let planet_orbit = sun_pitch_r + planet_pitch_r;  // 36mm
        
        // Add gears with STANDARD tooth heights
        gears.insert("sun".to_string(), GearData {
            id: "sun".to_string(),
            center: [0.0, 0.0],
            teeth: sun_teeth,
            module,
            phase: 0.0,
            internal: false,
        });
        
        gears.insert("planet1".to_string(), GearData {
            id: "planet1".to_string(),
            center: [planet_orbit, 0.0],
            teeth: planet_teeth,
            module,
            phase: 15.0,  // Some phase offset
            internal: false,
        });
        
        gears.insert("ring".to_string(), GearData {
            id: "ring".to_string(),
            center: [0.0, 0.0],
            teeth: ring_teeth,
            module,
            phase: 0.0,
            internal: true,
        });
        
        let mesh_constraints = vec![
            ("sun".to_string(), "planet1".to_string()),
            ("ring".to_string(), "planet1".to_string()),
        ];
        
        // Validate with our improved validator
        let result = validate_phase_solution(&gears, &mesh_constraints);
        
        // The validator MUST detect the overlap
        assert!(
            result.is_err(),
            "Validator must detect overlap with standard teeth! This was the bug."
        );
        
        if let Err(overlaps) = result {
            // Find the ring-planet overlap
            let ring_overlap = overlaps.iter().find(|o|
                (o.gear1 == "ring" && o.gear2 == "planet1") ||
                (o.gear1 == "planet1" && o.gear2 == "ring")
            );
            
            assert!(
                ring_overlap.is_some(),
                "Must specifically detect ring-planet overlap"
            );
            
            // The overlap should be approximately 2.6mm with 3D printing clearance
            // or 4mm without clearance
        }
    }

    #[test]
    fn test_planetary_builder_prevents_invalid_systems() {
        // This test ensures the type system prevents creating invalid planetary gears
        
        // Attempt to create with standard teeth - should fail
        let standard_result = PlanetarySystemBuilder::new()
            .sun_teeth(24)
            .planet_teeth(12)
            .ring_teeth(48)
            .module(2.0)
            // Default is standard teeth (factor = 1.0)
            .build();
        
        assert!(
            standard_result.is_err(),
            "Builder must reject standard teeth due to overlap"
        );
        
        // Even stub teeth can't save this geometry
        let stub_result = PlanetarySystemBuilder::new()
            .sun_teeth(24)
            .planet_teeth(12)
            .ring_teeth(48)
            .module(2.0)
            .stub_teeth(0.1)  // Even tiny stub teeth
            .build();
        
        assert!(
            stub_result.is_err(),
            "Even stub teeth can't fix impossible geometry"
        );
    }

    #[test]
    fn test_automatic_stub_factor_calculation() {
        // Test that we detect impossible geometry
        let factor = calculate_stub_factor(24, 12, 48, 2.0);
        
        // The factor should be 0 or negative for impossible geometry
        assert!(
            factor <= 0.0,
            "Must return 0 or negative for impossible geometry"
        );
        
        // Can't build with negative factor
        if factor <= 0.0 {
            // This geometry is impossible
            let result = PlanetarySystemBuilder::new()
                .sun_teeth(24)
                .planet_teeth(12)
                .ring_teeth(48)
                .module(2.0)
                .stub_teeth(0.01)  // Even tiny teeth
                .build();
                
            assert!(result.is_err(), "Impossible geometry can't be built");
        }
    }

    #[test]
    fn test_distance_validation_integration() {
        // Integration test with distance validator
        let mut gears = HashMap::new();
        
        // Use stub teeth configuration
        let stub_factor = 0.5;
        let module = 2.0;
        
        gears.insert("sun".to_string(), GearData {
            id: "sun".to_string(),
            center: [0.0, 0.0],
            teeth: 24,
            module,
            phase: 0.0,
            internal: false,
        });
        
        gears.insert("planet1".to_string(), GearData {
            id: "planet1".to_string(),
            center: [36.0, 0.0],  // Correct distance
            teeth: 12,
            module,
            phase: 15.0,
            internal: false,
        });
        
        gears.insert("planet2".to_string(), GearData {
            id: "planet2".to_string(),
            center: [35.0, 0.0],  // WRONG distance!
            teeth: 12,
            module,
            phase: 15.0,
            internal: false,
        });
        
        let mesh_constraints = vec![
            ("sun".to_string(), "planet1".to_string()),
            ("sun".to_string(), "planet2".to_string()),
        ];
        
        // Distance validator should catch the error
        let validations = validate_all_distances(&gears, &mesh_constraints, 0.1);
        let result = check_critical_distances(&validations);
        
        assert!(
            result.is_err(),
            "Distance validator must detect incorrect planet position"
        );
        
        if let Err(errors) = result {
            assert!(
                errors.iter().any(|e| e.contains("planet2")),
                "Must identify planet2 as having wrong distance"
            );
        }
    }

    /// This test documents the fundamental issue and solution
    #[test]
    fn test_document_the_deep_bug() {
        // The "deep bug" discovered:
        // 1. Standard formula S + 2P = R gives correct pitch relationships
        // 2. But with standard addendum/dedendum = module, teeth physically overlap
        // 3. This is NOT a bug in the formula, but a physical constraint
        // 4. Real planetary gears use profile shifting or stub teeth
        
        let module = 2.0;
        
        // Standard geometry
        let sun_pitch = 24.0;  // 24 teeth * 2mm / 2
        let planet_pitch = 12.0;  // 12 teeth * 2mm / 2  
        let ring_pitch = 48.0;  // 48 teeth * 2mm / 2
        
        let planet_orbit = sun_pitch + planet_pitch;  // 36mm
        
        // With standard teeth (addendum = module = 2mm)
        let planet_outer = planet_pitch + module;  // 14mm from planet center
        let ring_inner = ring_pitch - module;  // 46mm from ring center
        
        let planet_max_reach = planet_orbit + planet_outer;  // 50mm
        
        // The overlap
        let overlap = planet_max_reach - ring_inner;  // 4mm
        
        assert_eq!(overlap, 4.0, "Standard teeth have 4mm overlap");
        
        // The solution: stub teeth with 50% height
        let stub_addendum = module * 0.5;  // 1mm instead of 2mm
        let planet_outer_stub = planet_pitch + stub_addendum;  // 13mm
        let ring_inner_stub = ring_pitch - stub_addendum;  // 47mm
        
        let planet_max_reach_stub = planet_orbit + planet_outer_stub;  // 49mm
        let clearance = ring_inner_stub - planet_max_reach_stub;  // -2mm (still overlap!)
        
        // Even 50% stub teeth aren't enough! Need more reduction
        let required_factor = (ring_pitch - planet_orbit - planet_pitch) / (2.0 * module);
        assert!(required_factor < 0.5, "Need even smaller teeth than 50% stub");
    }
}