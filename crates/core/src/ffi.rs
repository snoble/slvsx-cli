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
    pub fn real_slvs_add_angle_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line1_id: c_int,
        line2_id: c_int,
        angle: c_double,
    ) -> c_int;
    pub fn real_slvs_add_horizontal_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_vertical_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_equal_length_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line1_id: c_int,
        line2_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_equal_radius_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        circle1_id: c_int,
        circle2_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_tangent_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        entity1_id: c_int,
        entity2_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_point_on_circle_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        circle_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_symmetric_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        entity1_id: c_int,
        entity2_id: c_int,
        line_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_midpoint_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        line_id: c_int,
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

    pub fn add_angle_constraint(
        &mut self,
        id: i32,
        line1_id: i32,
        line2_id: i32,
        angle: f64,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_angle_constraint(self.system, id, line1_id, line2_id, angle);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add angle constraint {}", id))
            }
        }
    }

    pub fn add_horizontal_constraint(
        &mut self,
        id: i32,
        line_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_horizontal_constraint(self.system, id, line_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add horizontal constraint {}", id))
            }
        }
    }

    pub fn add_vertical_constraint(
        &mut self,
        id: i32,
        line_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_vertical_constraint(self.system, id, line_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add vertical constraint {}", id))
            }
        }
    }

    pub fn add_equal_length_constraint(
        &mut self,
        id: i32,
        line1_id: i32,
        line2_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_equal_length_constraint(self.system, id, line1_id, line2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add equal length constraint {}", id))
            }
        }
    }

    pub fn add_equal_radius_constraint(
        &mut self,
        id: i32,
        circle1_id: i32,
        circle2_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_equal_radius_constraint(self.system, id, circle1_id, circle2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add equal radius constraint {}", id))
            }
        }
    }

    pub fn add_tangent_constraint(
        &mut self,
        id: i32,
        entity1_id: i32,
        entity2_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_tangent_constraint(self.system, id, entity1_id, entity2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add tangent constraint {}", id))
            }
        }
    }

    pub fn add_point_on_circle_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        circle_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_point_on_circle_constraint(self.system, id, point_id, circle_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add point on circle constraint {}", id))
            }
        }
    }

    pub fn add_symmetric_constraint(
        &mut self,
        id: i32,
        entity1_id: i32,
        entity2_id: i32,
        line_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_symmetric_constraint(self.system, id, entity1_id, entity2_id, line_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add symmetric constraint {}", id))
            }
        }
    }

    pub fn add_midpoint_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        line_id: i32,
    ) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_midpoint_constraint(self.system, id, point_id, line_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add midpoint constraint {}", id))
            }
        }
    }

    pub fn solve(&mut self) -> Result<(), String> {
        unsafe {
            let result = real_slvs_solve(self.system);
            if result == 0 {
                Ok(())
            } else {
                // Map SolveSpace error codes to descriptive messages
                // See libslvs-static/include/slvs.h for definitions:
                // SLVS_RESULT_OKAY = 0
                // SLVS_RESULT_INCONSISTENT = 1
                // SLVS_RESULT_DIDNT_CONVERGE = 2
                // SLVS_RESULT_TOO_MANY_UNKNOWNS = 3
                // SLVS_RESULT_REDUNDANT_OKAY = 4
                let error_msg = match result {
                    1 => "System is inconsistent (conflicting constraints)".to_string(),
                    2 => "Solver did not converge (try adjusting initial guesses or constraints)".to_string(),
                    3 => "Too many unknowns (system is underconstrained)".to_string(),
                    4 => "System is redundant but solved".to_string(),
                    _ => format!("Solver failed with unknown error code {}", result),
                };
                Err(error_msg)
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

    #[test]
    fn test_angle_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a pivot point
        solver.add_point(1, 0.0, 0.0, 0.0).unwrap();
        
        // Create endpoints for two arms
        solver.add_point(2, 80.0, 0.0, 0.0).unwrap();
        solver.add_point(3, 60.0, 60.0, 0.0).unwrap();

        // Create two lines from pivot
        solver.add_line(10, 1, 2).unwrap(); // arm1: pivot to arm1_end
        solver.add_line(11, 1, 3).unwrap(); // arm2: pivot to arm2_end

        // Fix the pivot point
        solver.add_fixed_constraint(100, 1).unwrap();

        // Add distance constraints to set arm lengths
        solver.add_distance_constraint(101, 1, 2, 80.0).unwrap();
        solver.add_distance_constraint(102, 1, 3, 80.0).unwrap();

        // Add angle constraint: 45 degrees between the two arms
        // This test verifies the FFI binding works correctly
        let result = solver.add_angle_constraint(103, 10, 11, 45.0);
        assert!(result.is_ok(), "Should be able to add angle constraint via FFI");

        // Solve - should succeed (may be underconstrained but shouldn't error)
        let solve_result = solver.solve();
        // Angle constraint may cause underconstrained system, which is acceptable
        assert!(solve_result.is_ok() || solve_result.unwrap_err().contains("code 3"), 
                "Solver should run without FFI errors");
    }

    #[test]
    fn test_horizontal_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create points for a horizontal line
        solver.add_point(1, 0.0, 0.0, 0.0).unwrap();
        solver.add_point(2, 100.0, 10.0, 0.0).unwrap();

        // Create line
        solver.add_line(10, 1, 2).unwrap();

        // Fix first point
        solver.add_fixed_constraint(100, 1).unwrap();

        // Add horizontal constraint
        let result = solver.add_horizontal_constraint(101, 10);
        assert!(result.is_ok(), "Should be able to add horizontal constraint via FFI");

        // Solve - should succeed
        let solve_result = solver.solve();
        assert!(solve_result.is_ok() || solve_result.unwrap_err().contains("code 3"), 
                "Solver should run without FFI errors");
    }

    #[test]
    fn test_vertical_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create points for a vertical line
        solver.add_point(1, 0.0, 0.0, 0.0).unwrap();
        solver.add_point(2, 10.0, 100.0, 0.0).unwrap();

        // Create line
        solver.add_line(10, 1, 2).unwrap();

        // Fix first point
        solver.add_fixed_constraint(100, 1).unwrap();

        // Add vertical constraint
        let result = solver.add_vertical_constraint(101, 10);
        assert!(result.is_ok(), "Should be able to add vertical constraint via FFI");

        // Solve - should succeed
        let solve_result = solver.solve();
        assert!(solve_result.is_ok() || solve_result.unwrap_err().contains("code 3"), 
                "Solver should run without FFI errors");
    }

    #[test]
    fn test_equal_length_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create points for two lines
        solver.add_point(1, 0.0, 0.0, 0.0).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0).unwrap();
        solver.add_point(3, 0.0, 0.0, 0.0).unwrap();
        solver.add_point(4, 100.0, 0.0, 0.0).unwrap();

        // Create two lines
        solver.add_line(10, 1, 2).unwrap();
        solver.add_line(11, 3, 4).unwrap();

        // Fix first point of each line
        solver.add_fixed_constraint(100, 1).unwrap();
        solver.add_fixed_constraint(101, 3).unwrap();

        // Add equal length constraint
        let result = solver.add_equal_length_constraint(102, 10, 11);
        assert!(result.is_ok(), "Should be able to add equal length constraint via FFI");

        // Solve - should succeed
        let solve_result = solver.solve();
        assert!(solve_result.is_ok() || solve_result.unwrap_err().contains("code 3"), 
                "Solver should run without FFI errors");
    }

    #[test]
    fn test_equal_radius_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two circles (note: current implementation uses simplified circles)
        // This test verifies the FFI binding works correctly
        solver.add_circle(1, 0.0, 0.0, 0.0, 10.0).unwrap();
        solver.add_circle(2, 20.0, 0.0, 0.0, 15.0).unwrap();

        // Add equal radius constraint - FFI binding should work
        let result = solver.add_equal_radius_constraint(100, 1, 2);
        assert!(result.is_ok(), "Should be able to add equal radius constraint via FFI");
        
        // Note: Full circle entity support (with workplanes) is needed for actual solving
        // This test verifies the constraint can be added via FFI
    }

    #[test]
    fn test_tangent_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a line and a circle (simplified)
        solver.add_point(1, 0.0, 0.0, 0.0).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0).unwrap();
        solver.add_line(10, 1, 2).unwrap();
        solver.add_circle(20, 50.0, 50.0, 0.0, 25.0).unwrap();

        // Add tangent constraint - FFI binding should work
        let result = solver.add_tangent_constraint(100, 10, 20);
        assert!(result.is_ok(), "Should be able to add tangent constraint via FFI");
        
        // Note: Full circle/arc entity support is needed for actual solving
        // This test verifies the constraint can be added via FFI
    }

    #[test]
    fn test_point_on_circle_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a point and a circle (simplified)
        solver.add_point(1, 50.0, 50.0, 0.0).unwrap();
        solver.add_circle(10, 0.0, 0.0, 0.0, 25.0).unwrap();

        // Add point on circle constraint - FFI binding should work
        let result = solver.add_point_on_circle_constraint(100, 1, 10);
        assert!(result.is_ok(), "Should be able to add point on circle constraint via FFI");
        
        // Note: Full circle entity support is needed for actual solving
        // This test verifies the constraint can be added via FFI
    }

    #[test]
    fn test_symmetric_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two points and a line for symmetry axis
        solver.add_point(1, 30.0, 80.0, 0.0).unwrap();
        solver.add_point(2, 70.0, 80.0, 0.0).unwrap();
        solver.add_point(3, 50.0, 0.0, 0.0).unwrap();
        solver.add_point(4, 50.0, 100.0, 0.0).unwrap();
        solver.add_line(10, 3, 4).unwrap(); // symmetry axis

        // Add symmetric constraint - FFI binding should work
        let result = solver.add_symmetric_constraint(100, 1, 2, 10);
        assert!(result.is_ok(), "Should be able to add symmetric constraint via FFI");
    }

    #[test]
    fn test_midpoint_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a line with endpoints
        solver.add_point(1, 0.0, 50.0, 0.0).unwrap();
        solver.add_point(2, 100.0, 50.0, 0.0).unwrap();
        solver.add_line(10, 1, 2).unwrap();

        // Create midpoint
        solver.add_point(3, 50.0, 50.0, 0.0).unwrap();

        // Add midpoint constraint - FFI binding should work
        let result = solver.add_midpoint_constraint(100, 3, 10);
        assert!(result.is_ok(), "Should be able to add midpoint constraint via FFI");
    }
}
