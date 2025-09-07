use std::f64::consts::PI;

/// Gear geometry calculations for proper meshing
pub struct GearGeometry {
    pub module: f64,           // Gear module (tooth size)
    pub pressure_angle: f64,   // Standard 20 degrees
}

impl Default for GearGeometry {
    fn default() -> Self {
        Self {
            module: 2.0,
            pressure_angle: 20.0 * PI / 180.0,  // Convert to radians
        }
    }
}

impl GearGeometry {
    pub fn new(module: f64) -> Self {
        Self {
            module,
            pressure_angle: 20.0 * PI / 180.0,
        }
    }
    
    /// Calculate number of teeth from pitch diameter
    pub fn teeth_from_diameter(&self, diameter: f64) -> u32 {
        (diameter / self.module).round() as u32
    }
    
    /// Calculate pitch diameter from number of teeth
    pub fn diameter_from_teeth(&self, teeth: u32) -> f64 {
        teeth as f64 * self.module
    }
    
    /// Calculate base circle diameter (for involute profile)
    pub fn base_diameter(&self, pitch_diameter: f64) -> f64 {
        pitch_diameter * self.pressure_angle.cos()
    }
    
    /// Calculate addendum (tooth height above pitch circle)
    pub fn addendum(&self) -> f64 {
        self.module
    }
    
    /// Calculate dedendum (tooth depth below pitch circle)
    pub fn dedendum(&self) -> f64 {
        1.25 * self.module
    }
    
    /// Calculate outer diameter
    pub fn outer_diameter(&self, pitch_diameter: f64) -> f64 {
        pitch_diameter + 2.0 * self.addendum()
    }
    
    /// Calculate root diameter
    pub fn root_diameter(&self, pitch_diameter: f64) -> f64 {
        pitch_diameter - 2.0 * self.dedendum()
    }
    
    /// Calculate rotation needed for gear meshing
    /// When two gears mesh, if one tooth is centered at angle 0,
    /// the mating gear's tooth valley must align
    pub fn mesh_phase_offset(&self, teeth1: u32, teeth2: u32, is_internal: bool) -> f64 {
        let tooth_angle2 = 2.0 * PI / teeth2 as f64;
        
        if is_internal {
            // For internal gears (ring), teeth point inward
            // No phase offset needed if both have even teeth
            if teeth1 % 2 == 0 && teeth2 % 2 == 0 {
                0.0
            } else {
                tooth_angle2 / 2.0
            }
        } else {
            // For external gears, tooth meets valley
            // Half tooth offset for proper meshing
            tooth_angle2 / 2.0
        }
    }
    
    /// Generate involute tooth profile points
    pub fn involute_tooth_profile(&self, pitch_radius: f64, num_points: usize) -> Vec<(f64, f64)> {
        let base_radius = pitch_radius * self.pressure_angle.cos();
        let outer_radius = pitch_radius + self.addendum();
        let root_radius = pitch_radius - self.dedendum();
        
        let mut points = Vec::new();
        
        // Generate involute curve from base circle to outer circle
        for i in 0..num_points {
            let t = i as f64 / (num_points - 1) as f64;
            let radius = base_radius + t * (outer_radius - base_radius);
            
            if radius >= base_radius {
                // Involute parametric equations
                let phi = (radius / base_radius).acos();
                let involute_angle = phi.tan() - phi;
                
                let x = radius * involute_angle.cos();
                let y = radius * involute_angle.sin();
                points.push((x, y));
            }
        }
        
        points
    }
    
    /// Generate complete tooth profile for a gear
    pub fn generate_gear_teeth(&self, pitch_diameter: f64, center_x: f64, center_y: f64, phase: f64) -> Vec<Vec<(f64, f64)>> {
        let teeth = self.teeth_from_diameter(pitch_diameter);
        let pitch_radius = pitch_diameter / 2.0;
        let outer_radius = pitch_radius + self.addendum();
        let root_radius = pitch_radius - self.dedendum();
        
        let tooth_angle = 2.0 * PI / teeth as f64;
        let tooth_thickness_angle = tooth_angle * 0.5; // Simplified - equal tooth and space
        
        let mut all_teeth = Vec::new();
        
        for i in 0..teeth {
            let base_angle = i as f64 * tooth_angle + phase;
            let mut tooth_points = Vec::new();
            
            // Simplified tooth profile (trapezoidal)
            // Start at root circle
            let angle1 = base_angle - tooth_thickness_angle * 0.4;
            let angle2 = base_angle - tooth_thickness_angle * 0.3;
            let angle3 = base_angle + tooth_thickness_angle * 0.3;
            let angle4 = base_angle + tooth_thickness_angle * 0.4;
            
            // Root circle points
            tooth_points.push((
                center_x + root_radius * angle1.cos(),
                center_y + root_radius * angle1.sin(),
            ));
            
            // Outer circle points (tooth tip)
            tooth_points.push((
                center_x + outer_radius * angle2.cos(),
                center_y + outer_radius * angle2.sin(),
            ));
            tooth_points.push((
                center_x + outer_radius * angle3.cos(),
                center_y + outer_radius * angle3.sin(),
            ));
            
            // Back to root
            tooth_points.push((
                center_x + root_radius * angle4.cos(),
                center_y + root_radius * angle4.sin(),
            ));
            
            all_teeth.push(tooth_points);
        }
        
        all_teeth
    }
}

/// Calculate proper positions for planetary gears
pub struct PlanetaryLayout {
    pub sun_teeth: u32,
    pub planet_teeth: u32,
    pub ring_teeth: u32,
    pub num_planets: u32,
    pub module: f64,
}

impl PlanetaryLayout {
    pub fn new(sun_teeth: u32, planet_teeth: u32, ring_teeth: u32, num_planets: u32, module: f64) -> Self {
        Self {
            sun_teeth,
            planet_teeth,
            ring_teeth,
            num_planets,
            module,
        }
    }
    
    /// Validate gear ratio constraints
    pub fn validate(&self) -> bool {
        // For assembly, (sun_teeth + ring_teeth) must be divisible by num_planets
        (self.sun_teeth + self.ring_teeth) % self.num_planets == 0
    }
    
    /// Calculate carrier radius (distance from sun center to planet centers)
    pub fn carrier_radius(&self) -> f64 {
        let sun_radius = self.sun_teeth as f64 * self.module / 2.0;
        let planet_radius = self.planet_teeth as f64 * self.module / 2.0;
        sun_radius + planet_radius
    }
    
    /// Calculate planet positions
    pub fn planet_positions(&self) -> Vec<(f64, f64)> {
        let carrier_r = self.carrier_radius();
        let mut positions = Vec::new();
        
        for i in 0..self.num_planets {
            let angle = 2.0 * PI * i as f64 / self.num_planets as f64;
            positions.push((
                carrier_r * angle.cos(),
                carrier_r * angle.sin(),
            ));
        }
        
        positions
    }
    
    /// Calculate phase angles for proper meshing
    pub fn calculate_mesh_phases(&self) -> PlanetaryPhases {
        let sun_planet_mesh_phase = PI / self.planet_teeth as f64;
        
        // Each planet needs to mesh with sun at its position
        let mut planet_phases = Vec::new();
        for i in 0..self.num_planets {
            let carrier_angle = 2.0 * PI * i as f64 / self.num_planets as f64;
            // Planet rotation to mesh with sun
            let planet_phase = -carrier_angle * (self.sun_teeth as f64 / self.planet_teeth as f64);
            planet_phases.push(planet_phase);
        }
        
        PlanetaryPhases {
            sun_phase: 0.0,
            planet_phases,
            ring_phase: 0.0,
        }
    }
}

pub struct PlanetaryPhases {
    pub sun_phase: f64,
    pub planet_phases: Vec<f64>,
    pub ring_phase: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gear_geometry_default() {
        let geom = GearGeometry::default();
        assert_eq!(geom.module, 2.0);
        assert!((geom.pressure_angle - 20.0 * PI / 180.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_teeth_from_diameter() {
        let geom = GearGeometry::new(2.0);
        assert_eq!(geom.teeth_from_diameter(48.0), 24);
        assert_eq!(geom.teeth_from_diameter(24.0), 12);
        assert_eq!(geom.teeth_from_diameter(36.0), 18);
    }
    
    #[test]
    fn test_diameter_from_teeth() {
        let geom = GearGeometry::new(2.0);
        assert_eq!(geom.diameter_from_teeth(24), 48.0);
        assert_eq!(geom.diameter_from_teeth(12), 24.0);
    }
    
    #[test]
    fn test_addendum_dedendum() {
        let geom = GearGeometry::new(2.0);
        assert_eq!(geom.addendum(), 2.0);
        assert_eq!(geom.dedendum(), 2.5);
    }
    
    #[test]
    fn test_outer_root_diameter() {
        let geom = GearGeometry::new(2.0);
        assert_eq!(geom.outer_diameter(48.0), 52.0);
        assert_eq!(geom.root_diameter(48.0), 43.0);
    }
    
    #[test]
    fn test_planetary_layout() {
        let layout = PlanetaryLayout::new(24, 12, 72, 6, 2.0);
        assert!(layout.validate());
        assert_eq!(layout.carrier_radius(), 36.0);
        
        let positions = layout.planet_positions();
        assert_eq!(positions.len(), 6);
        assert!((positions[0].0 - 36.0).abs() < 1e-10);
        assert!(positions[0].1.abs() < 1e-10);
    }
    
    #[test]
    fn test_generate_gear_teeth() {
        let geom = GearGeometry::new(2.0);
        let teeth = geom.generate_gear_teeth(48.0, 0.0, 0.0, 0.0);
        assert_eq!(teeth.len(), 24); // 24 teeth
        assert_eq!(teeth[0].len(), 4); // 4 points per tooth (simplified)
    }
}