//! WASM bindings for SLVSX constraint solver
//! 
//! This module provides JavaScript/TypeScript bindings for using SLVSX in web browsers
//! and Node.js environments.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
use crate::{InputDocument, SolveResult, solver::{Solver, SolverConfig}};

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmSolver {
    solver: Solver,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmSolver {
    /// Create a new WASM solver instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set up console error panic hook for better debugging
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        
        Self {
            solver: Solver::new(SolverConfig::default()),
        }
    }
    
    /// Solve a constraint system from JSON string
    /// 
    /// # Arguments
    /// * `json_str` - JSON string containing the constraint specification
    /// 
    /// # Returns
    /// JSON string containing the solve result or error
    #[wasm_bindgen]
    pub fn solve(&self, json_str: &str) -> Result<String, JsValue> {
        // Parse input JSON
        let doc: InputDocument = serde_json::from_str(json_str)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;
        
        // Solve constraints
        let result = self.solver.solve(&doc)
            .map_err(|e| JsValue::from_str(&format!("Solve error: {}", e)))?;
        
        // Serialize result to JSON
        serde_json::to_string(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
    
    /// Validate a constraint document without solving
    #[wasm_bindgen]
    pub fn validate(&self, json_str: &str) -> Result<bool, JsValue> {
        use crate::validator::Validator;
        
        let doc: InputDocument = serde_json::from_str(json_str)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON: {}", e)))?;
        
        let validator = Validator::new();
        validator.validate(&doc)
            .map(|_| true)
            .map_err(|e| JsValue::from_str(&format!("Validation error: {}", e)))
    }
    
    /// Get the JSON schema for constraint documents
    #[wasm_bindgen]
    pub fn get_schema() -> String {
        use schemars::schema_for;
        
        let schema = schema_for!(InputDocument);
        serde_json::to_string_pretty(&schema).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Solve constraints directly without creating a solver instance
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn solve_constraints(json_str: &str) -> Result<String, JsValue> {
    let solver = WasmSolver::new();
    solver.solve(json_str)
}

/// Validate a constraint document
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate_document(json_str: &str) -> Result<bool, JsValue> {
    let solver = WasmSolver::new();
    solver.validate(json_str)
}

/// Get the constraint system JSON schema
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_constraint_schema() -> String {
    WasmSolver::get_schema()
}

/// Initialize the WASM module (called automatically)
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn init() {
    // Set up console logging for debugging
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    // Initialize tracing if available
    #[cfg(feature = "wasm-logger")]
    wasm_logger::init(wasm_logger::Config::default());
}

// Re-export types for TypeScript generation
#[cfg(feature = "wasm")]
#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r#"
/**
 * SLVSX Constraint Solver WASM Module
 * 
 * Example usage:
 * ```typescript
 * import init, { WasmSolver, solve_constraints } from '@slvsx/core';
 * 
 * // Initialize WASM module
 * await init();
 * 
 * // Option 1: Use solver instance
 * const solver = new WasmSolver();
 * const result = solver.solve(constraintJson);
 * 
 * // Option 2: Use direct function
 * const result = solve_constraints(constraintJson);
 * ```
 */
export interface ConstraintDocument {
  schema: "slvs-json/1";
  units?: string;
  parameters?: Record<string, number>;
  entities: Entity[];
  constraints: Constraint[];
}

export interface SolveResult {
  status: string;
  diagnostics?: {
    iters: number;
    residual: number;
    dof: number;
    time_ms: number;
  };
  entities?: Record<string, ResolvedEntity>;
  warnings: string[];
}
"#;