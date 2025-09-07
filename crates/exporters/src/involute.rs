use std::f64::consts::PI;

/// Generate points for an involute curve
pub struct InvoluteGenerator {
    pub base_radius: f64,
    pub pitch_radius: f64,
    pub addendum: f64,
    pub dedendum: f64,
    pub pressure_angle_rad: f64,
}

impl InvoluteGenerator {
    pub fn new(teeth: u32, module: f64, pressure_angle_deg: f64) -> Self {
        let pitch_radius = (teeth as f64 * module) / 2.0;
        let pressure_angle_rad = pressure_angle_deg.to_radians();
        let base_radius = pitch_radius * pressure_angle_rad.cos();
        
        Self {
            base_radius,
            pitch_radius,
            addendum: module,
            dedendum: 1.25 * module,
            pressure_angle_rad,
        }
    }
    
    /// Generate a single involute curve from base circle to tip
    pub fn generate_involute(&self, num_points: usize) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        
        let outer_radius = self.pitch_radius + self.addendum;
        let inner_radius = self.pitch_radius - self.dedendum;
        
        // Start angle where involute begins (at base circle)
        let start_angle = if self.base_radius < inner_radius {
            // Involute starts at root circle
            self.angle_at_radius(inner_radius)
        } else {
            0.0
        };
        
        // End angle where involute reaches tip
        let end_angle = self.angle_at_radius(outer_radius);
        
        // Generate points along involute
        for i in 0..num_points {
            let t = start_angle + (end_angle - start_angle) * (i as f64) / (num_points as f64 - 1.0);
            let (x, y) = self.involute_point(t);
            points.push((x, y));
        }
        
        points
    }
    
    /// Calculate involute curve point at parameter t
    fn involute_point(&self, t: f64) -> (f64, f64) {
        let x = self.base_radius * (t.cos() + t * t.sin());
        let y = self.base_radius * (t.sin() - t * t.cos());
        (x, y)
    }
    
    /// Find the angle parameter t for a given radius
    fn angle_at_radius(&self, radius: f64) -> f64 {
        if radius <= self.base_radius {
            return 0.0;
        }
        
        // For involute: r = rb * sqrt(1 + t^2)
        // Solving for t: t = sqrt((r/rb)^2 - 1)
        let ratio = radius / self.base_radius;
        (ratio * ratio - 1.0).sqrt()
    }
    
    /// Generate a complete gear tooth profile
    pub fn generate_tooth(&self, tooth_thickness_angle: f64) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        
        // Generate right side of tooth (involute)
        let right_involute = self.generate_involute(10);
        
        // Calculate rotation to position involute correctly
        let pitch_point_angle = self.angle_at_radius(self.pitch_radius);
        let involute_angle_at_pitch = pitch_point_angle.atan();
        let half_tooth_angle = tooth_thickness_angle / 2.0;
        let rotation = half_tooth_angle - involute_angle_at_pitch;
        
        // Add right side (rotated)
        for (x, y) in &right_involute {
            let (rx, ry) = rotate_point(*x, *y, rotation);
            points.push((rx, ry));
        }
        
        // Add left side (mirror and rotate)
        for (x, y) in right_involute.iter().rev() {
            let (mx, my) = (*x, -*y);  // Mirror across x-axis
            let (rx, ry) = rotate_point(mx, my, -rotation);
            points.push((rx, ry));
        }
        
        points
    }
}

/// Rotate a point by angle (in radians)
fn rotate_point(x: f64, y: f64, angle: f64) -> (f64, f64) {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    (
        x * cos_a - y * sin_a,
        x * sin_a + y * cos_a
    )
}

/// Generate a complete gear profile with involute teeth
pub fn generate_gear_profile(
    teeth: u32,
    module: f64,
    pressure_angle_deg: f64,
    internal: bool,
) -> Vec<(f64, f64)> {
    let generator = InvoluteGenerator::new(teeth, module, pressure_angle_deg);
    let mut points = Vec::new();
    
    let tooth_angle = 2.0 * PI / teeth as f64;
    let tooth_thickness_angle = tooth_angle * 0.5;  // 50% tooth, 50% space
    
    let inner_radius = generator.pitch_radius - generator.dedendum;
    let outer_radius = generator.pitch_radius + generator.addendum;
    
    for i in 0..teeth {
        let base_angle = i as f64 * tooth_angle;
        
        if internal {
            // For internal gears, teeth point inward
            // Start with tooth space
            let space_start = base_angle - tooth_thickness_angle * 0.5;
            let space_end = base_angle + tooth_thickness_angle * 0.5;
            
            // Outer arc (root of internal gear)
            for j in 0..5 {
                let angle = space_start + (space_end - space_start) * (j as f64) / 4.0;
                points.push((
                    outer_radius * angle.cos(),
                    outer_radius * angle.sin()
                ));
            }
            
            // Inner points (tips of internal gear teeth)
            let tooth_start = base_angle + tooth_thickness_angle * 0.5;
            let tooth_end = tooth_start + tooth_angle - tooth_thickness_angle;
            
            for j in 0..5 {
                let angle = tooth_start + (tooth_end - tooth_start) * (j as f64) / 4.0;
                points.push((
                    inner_radius * angle.cos(),
                    inner_radius * angle.sin()
                ));
            }
        } else {
            // External gear - use involute profiles
            let tooth = generator.generate_tooth(tooth_thickness_angle);
            
            // Transform tooth to correct position
            for (x, y) in tooth {
                let (rx, ry) = rotate_point(x, y, base_angle);
                points.push((rx, ry));
            }
            
            // Add root circle arc to next tooth
            if i < teeth - 1 {
                let arc_start = base_angle + tooth_thickness_angle * 0.5;
                let arc_end = base_angle + tooth_angle - tooth_thickness_angle * 0.5;
                
                for j in 1..5 {
                    let angle = arc_start + (arc_end - arc_start) * (j as f64) / 4.0;
                    points.push((
                        inner_radius * angle.cos(),
                        inner_radius * angle.sin()
                    ));
                }
            }
        }
    }
    
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_involute_generator() {
        let gen = InvoluteGenerator::new(24, 2.0, 20.0);
        assert!((gen.pitch_radius - 24.0).abs() < 0.001);
        
        let points = gen.generate_involute(10);
        assert_eq!(points.len(), 10);
        
        // Points should increase in radius
        for i in 1..points.len() {
            let r1 = (points[i-1].0.powi(2) + points[i-1].1.powi(2)).sqrt();
            let r2 = (points[i].0.powi(2) + points[i].1.powi(2)).sqrt();
            assert!(r2 >= r1, "Radius should increase along involute");
        }
    }
    
    #[test]
    fn test_gear_profile() {
        let profile = generate_gear_profile(12, 2.0, 20.0, false);
        assert!(!profile.is_empty());
        
        // Check that points are roughly at expected radii
        for (x, y) in &profile {
            let r = (x.powi(2) + y.powi(2)).sqrt();
            assert!(r >= 10.5 && r <= 13.0, "Points should be within gear bounds");
        }
    }
}