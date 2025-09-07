use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct GearParameters {
    pub teeth: u32,
    pub module: f64,
    pub pressure_angle: f64,  // in degrees
    pub center: [f64; 2],
    pub phase: f64,  // rotation offset in degrees
    pub internal: bool,
}

impl GearParameters {
    pub fn pitch_radius(&self) -> f64 {
        (self.teeth as f64 * self.module) / 2.0
    }
    
    pub fn base_radius(&self) -> f64 {
        self.pitch_radius() * self.pressure_angle.to_radians().cos()
    }
    
    pub fn addendum(&self) -> f64 {
        self.module
    }
    
    pub fn dedendum(&self) -> f64 {
        1.25 * self.module
    }
    
    pub fn outer_radius(&self) -> f64 {
        if self.internal {
            self.pitch_radius() - self.addendum()
        } else {
            self.pitch_radius() + self.addendum()
        }
    }
    
    pub fn root_radius(&self) -> f64 {
        if self.internal {
            self.pitch_radius() + self.dedendum()
        } else {
            self.pitch_radius() - self.dedendum()
        }
    }
}

pub fn generate_involute_point(base_radius: f64, t: f64) -> (f64, f64) {
    // Involute curve: x = r(cos(t) + t*sin(t)), y = r(sin(t) - t*cos(t))
    let x = base_radius * (t.cos() + t * t.sin());
    let y = base_radius * (t.sin() - t * t.cos());
    (x, y)
}

pub fn generate_gear_svg_path(params: &GearParameters) -> String {
    let mut path = String::new();
    let outer_r = params.outer_radius();
    let root_r = params.root_radius();
    
    // Add clearance for 3D printing - teeth don't reach full height
    // CRITICAL: Must have enough clearance to prevent ANY overlap
    let clearance = 0.7; // mm of clearance for 3D printing (increased to prevent fusion)
    let tip_r = outer_r - clearance; // Pull tips back for clearance
    let root_clearance_r = root_r + clearance * 0.3; // Clearance at root too
    
    let tooth_angle = 2.0 * PI / params.teeth as f64;
    let tooth_thickness = tooth_angle * 0.2; // Narrower teeth for maximum clearance
    
    // Start path
    path.push_str("M ");
    
    for i in 0..params.teeth {
        let angle_offset = i as f64 * tooth_angle + params.phase.to_radians();
        
        if params.internal {
            // Internal gear - teeth point inward with clearance
            // For internal gears, we need to add clearance outward (larger radius)
            let internal_root_r = root_r - clearance * 0.5; // Move root inward slightly
            let internal_tip_r = outer_r + clearance; // Tips don't reach as far in
            
            // Start with gap at outer radius
            let gap_start = angle_offset - tooth_thickness * 0.8;
            let gap_end = angle_offset + tooth_thickness * 0.8;
            
            if i == 0 {
                let x = params.center[0] + internal_root_r * gap_start.cos();
                let y = params.center[1] + internal_root_r * gap_start.sin();
                path.push_str(&format!("{:.2} {:.2} ", x, y));
            }
            
            // Arc along outer radius (gap)
            let gap_end_x = params.center[0] + internal_root_r * gap_end.cos();
            let gap_end_y = params.center[1] + internal_root_r * gap_end.sin();
            
            path.push_str(&format!("A {:.2} {:.2} 0 0 1 {:.2} {:.2} ", 
                internal_root_r, internal_root_r, gap_end_x, gap_end_y));
            
            // Point inward to tooth tip (with clearance)
            let tooth_tip_angle = angle_offset + tooth_angle * 0.5;
            let tip_x = params.center[0] + internal_tip_r * tooth_tip_angle.cos();
            let tip_y = params.center[1] + internal_tip_r * tooth_tip_angle.sin();
            path.push_str(&format!("L {:.2} {:.2} ", tip_x, tip_y));
            
            // Back to outer radius
            let next_gap_start = angle_offset + tooth_angle - tooth_thickness * 0.8;
            let next_x = params.center[0] + internal_root_r * next_gap_start.cos();
            let next_y = params.center[1] + internal_root_r * next_gap_start.sin();
            path.push_str(&format!("L {:.2} {:.2} ", next_x, next_y));
        } else {
            // External gear - triangular teeth with clearance for 3D printing
            // Root point (with clearance)
            let root_angle = angle_offset - tooth_thickness * 0.5;
            let root_x = params.center[0] + root_clearance_r * root_angle.cos();
            let root_y = params.center[1] + root_clearance_r * root_angle.sin();
            
            if i == 0 {
                path.push_str(&format!("{:.2} {:.2} ", root_x, root_y));
            } else {
                path.push_str(&format!("L {:.2} {:.2} ", root_x, root_y));
            }
            
            // Tip point (pulled back for clearance)
            let tip_x = params.center[0] + tip_r * angle_offset.cos();
            let tip_y = params.center[1] + tip_r * angle_offset.sin();
            path.push_str(&format!("L {:.2} {:.2} ", tip_x, tip_y));
            
            // Next root point
            let next_root_angle = angle_offset + tooth_thickness * 0.5;
            let next_root_x = params.center[0] + root_clearance_r * next_root_angle.cos();
            let next_root_y = params.center[1] + root_clearance_r * next_root_angle.sin();
            path.push_str(&format!("L {:.2} {:.2} ", next_root_x, next_root_y));
            
            // Arc to next tooth
            if i < params.teeth - 1 {
                let arc_end_angle = angle_offset + tooth_angle - tooth_thickness * 0.5;
                let arc_end_x = params.center[0] + root_clearance_r * arc_end_angle.cos();
                let arc_end_y = params.center[1] + root_clearance_r * arc_end_angle.sin();
                path.push_str(&format!("A {:.2} {:.2} 0 0 1 {:.2} {:.2} ", 
                    root_clearance_r, root_clearance_r, arc_end_x, arc_end_y));
            }
        }
    }
    
    path.push_str("Z");
    path
}

pub fn generate_gear_svg(params: &GearParameters) -> String {
    let path = generate_gear_svg_path(params);
    format!(
        r#"<path d="{}" fill="none" stroke="black" stroke-width="0.5"/>"#,
        path
    )
}