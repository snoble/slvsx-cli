use std::f64::consts::PI;
use crate::gear_teeth::GearParameters;

/// Generate STL for a gear with proper teeth and depth
pub struct GearStlGenerator {
    pub depth: f64,  // Gear thickness in Z direction
    pub ring_width: f64,  // Width of ring gear wall
}

impl Default for GearStlGenerator {
    fn default() -> Self {
        Self {
            depth: 5.0,  // 5mm thick gears
            ring_width: 10.0,  // 10mm wall for ring gear
        }
    }
}

impl GearStlGenerator {
    /// Generate triangulated STL mesh for a gear
    pub fn generate_gear_stl(&self, params: &GearParameters) -> Vec<String> {
        let mut triangles = Vec::new();
        
        // Generate the tooth profile
        let profile = self.generate_tooth_profile(params);
        
        // Create top and bottom faces
        self.add_face(&mut triangles, &profile, 0.0, true);  // Bottom
        self.add_face(&mut triangles, &profile, self.depth, false);  // Top
        
        // Create side walls
        self.add_walls(&mut triangles, &profile, self.depth);
        
        // For ring gear, add outer wall
        if params.internal {
            self.add_ring_outer_wall(&mut triangles, params, &profile);
        }
        
        triangles
    }
    
    /// Generate 2D tooth profile points
    fn generate_tooth_profile(&self, params: &GearParameters) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        
        let outer_r = params.outer_radius();
        let root_r = params.root_radius();
        
        // Add clearance for 3D printing
        let clearance = 0.3;
        let tip_r = if params.internal {
            outer_r + clearance  // Internal tips don't reach as far in
        } else {
            outer_r - clearance  // External tips pulled back
        };
        let root_clearance_r = if params.internal {
            root_r - clearance * 0.5  // Internal root moved out
        } else {
            root_r + clearance * 0.5  // External root moved in
        };
        
        let tooth_angle = 2.0 * PI / params.teeth as f64;
        let tooth_thickness = tooth_angle * 0.25;
        
        for i in 0..params.teeth {
            let angle_offset = i as f64 * tooth_angle + params.phase.to_radians();
            
            if params.internal {
                // Internal gear - teeth point inward
                // Gap at outer radius
                let gap_start = angle_offset - tooth_thickness * 0.8;
                let gap_end = angle_offset + tooth_thickness * 0.8;
                
                points.push((
                    params.center[0] + root_clearance_r * gap_start.cos(),
                    params.center[1] + root_clearance_r * gap_start.sin()
                ));
                
                // Add arc points for smooth gap
                for j in 1..5 {
                    let angle = gap_start + (gap_end - gap_start) * (j as f64 / 5.0);
                    points.push((
                        params.center[0] + root_clearance_r * angle.cos(),
                        params.center[1] + root_clearance_r * angle.sin()
                    ));
                }
                
                // Tooth tip
                let tooth_tip_angle = angle_offset + tooth_angle * 0.5;
                points.push((
                    params.center[0] + tip_r * tooth_tip_angle.cos(),
                    params.center[1] + tip_r * tooth_tip_angle.sin()
                ));
            } else {
                // External gear - triangular teeth
                let root_angle = angle_offset - tooth_thickness * 0.5;
                points.push((
                    params.center[0] + root_clearance_r * root_angle.cos(),
                    params.center[1] + root_clearance_r * root_angle.sin()
                ));
                
                // Tip
                points.push((
                    params.center[0] + tip_r * angle_offset.cos(),
                    params.center[1] + tip_r * angle_offset.sin()
                ));
                
                // Next root
                let next_root_angle = angle_offset + tooth_thickness * 0.5;
                points.push((
                    params.center[0] + root_clearance_r * next_root_angle.cos(),
                    params.center[1] + root_clearance_r * next_root_angle.sin()
                ));
                
                // Arc to next tooth
                if i < params.teeth - 1 {
                    for j in 1..3 {
                        let angle = next_root_angle + (angle_offset + tooth_angle - tooth_thickness * 0.5 - next_root_angle) * (j as f64 / 3.0);
                        points.push((
                            params.center[0] + root_clearance_r * angle.cos(),
                            params.center[1] + root_clearance_r * angle.sin()
                        ));
                    }
                }
            }
        }
        
        points
    }
    
    /// Add a face (top or bottom) to the STL
    fn add_face(&self, triangles: &mut Vec<String>, profile: &[(f64, f64)], z: f64, flip: bool) {
        // Find center point for fan triangulation
        let center_x: f64 = profile.iter().map(|p| p.0).sum::<f64>() / profile.len() as f64;
        let center_y: f64 = profile.iter().map(|p| p.1).sum::<f64>() / profile.len() as f64;
        
        // Create triangles from center to each edge
        for i in 0..profile.len() {
            let j = (i + 1) % profile.len();
            
            if flip {
                triangles.push(self.format_triangle(
                    (center_x, center_y, z),
                    (profile[j].0, profile[j].1, z),
                    (profile[i].0, profile[i].1, z),
                ));
            } else {
                triangles.push(self.format_triangle(
                    (center_x, center_y, z),
                    (profile[i].0, profile[i].1, z),
                    (profile[j].0, profile[j].1, z),
                ));
            }
        }
    }
    
    /// Add walls connecting top and bottom
    fn add_walls(&self, triangles: &mut Vec<String>, profile: &[(f64, f64)], depth: f64) {
        for i in 0..profile.len() {
            let j = (i + 1) % profile.len();
            
            // Two triangles per wall segment
            triangles.push(self.format_triangle(
                (profile[i].0, profile[i].1, 0.0),
                (profile[j].0, profile[j].1, 0.0),
                (profile[i].0, profile[i].1, depth),
            ));
            
            triangles.push(self.format_triangle(
                (profile[j].0, profile[j].1, 0.0),
                (profile[j].0, profile[j].1, depth),
                (profile[i].0, profile[i].1, depth),
            ));
        }
    }
    
    /// Add outer wall for ring gear
    fn add_ring_outer_wall(&self, triangles: &mut Vec<String>, params: &GearParameters, inner_profile: &[(f64, f64)]) {
        let outer_r = params.root_radius() + self.ring_width;
        let num_segments = 72;  // Smooth circle
        
        // Generate outer circle points
        let mut outer_points = Vec::new();
        for i in 0..num_segments {
            let angle = i as f64 * 2.0 * PI / num_segments as f64;
            outer_points.push((
                params.center[0] + outer_r * angle.cos(),
                params.center[1] + outer_r * angle.sin()
            ));
        }
        
        // Add outer wall faces
        self.add_face(triangles, &outer_points, 0.0, false);
        self.add_face(triangles, &outer_points, self.depth, true);
        self.add_walls(triangles, &outer_points, self.depth);
        
        // Connect inner and outer walls at top and bottom
        // This creates the ring shape with teeth on the inside
    }
    
    /// Format a triangle for STL ASCII format
    fn format_triangle(&self, p1: (f64, f64, f64), p2: (f64, f64, f64), p3: (f64, f64, f64)) -> String {
        // Calculate normal vector
        let v1 = (p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2);
        let v2 = (p3.0 - p1.0, p3.1 - p1.1, p3.2 - p1.2);
        
        let normal = (
            v1.1 * v2.2 - v1.2 * v2.1,
            v1.2 * v2.0 - v1.0 * v2.2,
            v1.0 * v2.1 - v1.1 * v2.0
        );
        
        let mag = (normal.0 * normal.0 + normal.1 * normal.1 + normal.2 * normal.2).sqrt();
        let unit_normal = if mag > 0.0 {
            (normal.0 / mag, normal.1 / mag, normal.2 / mag)
        } else {
            (0.0, 0.0, 1.0)
        };
        
        format!(
            "  facet normal {} {} {}\n    outer loop\n      vertex {} {} {}\n      vertex {} {} {}\n      vertex {} {} {}\n    endloop\n  endfacet\n",
            unit_normal.0, unit_normal.1, unit_normal.2,
            p1.0, p1.1, p1.2,
            p2.0, p2.1, p2.2,
            p3.0, p3.1, p3.2
        )
    }
}

/// Generate STL for the complete planetary gear system
pub fn generate_planetary_stl(entities: &std::collections::HashMap<String, slvsx_core::ir::ResolvedEntity>) -> String {
    let generator = GearStlGenerator::default();
    let mut stl = String::from("solid planetary_gears\n");
    
    for (id, entity) in entities {
        if let slvsx_core::ir::ResolvedEntity::Gear { center, teeth, module, pressure_angle, phase, internal } = entity {
            let params = GearParameters {
                teeth: *teeth,
                module: *module,
                pressure_angle: *pressure_angle,
                center: [center[0], center[1]],
                phase: *phase,
                internal: *internal,
            };
            
            let triangles = generator.generate_gear_stl(&params);
            for triangle in triangles {
                stl.push_str(&triangle);
            }
        }
    }
    
    stl.push_str("endsolid planetary_gears\n");
    stl
}