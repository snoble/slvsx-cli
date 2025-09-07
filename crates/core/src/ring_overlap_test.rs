#[cfg(test)]
mod tests {
    use crate::phase_validator::{validate_phase_solution, GearData};
    use std::collections::HashMap;

    #[test]
    fn test_ring_planet_overlap_detection() {
        // This test verifies that the validator correctly detects 
        // when planets extend beyond the ring's inner teeth
        
        let mut gears = HashMap::new();
        
        // Ring gear at origin - internal teeth
        gears.insert("ring".to_string(), GearData {
            id: "ring".to_string(),
            center: [0.0, 0.0],
            teeth: 48,
            module: 2.0,
            phase: 0.0,
            internal: true,
        });
        
        // Planet at 36mm from origin - should overlap with ring
        // With clearance: planet extends to 36 + 13.3 = 49.3mm
        // Ring inner teeth at 46.7mm - OVERLAP!
        gears.insert("planet1".to_string(), GearData {
            id: "planet1".to_string(),
            center: [36.0, 0.0],
            teeth: 12,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        // Sun at origin
        gears.insert("sun".to_string(), GearData {
            id: "sun".to_string(),
            center: [0.0, 0.0],
            teeth: 24,
            module: 2.0,
            phase: 0.0,
            internal: false,
        });
        
        let mesh_constraints = vec![
            ("sun".to_string(), "planet1".to_string()),
            ("ring".to_string(), "planet1".to_string()),
        ];
        
        // Should fail validation due to ring-planet overlap
        let result = validate_phase_solution(&gears, &mesh_constraints);
        assert!(result.is_err(), "Should detect ring-planet overlap");
        
        if let Err(overlaps) = result {
            let ring_overlap = overlaps.iter().find(|o| 
                (o.gear1 == "ring" && o.gear2 == "planet1") ||
                (o.gear1 == "planet1" && o.gear2 == "ring")
            );
            assert!(ring_overlap.is_some(), "Should specifically detect ring-planet overlap");
            
            if let Some(overlap) = ring_overlap {
                // Planet at 36mm should be at most 33.4mm for no overlap
                assert!(overlap.min_safe_distance < 34.0, 
                    "Should calculate correct safe distance: {:?}", overlap);
            }
        }
    }

    #[test]
    fn test_correct_planetary_distances() {
        // Test that validates the fundamental constraint:
        // sun_teeth + 2*planet_teeth = ring_teeth
        
        let sun_teeth = 24;
        let planet_teeth = 12;
        let ring_teeth = 48;
        let module = 2.0;
        
        // Check fundamental relationship
        assert_eq!(sun_teeth + 2 * planet_teeth, ring_teeth,
            "Planetary gear constraint must be satisfied");
        
        // Calculate radii
        let sun_pitch_r = (sun_teeth as f64 * module) / 2.0;
        let planet_pitch_r = (planet_teeth as f64 * module) / 2.0;
        let ring_pitch_r = (ring_teeth as f64 * module) / 2.0;
        
        // Planet orbit radius
        let orbit_r = sun_pitch_r + planet_pitch_r;
        
        // Check that planet meshes with both sun and ring at same orbit
        assert_eq!(orbit_r, 36.0, "Planet orbit should be 36mm");
        assert_eq!(ring_pitch_r - planet_pitch_r, orbit_r,
            "Planet should mesh with ring at same orbit radius");
    }

    #[test]
    fn test_tooth_clearance_creates_overlap() {
        // This test demonstrates that 3D printing clearance
        // can create geometric impossibilities
        
        let module = 2.0;
        let clearance = 0.7;
        
        // Ring: 48 teeth, pitch radius 48mm
        let ring_pitch_r = 48.0;
        let ring_tooth_inner = ring_pitch_r - module + clearance; // 46.7mm
        
        // Planet: 12 teeth, pitch radius 12mm, at 36mm from origin
        let planet_pitch_r = 12.0;
        let planet_orbit = 36.0;
        let planet_tooth_outer = planet_pitch_r + module - clearance; // 13.3mm
        
        // Planet's outermost point
        let planet_max_r = planet_orbit + planet_tooth_outer; // 49.3mm
        
        // Check for overlap
        assert!(planet_max_r > ring_tooth_inner,
            "Planet teeth ({:.1}mm) extend beyond ring teeth ({:.1}mm) - OVERLAP!",
            planet_max_r, ring_tooth_inner);
        
        let overlap = planet_max_r - ring_tooth_inner;
        assert!((overlap - 2.6f64).abs() < 0.1, 
            "Overlap should be approximately 2.6mm, got {:.1}mm", overlap);
    }
}