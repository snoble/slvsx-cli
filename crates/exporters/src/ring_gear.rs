use std::f64::consts::PI;
use crate::gear_teeth::GearParameters;

/// Generate SVG for a ring gear with wall thickness
pub fn generate_ring_gear_svg(params: &GearParameters, wall_thickness: f64) -> String {
    // Ring gear has teeth on the inside and a solid wall on the outside
    let pitch_r = params.pitch_radius();
    let outer_wall_r = pitch_r + wall_thickness; // Outside of the wall
    
    let mut svg = String::new();
    
    // Generate the teeth path (inner profile)
    let teeth_path = crate::gear_teeth::generate_gear_svg_path(params);
    
    // Generate the outer wall circle
    let cx = params.center[0];
    let cy = params.center[1];
    
    // Create a compound path: outer circle (clockwise) and inner teeth (counter-clockwise)
    // This creates a filled ring with teeth cut out from the inside
    svg.push_str(&format!(
        r#"<g id="ring-gear">
  <!-- Outer wall -->
  <circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="none" stroke="black" stroke-width="0.5" stroke-dasharray="2,1" opacity="0.3"/>
  <!-- Ring with teeth -->
  <path d="M {:.2} {:.2} m -{:.2} 0 a {:.2} {:.2} 0 1 0 {:.2} 0 a {:.2} {:.2} 0 1 0 -{:.2} 0 Z M {} Z" fill-rule="evenodd" fill="lightgray" fill-opacity="0.3" stroke="black" stroke-width="0.5"/>
  <!-- Teeth only (for visibility) -->
  <path d="{}" fill="none" stroke="black" stroke-width="0.5"/>
</g>"#,
        cx, cy, outer_wall_r,  // Outer wall circle (visual guide)
        cx, cy, outer_wall_r,  // Start of outer circle path
        outer_wall_r, outer_wall_r, outer_wall_r * 2.0,  // First arc
        outer_wall_r, outer_wall_r, outer_wall_r * 2.0,  // Second arc
        teeth_path.trim_end_matches(" Z"),  // Inner teeth path (remove the Z, we'll add it)
        teeth_path  // Teeth outline
    ));
    
    svg
}

/// Generate a simpler ring representation with just inner and outer circles
pub fn generate_simple_ring_svg(params: &GearParameters, wall_thickness: f64) -> String {
    let pitch_r = params.pitch_radius();
    let inner_r = pitch_r - params.dedendum(); // Bottom of teeth
    let outer_r = pitch_r + wall_thickness;
    
    let cx = params.center[0];
    let cy = params.center[1];
    
    // Draw the ring as two concentric circles with teeth indicated
    let teeth_path = crate::gear_teeth::generate_gear_svg_path(params);
    
    format!(
        r#"<g id="ring-gear">
  <!-- Outer wall -->
  <circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="none" stroke="black" stroke-width="1.0"/>
  <!-- Inner teeth -->
  <path d="{}" fill="none" stroke="black" stroke-width="0.5"/>
  <!-- Cross-hatching to show it's a solid ring -->
  <path d="M {:.2} {:.2} L {:.2} {:.2}" stroke="black" stroke-width="0.3" opacity="0.5"/>
  <path d="M {:.2} {:.2} L {:.2} {:.2}" stroke="black" stroke-width="0.3" opacity="0.5"/>
</g>"#,
        cx, cy, outer_r,  // Outer circle
        teeth_path,       // Inner teeth
        cx - outer_r, cy, cx - inner_r, cy,  // Left cross-hatch
        cx + inner_r, cy, cx + outer_r, cy   // Right cross-hatch
    )
}