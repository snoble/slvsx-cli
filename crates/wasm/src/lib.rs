use wasm_bindgen::prelude::*;
use slvsx_core::{Input, solve};

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    // Set panic hook for better error messages in browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"SLVSX WASM module loaded".into());
}

/// Solve constraints from JSON input
#[wasm_bindgen]
pub fn solve_constraints(json_input: &str) -> Result<String, JsValue> {
    // Parse JSON input
    let input: Input = serde_json::from_str(json_input)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse input: {}", e)))?;
    
    // Solve constraints
    let result = solve(input)
        .map_err(|e| JsValue::from_str(&format!("Solver error: {}", e)))?;
    
    // Convert result to JSON
    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

/// Validate assembly constraint for planetary gears
#[wasm_bindgen]
pub fn validate_assembly(sun_teeth: u32, ring_teeth: u32, num_planets: u32) -> bool {
    slvsx_core::phase_calculator::validate_assembly_constraint(sun_teeth, ring_teeth, num_planets)
}

/// Calculate gear mesh distance
#[wasm_bindgen]
pub fn calculate_mesh_distance(teeth1: u32, teeth2: u32, module: f64) -> f64 {
    (teeth1 + teeth2) as f64 * module / 2.0
}