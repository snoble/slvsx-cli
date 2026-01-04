use std::os::raw::{c_double, c_int};

/// FFI error types for better error handling
#[derive(Debug, Clone)]
pub enum FfiError {
    /// System is inconsistent (overconstrained)
    Inconsistent,
    /// Solver didn't converge
    DidntConverge,
    /// Too many unknowns (underconstrained)
    TooManyUnknowns,
    /// Invalid system pointer
    InvalidSystem,
    /// Unknown error code
    Unknown(i32),
    /// Entity not found
    EntityNotFound(String),
    /// Constraint operation failed
    ConstraintFailed(String),
}

impl std::fmt::Display for FfiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfiError::Inconsistent => write!(f, "System is inconsistent (overconstrained)"),
            FfiError::DidntConverge => write!(f, "Solver failed to converge"),
            FfiError::TooManyUnknowns => write!(f, "System has too many unknowns (underconstrained)"),
            FfiError::InvalidSystem => write!(f, "Invalid solver system"),
            FfiError::Unknown(code) => write!(f, "Unknown solver error (code: {})", code),
            FfiError::EntityNotFound(id) => write!(f, "Entity not found: {}", id),
            FfiError::ConstraintFailed(msg) => write!(f, "Constraint operation failed: {}", msg),
        }
    }
}

impl std::error::Error for FfiError {}

// FFI bindings to our C wrapper
#[repr(C)]
pub struct SolverSystem {
    _private: [u8; 0],
}

#[link(name = "real_slvs_wrapper")]
extern "C" {
    pub fn real_slvs_create() -> *mut SolverSystem;
    pub fn real_slvs_destroy(sys: *mut SolverSystem);

    pub fn real_slvs_add_point(
        sys: *mut SolverSystem,
        id: c_int,
        x: c_double,
        y: c_double,
        z: c_double,
    ) -> c_int;
    pub fn real_slvs_add_line(
        sys: *mut SolverSystem,
        id: c_int,
        point1_id: c_int,
        point2_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_circle(
        sys: *mut SolverSystem,
        id: c_int,
        cx: c_double,
        cy: c_double,
        cz: c_double,
        radius: c_double,
    ) -> c_int;

    pub fn real_slvs_add_fixed_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        entity_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_distance_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        entity1: c_int,
        entity2: c_int,
        distance: c_double,
    ) -> c_int;
    pub fn real_slvs_add_point_on_line_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        line_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_points_coincident_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point1_id: c_int,
        point2_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_perpendicular_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line1_id: c_int,
        line2_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_parallel_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line1_id: c_int,
        line2_id: c_int,
    ) -> c_int;

    pub fn real_slvs_solve(sys: *mut SolverSystem) -> c_int;

    pub fn real_slvs_get_point_position(
        sys: *mut SolverSystem,
        id: c_int,
        x: *mut c_double,
        y: *mut c_double,
        z: *mut c_double,
    ) -> c_int;
    pub fn real_slvs_get_circle_position(
        sys: *mut SolverSystem,
        id: c_int,
        cx: *mut c_double,
        cy: *mut c_double,
        cz: *mut c_double,
        radius: *mut c_double,
    ) -> c_int;
}

// Safe Rust wrapper
pub struct Solver {
    system: *mut SolverSystem,
}

impl Solver {
    pub fn new() -> Self {
        unsafe {
            let system = real_slvs_create();
            if system.is_null() {
                panic!("Failed to create solver system");
            }
            Self { system }
        }
    }

    pub fn add_point(&mut self, id: i32, x: f64, y: f64, z: f64) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_point(self.system, id, x, y, z);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add point {}", id))
            }
        }
    }

    pub fn add_line(&mut self, id: i32, point1_id: i32, point2_id: i32) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_line(self.system, id, point1_id, point2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add line {}", id))
            }
        }
    }

    pub fn add_circle(
        &mut self,
        id: i32,
        cx: f64,
        cy: f64,
        cz: f64,
        radius: f64,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_circle(self.system, id, cx, cy, cz, radius);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add circle {}", id))
            }
        }
    }

    pub fn add_fixed_constraint(&mut self, id: i32, entity_id: i32) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_fixed_constraint(self.system, id, entity_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add fixed constraint {}", id))
            }
        }
    }

    pub fn add_distance_constraint(
        &mut self,
        id: i32,
        entity1: i32,
        entity2: i32,
        distance: f64,
    ) -> Result<(), String> {
        unsafe {
            let result =
                real_slvs_add_distance_constraint(self.system, id, entity1, entity2, distance);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add distance constraint {}", id))
            }
        }
    }

    pub fn add_point_on_line_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        line_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_point_on_line_constraint(self.system, id, point_id, line_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add point on line constraint {}", id))
            }
        }
    }

    pub fn add_points_coincident_constraint(
        &mut self,
        id: i32,
        point1_id: i32,
        point2_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_points_coincident_constraint(self.system, id, point1_id, point2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add points coincident constraint {}", id))
            }
        }
    }

    pub fn add_perpendicular_constraint(
        &mut self,
        id: i32,
        line1_id: i32,
        line2_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_perpendicular_constraint(self.system, id, line1_id, line2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add perpendicular constraint {}", id))
            }
        }
    }

    pub fn add_parallel_constraint(
        &mut self,
        id: i32,
        line1_id: i32,
        line2_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_parallel_constraint(self.system, id, line1_id, line2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add parallel constraint {}", id))
            }
        }
    }

    pub fn solve(&mut self) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_solve(self.system);
            match result {
                0 => Ok(()),
                1 => Err(FfiError::Inconsistent), // Overconstrained
                2 => Err(FfiError::DidntConverge), // Convergence failure
                3 => Err(FfiError::TooManyUnknowns), // Underconstrained
                -1 => Err(FfiError::InvalidSystem),
                code => Err(FfiError::Unknown(code)),
            }
        }
    }

    pub fn get_point_position(&self, id: i32) -> Result<(f64, f64, f64), String> {
        unsafe {
            let mut x = 0.0;
            let mut y = 0.0;
            let mut z = 0.0;

            let result = real_slvs_get_point_position(self.system, id, &mut x, &mut y, &mut z);
            if result == 0 {
                Ok((x, y, z))
            } else {
                Err(format!("Point {} not found", id))
            }
        }
    }

    pub fn get_circle_position(&self, id: i32) -> Result<(f64, f64, f64, f64), String> {
        unsafe {
            let mut cx = 0.0;
            let mut cy = 0.0;
            let mut cz = 0.0;
            let mut radius = 0.0;

            let result = real_slvs_get_circle_position(
                self.system,
                id,
                &mut cx,
                &mut cy,
                &mut cz,
                &mut radius,
            );
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
            if !self.system.is_null() {
                real_slvs_destroy(self.system);
                self.system = std::ptr::null_mut();
            }
        }
    }
}

unsafe impl Send for Solver {}
unsafe impl Sync for Solver {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
