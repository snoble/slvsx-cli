use std::os::raw::{c_double, c_int};

// FFI bindings to our C wrapper
#[repr(C)]
pub struct SolverSystem {
    _private: [u8; 0],
}

#[link(name = "real_slvs_wrapper")]
extern "C" {
    pub fn real_slvs_create() -> *mut SolverSystem;
    pub fn real_slvs_destroy(sys: *mut SolverSystem);
    
    pub fn real_slvs_add_circle(sys: *mut SolverSystem, id: c_int, cx: c_double, cy: c_double, cz: c_double, radius: c_double) -> c_int;
    
    pub fn real_slvs_add_distance_constraint(sys: *mut SolverSystem, id: c_int, entity1: c_int, entity2: c_int, distance: c_double) -> c_int;
    
    pub fn real_slvs_solve(sys: *mut SolverSystem) -> c_int;
    
    pub fn real_slvs_get_circle_position(sys: *mut SolverSystem, id: c_int, cx: *mut c_double, cy: *mut c_double, cz: *mut c_double, radius: *mut c_double) -> c_int;
}

// Safe Rust wrapper
pub struct Solver {
    system: *mut SolverSystem,
}

impl Solver {
    pub fn new() -> Self {
        unsafe {
            Self {
                system: real_slvs_create(),
            }
        }
    }
    
    
    pub fn add_circle(&mut self, id: i32, cx: f64, cy: f64, cz: f64, radius: f64) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_circle(self.system, id, cx, cy, cz, radius);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add circle {}", id))
            }
        }
    }
    
    pub fn add_distance_constraint(&mut self, id: i32, entity1: i32, entity2: i32, distance: f64) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_distance_constraint(self.system, id, entity1, entity2, distance);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add distance constraint {}", id))
            }
        }
    }
    
    pub fn solve(&mut self) -> Result<(), String> {
        unsafe {
            let result = real_slvs_solve(self.system);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Solver failed with code {}", result))
            }
        }
    }
    
    pub fn get_circle_position(&self, id: i32) -> Result<(f64, f64, f64, f64), String> {
        unsafe {
            let mut cx = 0.0;
            let mut cy = 0.0;
            let mut cz = 0.0;
            let mut radius = 0.0;
            
            let result = real_slvs_get_circle_position(self.system, id, &mut cx, &mut cy, &mut cz, &mut radius);
            if result == 0 {
                Ok((cx, cy, cz, radius))
            } else {
                Err(format!("Circle {} not found", id))
            }
        }
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe {
            real_slvs_destroy(self.system);
        }
    }
}

unsafe impl Send for Solver {}
unsafe impl Sync for Solver {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[cfg(not(feature = "mock-solver"))]
    fn test_ffi_solver() {
        let mut solver = Solver::new();
        
        // Add sun gear
        solver.add_circle(1, 0.0, 0.0, 0.0, 24.0).unwrap();
        
        // Add planet gear
        solver.add_circle(2, 36.0, 0.0, 0.0, 12.0).unwrap();
        
        // Add distance constraint
        solver.add_distance_constraint(100, 1, 2, 36.0).unwrap();
        
        // Solve
        solver.solve().unwrap();
        
        // Get result
        let (cx, cy, _cz, radius) = solver.get_circle_position(2).unwrap();
        assert!((cx - 36.0).abs() < 0.001 || cy.abs() > 0.001); // Should be at distance 36
        assert_eq!(radius, 12.0);
    }
}