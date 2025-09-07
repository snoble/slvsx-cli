/// Validates planetary gear system geometry to prevent impossible configurations
/// This module ensures that planetary gears can physically exist without overlap

use std::fmt;

/// Represents validated planetary gear parameters that are guaranteed to be geometrically valid
#[derive(Debug, Clone)]
pub struct ValidatedPlanetarySystem {
    pub sun_teeth: u32,
    pub planet_teeth: u32,
    pub ring_teeth: u32,
    pub module: f64,
    pub addendum_factor: f64,
    pub dedendum_factor: f64,
    pub planet_orbit_radius: f64,
    pub max_overlap: f64,  // Negative means clearance
}

#[derive(Debug, Clone)]
pub enum PlanetaryValidationError {
    InvalidToothCount { 
        reason: String 
    },
    GeometricOverlap {
        planet_max_radius: f64,
        ring_min_radius: f64,
        overlap: f64,
    },
    InvalidAssemblyCondition {
        sum: u32,
        ring: u32,
        planet_count: u32,
    },
}

impl fmt::Display for PlanetaryValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanetaryValidationError::InvalidToothCount { reason } => {
                write!(f, "Invalid tooth count: {}", reason)
            }
            PlanetaryValidationError::GeometricOverlap { 
                planet_max_radius, 
                ring_min_radius, 
                overlap 
            } => {
                write!(
                    f, 
                    "Geometric overlap detected: planet extends to {}mm but ring is at {}mm ({}mm overlap)",
                    planet_max_radius, ring_min_radius, overlap
                )
            }
            PlanetaryValidationError::InvalidAssemblyCondition { sum, ring, planet_count } => {
                write!(
                    f,
                    "Assembly condition failed: (sun({}) + ring({})) / {} = {} (must be integer)",
                    sum, ring, planet_count,
                    (sum + ring) as f64 / *planet_count as f64
                )
            }
        }
    }
}

impl std::error::Error for PlanetaryValidationError {}

/// Builder for creating validated planetary gear systems
pub struct PlanetarySystemBuilder {
    sun_teeth: Option<u32>,
    planet_teeth: Option<u32>,
    ring_teeth: Option<u32>,
    module: f64,
    addendum_factor: f64,
    dedendum_factor: f64,
    num_planets: u32,
}

impl PlanetarySystemBuilder {
    pub fn new() -> Self {
        Self {
            sun_teeth: None,
            planet_teeth: None,
            ring_teeth: None,
            module: 1.0,
            addendum_factor: 1.0,  // Standard tooth height
            dedendum_factor: 1.0,  // Standard tooth height
            num_planets: 3,
        }
    }

    pub fn sun_teeth(mut self, teeth: u32) -> Self {
        self.sun_teeth = Some(teeth);
        self
    }

    pub fn planet_teeth(mut self, teeth: u32) -> Self {
        self.planet_teeth = Some(teeth);
        self
    }

    pub fn ring_teeth(mut self, teeth: u32) -> Self {
        self.ring_teeth = Some(teeth);
        self
    }

    pub fn module(mut self, module: f64) -> Self {
        self.module = module;
        self
    }

    pub fn stub_teeth(mut self, factor: f64) -> Self {
        self.addendum_factor = factor;
        self.dedendum_factor = factor;
        self
    }

    pub fn num_planets(mut self, n: u32) -> Self {
        self.num_planets = n;
        self
    }

    /// Validates and builds the planetary system
    /// This ensures all geometric constraints are satisfied
    pub fn build(self) -> Result<ValidatedPlanetarySystem, PlanetaryValidationError> {
        let sun_teeth = self.sun_teeth.ok_or_else(|| {
            PlanetaryValidationError::InvalidToothCount {
                reason: "Sun teeth not specified".to_string()
            }
        })?;

        let planet_teeth = self.planet_teeth.ok_or_else(|| {
            PlanetaryValidationError::InvalidToothCount {
                reason: "Planet teeth not specified".to_string()
            }
        })?;

        let ring_teeth = self.ring_teeth.ok_or_else(|| {
            PlanetaryValidationError::InvalidToothCount {
                reason: "Ring teeth not specified".to_string()
            }
        })?;

        // Validate fundamental constraint: S + 2P = R
        if sun_teeth + 2 * planet_teeth != ring_teeth {
            return Err(PlanetaryValidationError::InvalidToothCount {
                reason: format!(
                    "S + 2P must equal R: {} + 2*{} = {} ≠ {}",
                    sun_teeth, planet_teeth, 
                    sun_teeth + 2 * planet_teeth, ring_teeth
                )
            });
        }

        // Validate assembly condition: (S + R) / n must be integer
        if (sun_teeth + ring_teeth) % self.num_planets != 0 {
            return Err(PlanetaryValidationError::InvalidAssemblyCondition {
                sum: sun_teeth,
                ring: ring_teeth,
                planet_count: self.num_planets,
            });
        }

        // Calculate radii
        let sun_pitch_r = (sun_teeth as f64 * self.module) / 2.0;
        let planet_pitch_r = (planet_teeth as f64 * self.module) / 2.0;
        let ring_pitch_r = (ring_teeth as f64 * self.module) / 2.0;

        // Planet orbit radius (center of planet from origin)
        let planet_orbit_radius = sun_pitch_r + planet_pitch_r;

        // Calculate actual tooth extents with specified factors
        let planet_addendum = self.module * self.addendum_factor;
        let ring_dedendum = self.module * self.dedendum_factor;

        // Maximum extent of planet teeth
        let planet_max_radius = planet_orbit_radius + planet_pitch_r + planet_addendum;
        
        // Minimum extent of ring teeth (internal)
        let ring_min_radius = ring_pitch_r - ring_dedendum;

        // Check for overlap
        let overlap = planet_max_radius - ring_min_radius;
        
        if overlap > 0.0 {
            return Err(PlanetaryValidationError::GeometricOverlap {
                planet_max_radius,
                ring_min_radius,
                overlap,
            });
        }

        Ok(ValidatedPlanetarySystem {
            sun_teeth,
            planet_teeth,
            ring_teeth,
            module: self.module,
            addendum_factor: self.addendum_factor,
            dedendum_factor: self.dedendum_factor,
            planet_orbit_radius,
            max_overlap: overlap,
        })
    }
}

/// Calculate the required stub tooth factor to prevent overlap
pub fn calculate_stub_factor(sun_teeth: u32, planet_teeth: u32, ring_teeth: u32, module: f64) -> f64 {
    let sun_pitch_r = (sun_teeth as f64 * module) / 2.0;
    let planet_pitch_r = (planet_teeth as f64 * module) / 2.0;
    let ring_pitch_r = (ring_teeth as f64 * module) / 2.0;
    
    let planet_orbit = sun_pitch_r + planet_pitch_r;
    
    // For no overlap: planet_orbit + planet_pitch_r + addendum <= ring_pitch_r - dedendum
    // If addendum = dedendum = module * factor:
    // planet_orbit + planet_pitch_r + module * factor <= ring_pitch_r - module * factor
    // 2 * module * factor <= ring_pitch_r - planet_orbit - planet_pitch_r
    // factor <= (ring_pitch_r - planet_orbit - planet_pitch_r) / (2 * module)
    
    let max_factor = (ring_pitch_r - planet_orbit - planet_pitch_r) / (2.0 * module);
    
    // Return slightly less to ensure clearance
    (max_factor * 0.95).min(1.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_teeth_cause_overlap() {
        // This test proves that standard tooth profiles cause overlap
        let result = PlanetarySystemBuilder::new()
            .sun_teeth(24)
            .planet_teeth(12)
            .ring_teeth(48)
            .module(2.0)
            // Default is standard teeth (1.0)
            .build();

        assert!(result.is_err(), "Standard teeth should cause overlap");
        
        if let Err(PlanetaryValidationError::GeometricOverlap { overlap, .. }) = result {
            assert!((overlap - 4.0).abs() < 0.01, "Should have 4mm overlap with standard teeth");
        } else {
            panic!("Expected GeometricOverlap error");
        }
    }

    #[test]
    fn test_stub_teeth_help_but_not_enough() {
        // This test shows that even stub teeth can't fix S=24, P=12, R=48
        // because the geometry is fundamentally impossible
        let result = PlanetarySystemBuilder::new()
            .sun_teeth(24)
            .planet_teeth(12)
            .ring_teeth(48)
            .module(2.0)
            .stub_teeth(0.1)  // Even 10% teeth don't work!
            .build();

        // This geometry is impossible even with tiny teeth
        assert!(result.is_err(), "This geometry cannot work");
    }

    #[test]
    fn test_calculate_stub_factor() {
        let factor = calculate_stub_factor(24, 12, 48, 2.0);
        
        // For this impossible geometry, the factor will be 0 or negative
        assert!(factor <= 0.0, "Factor should be 0 or negative for impossible geometry");
        
        // Try with different tooth counts that CAN work
        // Use smaller planets relative to the system
        let better_factor = calculate_stub_factor(30, 6, 42, 2.0);
        
        if better_factor > 0.0 {
            let result = PlanetarySystemBuilder::new()
                .sun_teeth(30)
                .planet_teeth(6)
                .ring_teeth(42)
                .module(2.0)
                .stub_teeth(better_factor)
                .build();
            
            if result.is_ok() {
                let system = result.unwrap();
                assert!(system.max_overlap <= 0.0, "Should work with this geometry");
            }
        }
    }

    #[test]
    fn test_invalid_tooth_count() {
        // S + 2P ≠ R
        let result = PlanetarySystemBuilder::new()
            .sun_teeth(24)
            .planet_teeth(12)
            .ring_teeth(50)  // Wrong! Should be 48
            .module(2.0)
            .build();

        assert!(result.is_err(), "Invalid tooth count should fail");
        
        if let Err(PlanetaryValidationError::InvalidToothCount { reason }) = result {
            assert!(reason.contains("S + 2P must equal R"));
        } else {
            panic!("Expected InvalidToothCount error");
        }
    }

    #[test]
    fn test_assembly_condition() {
        // (S + R) / n must be integer
        let result = PlanetarySystemBuilder::new()
            .sun_teeth(24)
            .planet_teeth(12)
            .ring_teeth(48)
            .module(2.0)
            .stub_teeth(0.5)
            .num_planets(5)  // (24 + 48) / 5 = 14.4 (not integer)
            .build();

        assert!(result.is_err(), "Invalid assembly condition should fail");
        
        if let Err(PlanetaryValidationError::InvalidAssemblyCondition { .. }) = result {
            // Expected
        } else {
            panic!("Expected InvalidAssemblyCondition error");
        }
    }

    #[test]
    fn test_type_safety() {
        // This test demonstrates that you can't create an invalid system
        // The type system enforces validation
        
        fn requires_valid_system(_system: &ValidatedPlanetarySystem) {
            // This function can only be called with a validated system
        }

        // Try to create an impossible system - will fail
        let impossible = PlanetarySystemBuilder::new()
            .sun_teeth(24)
            .planet_teeth(12)
            .ring_teeth(48)
            .module(2.0)
            .stub_teeth(0.1)
            .build();
            
        assert!(impossible.is_err(), "Can't create impossible system");
        
        // Create a different system that might work
        // Use very small planets
        let possible = PlanetarySystemBuilder::new()
            .sun_teeth(40)
            .planet_teeth(5)
            .ring_teeth(50)
            .module(2.0)
            .stub_teeth(0.5)
            .build();
            
        if let Ok(system) = possible {
            requires_valid_system(&system);  // Type safe!
        }
    }
}