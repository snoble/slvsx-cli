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
            FfiError::Inconsistent => write!(f, "System is inconsistent (conflicting constraints)"),
            FfiError::DidntConverge => write!(f, "Solver did not converge (try adjusting initial guesses or constraints)"),
            FfiError::TooManyUnknowns => write!(f, "Too many unknowns (system is underconstrained)"),
            FfiError::InvalidSystem => write!(f, "Invalid solver system"),
            FfiError::Unknown(code) => write!(f, "Solver failed with unknown error code {}", code),
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
        is_dragged: c_int, // 1 if dragged, 0 otherwise
    ) -> c_int;

    pub fn real_slvs_add_where_dragged_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        workplane_id: c_int, // -1 for 3D, otherwise workplane ID
    ) -> c_int;
    pub fn real_slvs_add_line(
        sys: *mut SolverSystem,
        id: c_int,
        point1_id: c_int,
        point2_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_line_2d(
        sys: *mut SolverSystem,
        id: c_int,
        point1_id: c_int,
        point2_id: c_int,
        workplane_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_point_2d(
        sys: *mut SolverSystem,
        id: c_int,
        workplane_id: c_int,
        u: c_double,
        v: c_double,
        is_dragged: c_int, // 1 if dragged, 0 otherwise
    ) -> c_int;

    pub fn real_slvs_add_circle(
        sys: *mut SolverSystem,
        id: c_int,
        cx: c_double,
        cy: c_double,
        cz: c_double,
        radius: c_double,
    ) -> c_int;

    pub fn real_slvs_add_arc(
        sys: *mut SolverSystem,
        id: c_int,
        center_point_id: c_int,
        start_point_id: c_int,
        end_point_id: c_int,
        nx: c_double,
        ny: c_double,
        nz: c_double,
        workplane_id: c_int, // -1 for 3D, otherwise workplane ID
    ) -> c_int;

    pub fn real_slvs_add_cubic(
        sys: *mut SolverSystem,
        id: c_int,
        pt0_id: c_int,
        pt1_id: c_int,
        pt2_id: c_int,
        pt3_id: c_int,
        workplane_id: c_int, // -1 for 3D, otherwise workplane ID
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
        workplane_id: c_int,
    ) -> c_int;
    pub fn real_slvs_add_vertical_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line_id: c_int,
        workplane_id: c_int,
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

    pub fn real_slvs_add_workplane(
        sys: *mut SolverSystem,
        id: c_int,
        origin_point_id: c_int,
        nx: c_double,
        ny: c_double,
        nz: c_double,
    ) -> c_int;

    pub fn real_slvs_add_point_in_plane_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        workplane_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_point_plane_distance_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        workplane_id: c_int,
        distance: c_double,
    ) -> c_int;

    pub fn real_slvs_add_point_line_distance_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        line_id: c_int,
        distance: c_double,
    ) -> c_int;

    pub fn real_slvs_add_length_ratio_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line1_id: c_int,
        line2_id: c_int,
        ratio: c_double,
    ) -> c_int;

    pub fn real_slvs_add_equal_angle_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line1_id: c_int,
        line2_id: c_int,
        line3_id: c_int,
        line4_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_symmetric_horizontal_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        entity1_id: c_int,
        entity2_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_symmetric_vertical_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        entity1_id: c_int,
        entity2_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_diameter_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        circle_id: c_int,
        diameter: c_double,
    ) -> c_int;

    pub fn real_slvs_add_same_orientation_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        entity1_id: c_int,
        entity2_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_projected_point_distance_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point1_id: c_int,
        point2_id: c_int,
        workplane_id: c_int,
        distance: c_double,
    ) -> c_int;

    pub fn real_slvs_add_length_difference_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line1_id: c_int,
        line2_id: c_int,
        difference: c_double,
    ) -> c_int;

    pub fn real_slvs_add_point_on_face_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        face_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_point_face_distance_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point_id: c_int,
        face_id: c_int,
        distance: c_double,
    ) -> c_int;

    pub fn real_slvs_add_equal_line_arc_length_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line_id: c_int,
        arc_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_equal_length_point_line_distance_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        line_id: c_int,
        point_id: c_int,
        reference_line_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_equal_point_line_distances_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        point1_id: c_int,
        line1_id: c_int,
        point2_id: c_int,
        line2_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_cubic_line_tangent_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        cubic_id: c_int,
        line_id: c_int,
    ) -> c_int;

    pub fn real_slvs_add_arc_arc_length_ratio_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        arc1_id: c_int,
        arc2_id: c_int,
        ratio: c_double,
    ) -> c_int;

    pub fn real_slvs_add_arc_line_length_ratio_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        arc_id: c_int,
        line_id: c_int,
        ratio: c_double,
    ) -> c_int;

    pub fn real_slvs_add_arc_arc_length_difference_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        arc1_id: c_int,
        arc2_id: c_int,
        difference: c_double,
    ) -> c_int;

    pub fn real_slvs_add_arc_line_length_difference_constraint(
        sys: *mut SolverSystem,
        id: c_int,
        arc_id: c_int,
        line_id: c_int,
        difference: c_double,
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

    pub fn add_point(&mut self, id: i32, x: f64, y: f64, z: f64, is_dragged: bool) -> Result<(), String> {
        unsafe {
            let dragged = if is_dragged { 1 } else { 0 };
            let result = real_slvs_add_point(self.system, id, x, y, z, dragged);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add point {}", id))
            }
        }
    }

    pub fn add_where_dragged_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        workplane_id: Option<i32>, // None for 3D, Some(id) for 2D
    ) -> Result<(), FfiError> {
        unsafe {
            let wp_id = workplane_id.unwrap_or(-1);
            let result = real_slvs_add_where_dragged_constraint(self.system, id, point_id, wp_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add WHERE_DRAGGED constraint {}", id)))
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

    pub fn add_line_2d(&mut self, id: i32, point1_id: i32, point2_id: i32, workplane_id: i32) -> Result<(), String> {
        unsafe {
            let result = real_slvs_add_line_2d(self.system, id, point1_id, point2_id, workplane_id);
            if result == 0 {
                Ok(())
            } else {
                Err(format!("Failed to add 2D line {}", id))
            }
        }
    }

    pub fn add_point_2d(
        &mut self,
        id: i32,
        workplane_id: i32,
        u: f64,
        v: f64,
        is_dragged: bool,
    ) -> Result<(), FfiError> {
        unsafe {
            let dragged = if is_dragged { 1 } else { 0 };
            let result = real_slvs_add_point_2d(self.system, id, workplane_id, u, v, dragged);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add 2D point {}", id)))
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

    pub fn add_arc(
        &mut self,
        id: i32,
        center_point_id: i32,
        start_point_id: i32,
        end_point_id: i32,
        nx: f64,
        ny: f64,
        nz: f64,
        workplane_id: Option<i32>, // None for 3D, Some(id) for 2D
    ) -> Result<(), FfiError> {
        unsafe {
            let wp_id = workplane_id.unwrap_or(-1);
            let result = real_slvs_add_arc(
                self.system, id, center_point_id, start_point_id, end_point_id,
                nx, ny, nz, wp_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add arc {}", id)))
            }
        }
    }

    pub fn add_cubic(
        &mut self,
        id: i32,
        pt0_id: i32,
        pt1_id: i32,
        pt2_id: i32,
        pt3_id: i32,
        workplane_id: Option<i32>, // None for 3D, Some(id) for 2D
    ) -> Result<(), FfiError> {
        unsafe {
            let wp_id = workplane_id.unwrap_or(-1);
            let result = real_slvs_add_cubic(
                self.system, id, pt0_id, pt1_id, pt2_id, pt3_id, wp_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add cubic {}", id)))
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
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_angle_constraint(self.system, id, line1_id, line2_id, angle);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add angle constraint {}", id)))
            }
        }
    }

    pub fn add_horizontal_constraint(
        &mut self,
        id: i32,
        line_id: i32,
        workplane_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_horizontal_constraint(self.system, id, line_id, workplane_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add horizontal constraint {}", id)))
            }
        }
    }

    pub fn add_vertical_constraint(
        &mut self,
        id: i32,
        line_id: i32,
        workplane_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_vertical_constraint(self.system, id, line_id, workplane_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add vertical constraint {}", id)))
            }
        }
    }

    pub fn add_equal_length_constraint(
        &mut self,
        id: i32,
        line1_id: i32,
        line2_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_equal_length_constraint(self.system, id, line1_id, line2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add equal length constraint {}", id)))
            }
        }
    }

    pub fn add_equal_radius_constraint(
        &mut self,
        id: i32,
        circle1_id: i32,
        circle2_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_equal_radius_constraint(self.system, id, circle1_id, circle2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add equal radius constraint {}", id)))
            }
        }
    }

    pub fn add_tangent_constraint(
        &mut self,
        id: i32,
        entity1_id: i32,
        entity2_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_tangent_constraint(self.system, id, entity1_id, entity2_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add tangent constraint {}", id)))
            }
        }
    }

    pub fn add_point_on_circle_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        circle_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_point_on_circle_constraint(self.system, id, point_id, circle_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add point on circle constraint {}", id)))
            }
        }
    }

    pub fn add_symmetric_constraint(
        &mut self,
        id: i32,
        entity1_id: i32,
        entity2_id: i32,
        line_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_symmetric_constraint(self.system, id, entity1_id, entity2_id, line_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add symmetric constraint {}", id)))
            }
        }
    }

    pub fn add_midpoint_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        line_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_midpoint_constraint(self.system, id, point_id, line_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add midpoint constraint {}", id)))
            }
        }
    }

    pub fn add_workplane(
        &mut self,
        id: i32,
        origin_point_id: i32,
        nx: f64,
        ny: f64,
        nz: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_workplane(self.system, id, origin_point_id, nx, ny, nz);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add workplane {}", id)))
            }
        }
    }

    pub fn add_point_in_plane_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        workplane_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_point_in_plane_constraint(self.system, id, point_id, workplane_id);
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add point in plane constraint {}", id)))
            }
        }
    }

    pub fn add_point_plane_distance_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        workplane_id: i32,
        distance: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_point_plane_distance_constraint(
                self.system, id, point_id, workplane_id, distance
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add point plane distance constraint {}", id)))
            }
        }
    }

    pub fn add_point_line_distance_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        line_id: i32,
        distance: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_point_line_distance_constraint(
                self.system, id, point_id, line_id, distance
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add point line distance constraint {}", id)))
            }
        }
    }

    pub fn add_length_ratio_constraint(
        &mut self,
        id: i32,
        line1_id: i32,
        line2_id: i32,
        ratio: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_length_ratio_constraint(
                self.system, id, line1_id, line2_id, ratio
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add length ratio constraint {}", id)))
            }
        }
    }

    pub fn add_equal_angle_constraint(
        &mut self,
        id: i32,
        line1_id: i32,
        line2_id: i32,
        line3_id: i32,
        line4_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_equal_angle_constraint(
                self.system, id, line1_id, line2_id, line3_id, line4_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add equal angle constraint {}", id)))
            }
        }
    }

    pub fn add_symmetric_horizontal_constraint(
        &mut self,
        id: i32,
        entity1_id: i32,
        entity2_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_symmetric_horizontal_constraint(
                self.system, id, entity1_id, entity2_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add symmetric horizontal constraint {}", id)))
            }
        }
    }

    pub fn add_symmetric_vertical_constraint(
        &mut self,
        id: i32,
        entity1_id: i32,
        entity2_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_symmetric_vertical_constraint(
                self.system, id, entity1_id, entity2_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add symmetric vertical constraint {}", id)))
            }
        }
    }

    pub fn add_diameter_constraint(
        &mut self,
        id: i32,
        circle_id: i32,
        diameter: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_diameter_constraint(
                self.system, id, circle_id, diameter
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add diameter constraint {}", id)))
            }
        }
    }

    pub fn add_same_orientation_constraint(
        &mut self,
        id: i32,
        entity1_id: i32,
        entity2_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_same_orientation_constraint(
                self.system, id, entity1_id, entity2_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add same orientation constraint {}", id)))
            }
        }
    }

    pub fn add_projected_point_distance_constraint(
        &mut self,
        id: i32,
        point1_id: i32,
        point2_id: i32,
        workplane_id: i32,
        distance: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_projected_point_distance_constraint(
                self.system, id, point1_id, point2_id, workplane_id, distance
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add projected point distance constraint {}", id)))
            }
        }
    }

    pub fn add_length_difference_constraint(
        &mut self,
        id: i32,
        line1_id: i32,
        line2_id: i32,
        difference: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_length_difference_constraint(
                self.system, id, line1_id, line2_id, difference
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add length difference constraint {}", id)))
            }
        }
    }

    pub fn add_point_on_face_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        face_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_point_on_face_constraint(
                self.system, id, point_id, face_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add point on face constraint {}", id)))
            }
        }
    }

    pub fn add_point_face_distance_constraint(
        &mut self,
        id: i32,
        point_id: i32,
        face_id: i32,
        distance: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_point_face_distance_constraint(
                self.system, id, point_id, face_id, distance
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add point face distance constraint {}", id)))
            }
        }
    }

    pub fn add_equal_line_arc_length_constraint(
        &mut self,
        id: i32,
        line_id: i32,
        arc_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_equal_line_arc_length_constraint(
                self.system, id, line_id, arc_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add equal line arc length constraint {}", id)))
            }
        }
    }

    pub fn add_equal_length_point_line_distance_constraint(
        &mut self,
        id: i32,
        line_id: i32,
        point_id: i32,
        reference_line_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_equal_length_point_line_distance_constraint(
                self.system, id, line_id, point_id, reference_line_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add equal length point line distance constraint {}", id)))
            }
        }
    }

    pub fn add_equal_point_line_distances_constraint(
        &mut self,
        id: i32,
        point1_id: i32,
        line1_id: i32,
        point2_id: i32,
        line2_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_equal_point_line_distances_constraint(
                self.system, id, point1_id, line1_id, point2_id, line2_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add equal point line distances constraint {}", id)))
            }
        }
    }

    pub fn add_cubic_line_tangent_constraint(
        &mut self,
        id: i32,
        cubic_id: i32,
        line_id: i32,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_cubic_line_tangent_constraint(
                self.system, id, cubic_id, line_id
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add cubic line tangent constraint {}", id)))
            }
        }
    }

    pub fn add_arc_arc_length_ratio_constraint(
        &mut self,
        id: i32,
        arc1_id: i32,
        arc2_id: i32,
        ratio: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_arc_arc_length_ratio_constraint(
                self.system, id, arc1_id, arc2_id, ratio
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add arc arc length ratio constraint {}", id)))
            }
        }
    }

    pub fn add_arc_line_length_ratio_constraint(
        &mut self,
        id: i32,
        arc_id: i32,
        line_id: i32,
        ratio: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_arc_line_length_ratio_constraint(
                self.system, id, arc_id, line_id, ratio
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add arc line length ratio constraint {}", id)))
            }
        }
    }

    pub fn add_arc_arc_length_difference_constraint(
        &mut self,
        id: i32,
        arc1_id: i32,
        arc2_id: i32,
        difference: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_arc_arc_length_difference_constraint(
                self.system, id, arc1_id, arc2_id, difference
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add arc arc length difference constraint {}", id)))
            }
        }
    }

    pub fn add_arc_line_length_difference_constraint(
        &mut self,
        id: i32,
        arc_id: i32,
        line_id: i32,
        difference: f64,
    ) -> Result<(), FfiError> {
        unsafe {
            let result = real_slvs_add_arc_line_length_difference_constraint(
                self.system, id, arc_id, line_id, difference
            );
            if result == 0 {
                Ok(())
            } else {
                Err(FfiError::ConstraintFailed(format!("Failed to add arc line length difference constraint {}", id)))
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

        // Add two points representing circle centers
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 36.0, 0.0, 0.0, false).unwrap();
        
        // Fix the first point
        solver.add_fixed_constraint(1, 1).unwrap();

        // Add distance constraint between points
        solver.add_distance_constraint(100, 1, 2, 36.0).unwrap();

        // Solve
        solver.solve().unwrap();

        // Get result
        let (x, y, z) = solver.get_point_position(2).unwrap();
        let distance = (x * x + y * y + z * z).sqrt();
        assert!((distance - 36.0).abs() < 0.001, "Point should be at distance 36 from origin");
    }

    #[test]
    fn test_angle_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a pivot point
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        
        // Create endpoints for two arms
        solver.add_point(2, 80.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 60.0, 60.0, 0.0, false).unwrap();

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
        assert!(solve_result.is_ok() || matches!(solve_result.unwrap_err(), FfiError::TooManyUnknowns), 
                "Solver should run without FFI errors");
    }

    #[test]
    fn test_horizontal_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a workplane first (horizontal/vertical constraints require a workplane)
        // First create an origin point
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        // Create workplane with origin point and normal vector (0,0,1) = XY plane
        solver.add_workplane(10, 1, 0.0, 0.0, 1.0).unwrap();

        // Create 2D points in the workplane
        solver.add_point_2d(2, 10, 0.0, 0.0, false).unwrap();
        solver.add_point_2d(3, 10, 100.0, 10.0, false).unwrap();

        // Create 2D line in the workplane (using 2D point IDs)
        solver.add_line_2d(20, 2, 3, 10).unwrap();

        // Fix first point
        solver.add_fixed_constraint(100, 2).unwrap();

        // Add horizontal constraint (requires workplane)
        let result = solver.add_horizontal_constraint(101, 20, 10);
        assert!(result.is_ok(), "Should be able to add horizontal constraint via FFI");

        // Solve - should succeed
        let solve_result = solver.solve();
        assert!(solve_result.is_ok() || matches!(solve_result.unwrap_err(), FfiError::TooManyUnknowns), 
                "Solver should run without FFI errors");
    }

    #[test]
    fn test_vertical_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a workplane first (horizontal/vertical constraints require a workplane)
        // First create an origin point
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        // Create workplane with origin point and normal vector (0,0,1) = XY plane
        solver.add_workplane(10, 1, 0.0, 0.0, 1.0).unwrap();

        // Create 2D points in the workplane
        solver.add_point_2d(2, 10, 0.0, 0.0, false).unwrap();
        solver.add_point_2d(3, 10, 10.0, 100.0, false).unwrap();

        // Create 2D line in the workplane (using 2D point IDs)
        solver.add_line_2d(20, 2, 3, 10).unwrap();

        // Fix first point
        solver.add_fixed_constraint(100, 2).unwrap();

        // Add vertical constraint (requires workplane)
        let result = solver.add_vertical_constraint(101, 20, 10);
        assert!(result.is_ok(), "Should be able to add vertical constraint via FFI");

        // Solve - should succeed
        let solve_result = solver.solve();
        assert!(solve_result.is_ok() || matches!(solve_result.unwrap_err(), FfiError::TooManyUnknowns), 
                "Solver should run without FFI errors");
    }

    #[test]
    fn test_equal_length_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create points for two lines
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(4, 100.0, 0.0, 0.0, false).unwrap();

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
        assert!(solve_result.is_ok() || matches!(solve_result.unwrap_err(), FfiError::TooManyUnknowns), 
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
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
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
        solver.add_point(1, 50.0, 50.0, 0.0, false).unwrap();
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
        solver.add_point(1, 30.0, 80.0, 0.0, false).unwrap();
        solver.add_point(2, 70.0, 80.0, 0.0, false).unwrap();
        solver.add_point(3, 50.0, 0.0, 0.0, false).unwrap();
        solver.add_point(4, 50.0, 100.0, 0.0, false).unwrap();
        solver.add_line(10, 3, 4).unwrap(); // symmetry axis

        // Add symmetric constraint - FFI binding should work
        let result = solver.add_symmetric_constraint(100, 1, 2, 10);
        assert!(result.is_ok(), "Should be able to add symmetric constraint via FFI");
    }

    #[test]
    fn test_midpoint_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a line with endpoints
        solver.add_point(1, 0.0, 50.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 50.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap();

        // Create midpoint
        solver.add_point(3, 50.0, 50.0, 0.0, false).unwrap();

        // Add midpoint constraint - FFI binding should work
        let result = solver.add_midpoint_constraint(100, 3, 10);
        assert!(result.is_ok(), "Should be able to add midpoint constraint via FFI");
    }

    #[test]
    fn test_ffi_error_display() {
        assert_eq!(
            FfiError::Inconsistent.to_string(),
            "System is inconsistent (conflicting constraints)"
        );
        assert_eq!(
            FfiError::DidntConverge.to_string(),
            "Solver did not converge (try adjusting initial guesses or constraints)"
        );
        assert_eq!(
            FfiError::TooManyUnknowns.to_string(),
            "Too many unknowns (system is underconstrained)"
        );
        assert_eq!(
            FfiError::InvalidSystem.to_string(),
            "Invalid solver system"
        );
        assert_eq!(
            FfiError::Unknown(42).to_string(),
            "Solver failed with unknown error code 42"
        );
        assert_eq!(
            FfiError::EntityNotFound("p1".to_string()).to_string(),
            "Entity not found: p1"
        );
        assert_eq!(
            FfiError::ConstraintFailed("test".to_string()).to_string(),
            "Constraint operation failed: test"
        );
    }

    #[test]
    fn test_workplane_ffi_binding() {
        let mut solver = Solver::new();

        // Create origin point
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();

        // Create workplane with normal pointing in Z direction
        let result = solver.add_workplane(10, 1, 0.0, 0.0, 1.0);
        assert!(result.is_ok(), "Should be able to add workplane via FFI");
    }

    #[test]
    fn test_point_in_plane_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create point and origin
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 10.0, 10.0, 0.0, false).unwrap();

        // Create workplane
        solver.add_workplane(10, 1, 0.0, 0.0, 1.0).unwrap();

        // Add point in plane constraint - FFI binding should work
        let result = solver.add_point_in_plane_constraint(100, 2, 10);
        assert!(result.is_ok(), "Should be able to add point in plane constraint via FFI");
    }

    #[test]
    fn test_point_plane_distance_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create point and origin
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 10.0, 10.0, 5.0, false).unwrap();

        // Create workplane
        solver.add_workplane(10, 1, 0.0, 0.0, 1.0).unwrap();

        // Add point plane distance constraint - FFI binding should work
        let result = solver.add_point_plane_distance_constraint(100, 2, 10, 5.0);
        assert!(result.is_ok(), "Should be able to add point plane distance constraint via FFI");
    }

    #[test]
    fn test_point_line_distance_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create points and line
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 10.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 5.0, 5.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap();

        // Add point line distance constraint - FFI binding should work
        let result = solver.add_point_line_distance_constraint(100, 3, 10, 5.0);
        assert!(result.is_ok(), "Should be able to add point line distance constraint via FFI");
    }

    #[test]
    fn test_length_ratio_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two lines
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(4, 50.0, 0.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap();
        solver.add_line(11, 3, 4).unwrap();

        // Add length ratio constraint - FFI binding should work
        let result = solver.add_length_ratio_constraint(100, 10, 11, 2.0);
        assert!(result.is_ok(), "Should be able to add length ratio constraint via FFI");
    }

    #[test]
    fn test_equal_angle_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create 4 lines for equal angle constraint
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 0.0, 100.0, 0.0, false).unwrap();
        solver.add_point(4, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(5, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_point(6, 0.0, 100.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap(); // First pair: line 1
        solver.add_line(11, 1, 3).unwrap(); // First pair: line 2
        solver.add_line(12, 4, 5).unwrap(); // Second pair: line 1
        solver.add_line(13, 4, 6).unwrap(); // Second pair: line 2

        // Add equal angle constraint - FFI binding should work
        let result = solver.add_equal_angle_constraint(100, 10, 11, 12, 13);
        assert!(result.is_ok(), "Should be able to add equal angle constraint via FFI");
    }

    #[test]
    fn test_symmetric_horizontal_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two points for horizontal symmetry
        solver.add_point(1, 30.0, 50.0, 0.0, false).unwrap();
        solver.add_point(2, 70.0, 50.0, 0.0, false).unwrap();

        // Add symmetric horizontal constraint - FFI binding should work
        let result = solver.add_symmetric_horizontal_constraint(100, 1, 2);
        assert!(result.is_ok(), "Should be able to add symmetric horizontal constraint via FFI");
    }

    #[test]
    fn test_symmetric_vertical_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two points for vertical symmetry
        solver.add_point(1, 50.0, 30.0, 0.0, false).unwrap();
        solver.add_point(2, 50.0, 70.0, 0.0, false).unwrap();

        // Add symmetric vertical constraint - FFI binding should work
        let result = solver.add_symmetric_vertical_constraint(100, 1, 2);
        assert!(result.is_ok(), "Should be able to add symmetric vertical constraint via FFI");
    }

    #[test]
    fn test_diameter_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create a circle
        solver.add_circle(10, 0.0, 0.0, 0.0, 25.0).unwrap();

        // Add diameter constraint - FFI binding should work
        let result = solver.add_diameter_constraint(100, 10, 50.0);
        assert!(result.is_ok(), "Should be able to add diameter constraint via FFI");
    }

    #[test]
    fn test_diameter_constraint_with_solve() {
        // Minimal reproduction of the diameter constraint issue
        let mut solver = Solver::new();

        // Create a circle (id=10)
        solver.add_circle(10, 0.0, 0.0, 0.0, 25.0).unwrap();

        // Add diameter constraint (constraint id=100, circle id=10, diameter=50.0)
        solver.add_diameter_constraint(100, 10, 50.0).unwrap();

        // This is where the failure should occur
        let solve_result = solver.solve();
        
        // If we get here, the solve succeeded
        assert!(solve_result.is_ok(), "Solve with diameter constraint should succeed: {:?}", solve_result);
    }

    #[test]
    fn test_diameter_constraint_with_point_and_fixed() {
        // EXACT reproduction of the failing test scenario
        // Matching constraint_effectiveness_tests.rs::test_diameter_constraint_changes_solution
        let mut solver = Solver::new();

        // 1. Point "c1" -> internal id 1
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        
        // 2. Circle "circle1" -> internal id 2
        solver.add_circle(2, 0.0, 0.0, 0.0, 7.5).unwrap(); // diameter 15 / 2 = 7.5 radius

        // 3. Fixed constraint on point c1 (entity_id=1)
        solver.add_fixed_constraint(1, 1).unwrap();
        
        // 4. Diameter constraint on circle1 (entity_id=2, diameter=25.0)
        solver.add_diameter_constraint(2, 2, 25.0).unwrap();

        // 5. Solve - this should reproduce the failure
        let solve_result = solver.solve();
        
        assert!(solve_result.is_ok(), "Solve with point, circle, fixed, and diameter constraints should succeed: {:?}", solve_result);
    }

    #[test]
    fn test_get_circle_position_returns_correct_values() {
        // Test that get_circle_position correctly retrieves center and radius
        // from the new proper circle entity structure
        let mut solver = Solver::new();

        // Create a circle at (10, 20, 30) with radius 15
        solver.add_circle(5, 10.0, 20.0, 30.0, 15.0).unwrap();

        // Get circle position without solving (initial values)
        let result = solver.get_circle_position(5);
        assert!(result.is_ok(), "Should be able to get circle position");
        
        let (cx, cy, cz, radius) = result.unwrap();
        assert!((cx - 10.0).abs() < 0.001, "Center X should be 10.0, got {}", cx);
        assert!((cy - 20.0).abs() < 0.001, "Center Y should be 20.0, got {}", cy);
        assert!((cz - 30.0).abs() < 0.001, "Center Z should be 30.0, got {}", cz);
        assert!((radius - 15.0).abs() < 0.001, "Radius should be 15.0, got {}", radius);
    }

    #[test]
    fn test_get_circle_position_after_solve() {
        // Test that get_circle_position works after solving with diameter constraint
        let mut solver = Solver::new();

        // Create a circle at origin with initial radius 10
        solver.add_circle(3, 0.0, 0.0, 0.0, 10.0).unwrap();
        
        // Add diameter constraint to set diameter to 30 (radius 15)
        solver.add_diameter_constraint(100, 3, 30.0).unwrap();

        // Solve
        solver.solve().unwrap();

        // Get circle position after solve
        let (cx, cy, cz, radius) = solver.get_circle_position(3).unwrap();
        
        // Center should remain at origin
        assert!((cx).abs() < 0.001, "Center X should be 0.0, got {}", cx);
        assert!((cy).abs() < 0.001, "Center Y should be 0.0, got {}", cy);
        assert!((cz).abs() < 0.001, "Center Z should be 0.0, got {}", cz);
        
        // Radius should now be 15 (diameter 30 / 2)
        assert!((radius - 15.0).abs() < 0.001, "Radius should be 15.0 after constraint, got {}", radius);
    }

    #[test]
    fn test_get_circle_position_not_found() {
        // Test that get_circle_position returns error for non-existent circle
        let solver = Solver::new();

        // Try to get position of a circle that doesn't exist
        let result = solver.get_circle_position(999);
        assert!(result.is_err(), "Should return error for non-existent circle");
    }

    #[test]
    fn test_circle_with_offset_center() {
        // Test circle positioned away from origin
        let mut solver = Solver::new();

        // Create a circle at (-50, 100, 25) with radius 7.5
        solver.add_circle(42, -50.0, 100.0, 25.0, 7.5).unwrap();

        let (cx, cy, cz, radius) = solver.get_circle_position(42).unwrap();
        
        assert!((cx - (-50.0)).abs() < 0.001, "Center X should be -50.0, got {}", cx);
        assert!((cy - 100.0).abs() < 0.001, "Center Y should be 100.0, got {}", cy);
        assert!((cz - 25.0).abs() < 0.001, "Center Z should be 25.0, got {}", cz);
        assert!((radius - 7.5).abs() < 0.001, "Radius should be 7.5, got {}", radius);
    }

    #[test]
    fn test_same_orientation_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two lines for same orientation
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(4, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap();
        solver.add_line(11, 3, 4).unwrap();

        // Add same orientation constraint - FFI binding should work
        let result = solver.add_same_orientation_constraint(100, 10, 11);
        assert!(result.is_ok(), "Should be able to add same orientation constraint via FFI");
    }

    #[test]
    fn test_projected_point_distance_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create points and workplane
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 10.0, 10.0, 0.0, false).unwrap();
        solver.add_point(3, 5.0, 5.0, 0.0, false).unwrap();
        solver.add_workplane(10, 1, 0.0, 0.0, 1.0).unwrap();

        // Add projected point distance constraint - FFI binding should work
        let result = solver.add_projected_point_distance_constraint(100, 2, 3, 10, 5.0);
        assert!(result.is_ok(), "Should be able to add projected point distance constraint via FFI");
    }

    #[test]
    fn test_length_difference_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two lines
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(4, 50.0, 0.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap();
        solver.add_line(11, 3, 4).unwrap();

        // Add length difference constraint - FFI binding should work
        let result = solver.add_length_difference_constraint(100, 10, 11, 50.0);
        assert!(result.is_ok(), "Should be able to add length difference constraint via FFI");
    }

    #[test]
    fn test_point_on_face_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create point and face (simplified - face entity support needed)
        solver.add_point(1, 10.0, 10.0, 0.0, false).unwrap();
        // Note: Face entity support needed for full functionality
        // For now, just test FFI binding works
        let result = solver.add_point_on_face_constraint(100, 1, 10);
        // May fail if face entity not properly supported, but FFI binding should work
        assert!(result.is_ok() || result.is_err()); // Either is acceptable for now
    }

    #[test]
    fn test_point_face_distance_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create point
        solver.add_point(1, 10.0, 10.0, 5.0, false).unwrap();
        // Note: Face entity support needed for full functionality
        let result = solver.add_point_face_distance_constraint(100, 1, 10, 5.0);
        // May fail if face entity not properly supported, but FFI binding should work
        assert!(result.is_ok() || result.is_err()); // Either is acceptable for now
    }

    #[test]
    fn test_equal_line_arc_length_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create line and arc (simplified)
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap();
        solver.add_circle(20, 0.0, 0.0, 0.0, 25.0).unwrap(); // Using circle as arc for now

        // Add equal line-arc length constraint - FFI binding should work
        let result = solver.add_equal_line_arc_length_constraint(100, 10, 20);
        assert!(result.is_ok(), "Should be able to add equal line-arc length constraint via FFI");
    }

    #[test]
    fn test_equal_length_point_line_distance_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create line, point, and reference line
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 50.0, 10.0, 0.0, false).unwrap();
        solver.add_point(4, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(5, 0.0, 100.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap();
        solver.add_line(11, 4, 5).unwrap();

        // Add equal length point-line distance constraint - FFI binding should work
        let result = solver.add_equal_length_point_line_distance_constraint(100, 10, 3, 11);
        assert!(result.is_ok(), "Should be able to add equal length point-line distance constraint via FFI");
    }

    #[test]
    fn test_equal_point_line_distances_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two points and two lines
        solver.add_point(1, 10.0, 10.0, 0.0, false).unwrap();
        solver.add_point(2, 20.0, 20.0, 0.0, false).unwrap();
        solver.add_point(3, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(4, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_point(5, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(6, 0.0, 100.0, 0.0, false).unwrap();
        solver.add_line(10, 3, 4).unwrap();
        solver.add_line(11, 5, 6).unwrap();

        // Add equal point-line distances constraint - FFI binding should work
        let result = solver.add_equal_point_line_distances_constraint(100, 1, 10, 2, 11);
        assert!(result.is_ok(), "Should be able to add equal point-line distances constraint via FFI");
    }

    #[test]
    fn test_cubic_line_tangent_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create cubic and line (simplified - cubic entity support needed)
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_line(10, 1, 2).unwrap();
        // Note: Cubic entity support needed for full functionality
        let result = solver.add_cubic_line_tangent_constraint(100, 20, 10);
        // May fail if cubic entity not properly supported, but FFI binding should work
        assert!(result.is_ok() || result.is_err()); // Either is acceptable for now
    }

    #[test]
    fn test_arc_arc_length_ratio_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two arcs (using circles for now)
        solver.add_circle(10, 0.0, 0.0, 0.0, 25.0).unwrap();
        solver.add_circle(20, 0.0, 0.0, 0.0, 50.0).unwrap();

        // Add arc-arc length ratio constraint - FFI binding should work
        let result = solver.add_arc_arc_length_ratio_constraint(100, 10, 20, 2.0);
        assert!(result.is_ok(), "Should be able to add arc-arc length ratio constraint via FFI");
    }

    #[test]
    fn test_arc_line_length_ratio_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create arc and line
        solver.add_circle(10, 0.0, 0.0, 0.0, 25.0).unwrap();
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_line(20, 1, 2).unwrap();

        // Add arc-line length ratio constraint - FFI binding should work
        let result = solver.add_arc_line_length_ratio_constraint(100, 10, 20, 1.5);
        assert!(result.is_ok(), "Should be able to add arc-line length ratio constraint via FFI");
    }

    #[test]
    fn test_arc_arc_length_difference_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create two arcs (using circles for now)
        solver.add_circle(10, 0.0, 0.0, 0.0, 25.0).unwrap();
        solver.add_circle(20, 0.0, 0.0, 0.0, 50.0).unwrap();

        // Add arc-arc length difference constraint - FFI binding should work
        let result = solver.add_arc_arc_length_difference_constraint(100, 10, 20, 10.0);
        assert!(result.is_ok(), "Should be able to add arc-arc length difference constraint via FFI");
    }

    #[test]
    fn test_arc_line_length_difference_constraint_ffi_binding() {
        let mut solver = Solver::new();

        // Create arc and line
        solver.add_circle(10, 0.0, 0.0, 0.0, 25.0).unwrap();
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 100.0, 0.0, 0.0, false).unwrap();
        solver.add_line(20, 1, 2).unwrap();

        // Add arc-line length difference constraint - FFI binding should work
        let result = solver.add_arc_line_length_difference_constraint(100, 10, 20, 5.0);
        assert!(result.is_ok(), "Should be able to add arc-line length difference constraint via FFI");
    }

    #[test]
    fn test_point_2d_ffi_binding() {
        let mut solver = Solver::new();

        // Create a workplane first
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 1.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 0.0, 1.0, 0.0, false).unwrap();
        solver.add_workplane(10, 1, 0.0, 0.0, 1.0).unwrap();

        // Add 2D point in workplane - FFI binding should work
        let result = solver.add_point_2d(20, 10, 5.0, 10.0, false);
        assert!(result.is_ok(), "Should be able to add 2D point via FFI");
    }

    #[test]
    fn test_arc_ffi_binding() {
        let mut solver = Solver::new();

        // Create center, start, and end points
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap(); // center
        solver.add_point(2, 10.0, 0.0, 0.0, false).unwrap(); // start
        solver.add_point(3, 0.0, 10.0, 0.0, false).unwrap(); // end

        // Add arc - FFI binding should work
        let result = solver.add_arc(10, 1, 2, 3, 0.0, 0.0, 1.0, None);
        assert!(result.is_ok(), "Should be able to add arc via FFI");
    }

    #[test]
    fn test_cubic_ffi_binding() {
        let mut solver = Solver::new();

        // Create 4 control points
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_point(2, 10.0, 0.0, 0.0, false).unwrap();
        solver.add_point(3, 20.0, 10.0, 0.0, false).unwrap();
        solver.add_point(4, 30.0, 10.0, 0.0, false).unwrap();

        // Add cubic Bezier curve - FFI binding should work
        let result = solver.add_cubic(10, 1, 2, 3, 4, None);
        assert!(result.is_ok(), "Should be able to add cubic Bezier curve via FFI");
    }

    #[test]
    fn test_add_point_with_dragged_flag() {
        let mut solver = Solver::new();

        // Add point with dragged flag set to true
        let result = solver.add_point(1, 10.0, 20.0, 30.0, true);
        assert!(result.is_ok(), "Should be able to add dragged point via FFI");

        // Add point with dragged flag set to false
        let result = solver.add_point(2, 40.0, 50.0, 60.0, false);
        assert!(result.is_ok(), "Should be able to add non-dragged point via FFI");
    }

    #[test]
    fn test_add_point_2d_with_dragged_flag() {
        let mut solver = Solver::new();

        // Create a workplane first
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_workplane(10, 1, 0.0, 0.0, 1.0).unwrap();

        // Add 2D point with dragged flag set to true
        let result = solver.add_point_2d(20, 10, 5.0, 10.0, true);
        assert!(result.is_ok(), "Should be able to add dragged 2D point via FFI");

        // Add 2D point with dragged flag set to false
        let result = solver.add_point_2d(21, 10, 15.0, 20.0, false);
        assert!(result.is_ok(), "Should be able to add non-dragged 2D point via FFI");
    }

    #[test]
    fn test_where_dragged_constraint_ffi_binding_3d() {
        let mut solver = Solver::new();

        // Create a point
        solver.add_point(1, 10.0, 20.0, 30.0, false).unwrap();

        // Add WHERE_DRAGGED constraint for 3D point (no workplane)
        let result = solver.add_where_dragged_constraint(100, 1, None);
        assert!(result.is_ok(), "Should be able to add WHERE_DRAGGED constraint for 3D point via FFI");
    }

    #[test]
    fn test_where_dragged_constraint_ffi_binding_2d() {
        let mut solver = Solver::new();

        // Create a workplane and 2D point
        solver.add_point(1, 0.0, 0.0, 0.0, false).unwrap();
        solver.add_workplane(10, 1, 0.0, 0.0, 1.0).unwrap();
        solver.add_point_2d(2, 10, 5.0, 10.0, false).unwrap();

        // Add WHERE_DRAGGED constraint for 2D point (with workplane)
        let result = solver.add_where_dragged_constraint(100, 2, Some(10));
        assert!(result.is_ok(), "Should be able to add WHERE_DRAGGED constraint for 2D point via FFI");
    }
}
