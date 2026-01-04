#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdio.h>
#include "slvs.h"

// Structure to hold the SolveSpace system
typedef struct {
    Slvs_System sys;
    int next_param;
    int next_entity;
    int next_constraint;
    double circle_radii[1000];  // Store circle radii
} RealSlvsSystem;

// Create a new system
RealSlvsSystem* real_slvs_create() {
    RealSlvsSystem* s = (RealSlvsSystem*)calloc(1, sizeof(RealSlvsSystem));
    if (!s) return NULL;
    
    // Allocate space for parameters, entities, and constraints
    s->sys.param = (Slvs_Param*)calloc(5000, sizeof(Slvs_Param));
    s->sys.entity = (Slvs_Entity*)calloc(5000, sizeof(Slvs_Entity));
    s->sys.constraint = (Slvs_Constraint*)calloc(5000, sizeof(Slvs_Constraint));
    
    if (!s->sys.param || !s->sys.entity || !s->sys.constraint) {
        free(s->sys.param);
        free(s->sys.entity);
        free(s->sys.constraint);
        free(s);
        return NULL;
    }
    
    s->sys.params = 0;
    s->sys.entities = 0;
    s->sys.constraints = 0;
    s->sys.calculateFaileds = 0;
    
    // Start numbering from higher values to avoid conflicts
    // Use different ranges to prevent ID collisions
    s->next_param = 10000;  // Parameters: 10000+
    s->next_entity = 100;   // Entities stay at 100+
    s->next_constraint = 100; // Constraints stay at 100+
    
    return s;
}

// Destroy the system
void real_slvs_destroy(RealSlvsSystem* s) {
    if (s) {
        if (s->sys.param) free(s->sys.param);
        if (s->sys.entity) free(s->sys.entity);
        if (s->sys.constraint) free(s->sys.constraint);
        free(s);
    }
}

// Add a 3D point
int real_slvs_add_point(RealSlvsSystem* s, int id, double x, double y, double z) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Create parameters for the point coordinates
    int px = s->next_param++;
    int py = s->next_param++;
    int pz = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(px, g, x);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(py, g, y);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pz, g, z);
    
    // Create the point entity with 1000+ offset like working version
    Slvs_hEntity entity_id = 1000 + id;
    s->sys.entity[s->sys.entities++] = Slvs_MakePoint3d(entity_id, g, px, py, pz);
    
    return 0;
}

// Add a line between two points
int real_slvs_add_line(RealSlvsSystem* s, int id, int point1_id, int point2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Create line segment entity with proper ID mapping
    Slvs_hEntity line_id = 1000 + id;
    Slvs_hEntity p1 = 1000 + point1_id;
    Slvs_hEntity p2 = 1000 + point2_id;
    s->sys.entity[s->sys.entities++] = Slvs_MakeLineSegment(line_id, g, 
        SLVS_FREE_IN_3D, p1, p2);
    
    return 0;
}

// Add a circle (simplified - just stores center point and radius)
int real_slvs_add_circle(RealSlvsSystem* s, int id, double cx, double cy, double cz, double radius) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // For simplicity, create a 3D point at the center
    int px = s->next_param++;
    int py = s->next_param++;
    int pz = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(px, g, cx);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(py, g, cy);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pz, g, cz);
    
    // Use a unique entity ID based on input id
    Slvs_hEntity entity_id = 1000 + id;
    s->sys.entity[s->sys.entities++] = Slvs_MakePoint3d(entity_id, g, px, py, pz);
    
    // Store radius for later retrieval
    if (id < 1000) {
        s->circle_radii[id] = radius;
    }
    
    return 0;
}

// Add a distance constraint
int real_slvs_add_distance_constraint(RealSlvsSystem* s, int id, int entity1, int entity2, double distance) {
    if (!s) return -1;
    
    
    Slvs_hGroup g = 1;
    
    // Convert entity IDs to internal IDs with 1000+ offset
    Slvs_hEntity point1 = 1000 + entity1;
    Slvs_hEntity point2 = 1000 + entity2;
    
    // Use a unique constraint ID with large offset
    Slvs_hConstraint constraint_id = 10000 + id;
    
    // Add distance constraint - pass distance directly as valParam like working version
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_PT_DISTANCE, SLVS_FREE_IN_3D,
        distance, point1, point2, 0, 0);
    
    
    return 0;
}

// Add a fixed constraint
int real_slvs_add_fixed_constraint(RealSlvsSystem* s, int id, int entity_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Convert entity ID to internal ID with 1000+ offset
    Slvs_hEntity e = 1000 + entity_id;
    
    // Use a unique constraint ID with large offset
    Slvs_hConstraint constraint_id = 10000 + id;
    
    // Where the point is constrained to be
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_WHERE_DRAGGED, SLVS_FREE_IN_3D,
        0, e, 0, 0, 0);
    
    return 0;
}

// Add parallel constraint
int real_slvs_add_parallel_constraint(RealSlvsSystem* s, int id, int line1_id, int line2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity line1 = 1000 + line1_id;
    Slvs_hEntity line2 = 1000 + line2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PARALLEL, SLVS_FREE_IN_3D,
        0, 0, 0, line1, line2);
    
    return 0;
}

// Add perpendicular constraint
int real_slvs_add_perpendicular_constraint(RealSlvsSystem* s, int id, int line1_id, int line2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity line1 = 1000 + line1_id;
    Slvs_hEntity line2 = 1000 + line2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PERPENDICULAR, SLVS_FREE_IN_3D,
        0, 0, 0, line1, line2);
    
    return 0;
}

// Add angle constraint
int real_slvs_add_angle_constraint(RealSlvsSystem* s, int id, int line1_id, int line2_id, double angle) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Create angle parameter (in degrees)
    int angle_param = s->next_param++;
    s->sys.param[s->sys.params++] = Slvs_MakeParam(angle_param, g, angle);
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity line1 = 1000 + line1_id;
    Slvs_hEntity line2 = 1000 + line2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_ANGLE, SLVS_FREE_IN_3D,
        angle_param, 0, 0, line1, line2);
    
    return 0;
}

// Add horizontal constraint
int real_slvs_add_horizontal_constraint(RealSlvsSystem* s, int id, int line_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entity
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity line = 1000 + line_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_HORIZONTAL, SLVS_FREE_IN_3D,
        0, 0, 0, line, 0);
    
    return 0;
}

// Add vertical constraint
int real_slvs_add_vertical_constraint(RealSlvsSystem* s, int id, int line_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entity
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity line = 1000 + line_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_VERTICAL, SLVS_FREE_IN_3D,
        0, 0, 0, line, 0);
    
    return 0;
}

// Add equal length constraint (between two lines)
int real_slvs_add_equal_length_constraint(RealSlvsSystem* s, int id, int line1_id, int line2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity line1 = 1000 + line1_id;
    Slvs_hEntity line2 = 1000 + line2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_EQUAL_LENGTH_LINES, SLVS_FREE_IN_3D,
        0, 0, 0, line1, line2);
    
    return 0;
}

// Add equal radius constraint (between two circles/arcs)
int real_slvs_add_equal_radius_constraint(RealSlvsSystem* s, int id, int circle1_id, int circle2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity circle1 = 1000 + circle1_id;
    Slvs_hEntity circle2 = 1000 + circle2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_EQUAL_RADIUS, SLVS_FREE_IN_3D,
        0, 0, 0, circle1, circle2);
    
    return 0;
}

// Add tangent constraint (between two curves: line/circle/arc)
int real_slvs_add_tangent_constraint(RealSlvsSystem* s, int id, int entity1_id, int entity2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity entity1 = 1000 + entity1_id;
    Slvs_hEntity entity2 = 1000 + entity2_id;
    
    // Use CURVE_CURVE_TANGENT for general curve-to-curve tangency
    // This works for line-circle, circle-circle, arc-line, etc.
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_CURVE_CURVE_TANGENT, SLVS_FREE_IN_3D,
        0, 0, 0, entity1, entity2);
    
    return 0;
}

// Add point on circle constraint
int real_slvs_add_point_on_circle_constraint(RealSlvsSystem* s, int id, int point_id, int circle_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity circle = 1000 + circle_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_ON_CIRCLE, SLVS_FREE_IN_3D,
        0, point, 0, circle, 0);
    
    return 0;
}

// Add symmetric constraint (two entities symmetric about a line)
int real_slvs_add_symmetric_constraint(RealSlvsSystem* s, int id, int entity1_id, int entity2_id, int line_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity entity1 = 1000 + entity1_id;
    Slvs_hEntity entity2 = 1000 + entity2_id;
    Slvs_hEntity line = 1000 + line_id;
    
    // Use SYMMETRIC_LINE for symmetric about a line
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_SYMMETRIC_LINE, SLVS_FREE_IN_3D,
        0, entity1, entity2, line, 0);
    
    return 0;
}

// Add midpoint constraint (point at midpoint of line)
int real_slvs_add_midpoint_constraint(RealSlvsSystem* s, int id, int point_id, int line_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity line = 1000 + line_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_AT_MIDPOINT, SLVS_FREE_IN_3D,
        0, point, 0, line, 0);
    
    return 0;
}

// Add point on line constraint
int real_slvs_add_point_on_line_constraint(RealSlvsSystem* s, int id, int point_id, int line_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity line = 1000 + line_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_ON_LINE, SLVS_FREE_IN_3D,
        0, point, 0, line, 0);
    
    return 0;
}

// Add points coincident constraint
int real_slvs_add_points_coincident_constraint(RealSlvsSystem* s, int id, int point1_id, int point2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity point1 = 1000 + point1_id;
    Slvs_hEntity point2 = 1000 + point2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_POINTS_COINCIDENT, SLVS_FREE_IN_3D,
        0, point1, point2, 0, 0);
    
    return 0;
}

// Solve the system
int real_slvs_solve(RealSlvsSystem* s) {
    if (!s) return -1;
    
    // Solve the system for group 1 (default group)
    Slvs_Solve(&s->sys, 1);
    
    // Return status (0 = success, 1 = inconsistent, 2 = didn't converge, 3 = too many unknowns)
    if (s->sys.result == SLVS_RESULT_OKAY) {
        return 0;
    } else if (s->sys.result == SLVS_RESULT_INCONSISTENT) {
        return 1;
    } else if (s->sys.result == SLVS_RESULT_DIDNT_CONVERGE) {
        return 2;
    } else if (s->sys.result == SLVS_RESULT_TOO_MANY_UNKNOWNS) {
        return 3;
    }
    
    return -1;
}

// Get point position after solving
int real_slvs_get_point_position(RealSlvsSystem* s, int point_id, double* x, double* y, double* z) {
    if (!s || !x || !y || !z) return -1;
    
    // Find the point entity (look for internal ID with 1000+ offset)
    Slvs_hEntity internal_id = 1000 + point_id;
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entity[i].h == internal_id && 
            s->sys.entity[i].type == SLVS_E_POINT_IN_3D) {
            
            // Get the parameter values
            for (int j = 0; j < s->sys.params; j++) {
                if (s->sys.param[j].h == s->sys.entity[i].param[0]) {
                    *x = s->sys.param[j].val;
                }
                if (s->sys.param[j].h == s->sys.entity[i].param[1]) {
                    *y = s->sys.param[j].val;
                }
                if (s->sys.param[j].h == s->sys.entity[i].param[2]) {
                    *z = s->sys.param[j].val;
                }
            }
            return 0;
        }
    }
    
    return -1;
}

// Get circle position and radius after solving
int real_slvs_get_circle_position(RealSlvsSystem* s, int circle_id, double* cx, double* cy, double* cz, double* radius) {
    if (!s || !cx || !cy || !cz || !radius) return -1;
    
    // For our simplified circles, get the center point
    Slvs_hEntity entity_id = 1000 + circle_id;
    
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entity[i].h == entity_id && 
            s->sys.entity[i].type == SLVS_E_POINT_IN_3D) {
            
            // Get the parameter values
            for (int j = 0; j < s->sys.params; j++) {
                if (s->sys.param[j].h == s->sys.entity[i].param[0]) {
                    *cx = s->sys.param[j].val;
                }
                if (s->sys.param[j].h == s->sys.entity[i].param[1]) {
                    *cy = s->sys.param[j].val;
                }
                if (s->sys.param[j].h == s->sys.entity[i].param[2]) {
                    *cz = s->sys.param[j].val;
                }
            }
            
            // Get stored radius
            if (circle_id < 1000) {
                *radius = s->circle_radii[circle_id];
            }
            return 0;
        }
    }
    
    return -1;
}

// Get solver DOF (degrees of freedom)
int real_slvs_get_dof(RealSlvsSystem* s) {
    if (!s) return -1;
    return s->sys.dof;
}

// Helper function to convert a normal vector to a quaternion
// The quaternion represents the rotation from default Z-axis (0,0,1) to the desired normal
static void normal_to_quaternion(double nx, double ny, double nz, double* qw, double* qx, double* qy, double* qz) {
    // Normalize the input vector
    double len = sqrt(nx*nx + ny*ny + nz*nz);
    if (len < 1e-10) {
        // Default to Z-axis if zero vector
        *qw = 1.0; *qx = 0.0; *qy = 0.0; *qz = 0.0;
        return;
    }
    nx /= len;
    ny /= len;
    nz /= len;
    
    // Default Z-axis
    double zx = 0.0, zy = 0.0, zz = 1.0;
    
    // If normal is already Z-axis, use identity quaternion
    if (fabs(nx) < 1e-10 && fabs(ny) < 1e-10 && fabs(nz - 1.0) < 1e-10) {
        *qw = 1.0; *qx = 0.0; *qy = 0.0; *qz = 0.0;
        return;
    }
    
    // If normal is opposite Z-axis, rotate 180 degrees around X-axis
    if (fabs(nx) < 1e-10 && fabs(ny) < 1e-10 && fabs(nz + 1.0) < 1e-10) {
        *qw = 0.0; *qx = 1.0; *qy = 0.0; *qz = 0.0;
        return;
    }
    
    // General case: compute quaternion from axis-angle representation
    // Axis is cross product of Z-axis and normal
    double ax = zy * nz - zz * ny;
    double ay = zz * nx - zx * nz;
    double az = zx * ny - zy * nx;
    
    double axis_len = sqrt(ax*ax + ay*ay + az*az);
    if (axis_len < 1e-10) {
        // Vectors are parallel, use identity
        *qw = 1.0; *qx = 0.0; *qy = 0.0; *qz = 0.0;
        return;
    }
    
    // Angle between vectors
    double dot = zx*nx + zy*ny + zz*nz;
    double angle = acos(fmax(-1.0, fmin(1.0, dot)));
    
    // Quaternion from axis-angle: q = (cos(θ/2), sin(θ/2) * axis)
    double half_angle = angle / 2.0;
    *qw = cos(half_angle);
    double sin_half = sin(half_angle);
    *qx = (ax / axis_len) * sin_half;
    *qy = (ay / axis_len) * sin_half;
    *qz = (az / axis_len) * sin_half;
}

// Add a workplane (plane entity)
// Creates a normal from the normal vector and a workplane entity
int real_slvs_add_workplane(RealSlvsSystem* s, int id, int origin_point_id, 
                            double nx, double ny, double nz) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Convert normal vector to quaternion
    double qw, qx, qy, qz;
    normal_to_quaternion(nx, ny, nz, &qw, &qx, &qy, &qz);
    
    // Create parameters for the quaternion
    int pqw = s->next_param++;
    int pqx = s->next_param++;
    int pqy = s->next_param++;
    int pqz = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pqw, g, qw);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pqx, g, qx);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pqy, g, qy);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pqz, g, qz);
    
    // Create normal entity
    Slvs_hEntity normal_id = 2000 + id; // Use different range to avoid conflicts
    s->sys.entity[s->sys.entities++] = Slvs_MakeNormal3d(normal_id, g, pqw, pqx, pqy, pqz);
    
    // Create workplane entity
    Slvs_hEntity wp_id = 1000 + id;
    Slvs_hEntity origin = 1000 + origin_point_id;
    s->sys.entity[s->sys.entities++] = Slvs_MakeWorkplane(wp_id, g, origin, normal_id);
    
    return 0;
}

// Add point-in-plane constraint
int real_slvs_add_point_in_plane_constraint(RealSlvsSystem* s, int id, 
                                            int point_id, int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity wp = 1000 + workplane_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_IN_PLANE, wp,
        0, point, 0, 0, 0);
    
    return 0;
}

// Add point-to-plane distance constraint
int real_slvs_add_point_plane_distance_constraint(RealSlvsSystem* s, int id,
                                                   int point_id, int workplane_id,
                                                   double distance) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity wp = 1000 + workplane_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_PLANE_DISTANCE, SLVS_FREE_IN_3D,
        distance, point, 0, wp, 0);
    
    return 0;
}

// Add point-to-line distance constraint
int real_slvs_add_point_line_distance_constraint(RealSlvsSystem* s, int id,
                                                  int point_id, int line_id,
                                                  double distance) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity line = 1000 + line_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_LINE_DISTANCE, SLVS_FREE_IN_3D,
        distance, point, 0, line, 0);
    
    return 0;
}

// Add length ratio constraint
int real_slvs_add_length_ratio_constraint(RealSlvsSystem* s, int id,
                                          int line1_id, int line2_id,
                                          double ratio) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity line1 = 1000 + line1_id;
    Slvs_hEntity line2 = 1000 + line2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_LENGTH_RATIO, SLVS_FREE_IN_3D,
        ratio, 0, 0, line1, line2);
    
    return 0;
}

// Add equal angle constraint
int real_slvs_add_equal_angle_constraint(RealSlvsSystem* s, int id,
                                         int line1_id, int line2_id,
                                         int line3_id, int line4_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity line1 = 1000 + line1_id;
    Slvs_hEntity line2 = 1000 + line2_id;
    Slvs_hEntity line3 = 1000 + line3_id;
    Slvs_hEntity line4 = 1000 + line4_id;
    
    Slvs_Constraint c = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_EQUAL_ANGLE, SLVS_FREE_IN_3D,
        0, 0, 0, line1, line2);
    c.entityC = line3;
    c.entityD = line4;
    s->sys.constraint[s->sys.constraints++] = c;
    
    return 0;
}

// Add symmetric horizontal constraint
int real_slvs_add_symmetric_horizontal_constraint(RealSlvsSystem* s, int id,
                                                   int entity1_id, int entity2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity entity1 = 1000 + entity1_id;
    Slvs_hEntity entity2 = 1000 + entity2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_SYMMETRIC_HORIZ, SLVS_FREE_IN_3D,
        0, entity1, entity2, 0, 0);
    
    return 0;
}

// Add symmetric vertical constraint
int real_slvs_add_symmetric_vertical_constraint(RealSlvsSystem* s, int id,
                                                 int entity1_id, int entity2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity entity1 = 1000 + entity1_id;
    Slvs_hEntity entity2 = 1000 + entity2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_SYMMETRIC_VERT, SLVS_FREE_IN_3D,
        0, entity1, entity2, 0, 0);
    
    return 0;
}

// Add diameter constraint
int real_slvs_add_diameter_constraint(RealSlvsSystem* s, int id,
                                      int circle_id, double diameter) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity circle = 1000 + circle_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_DIAMETER, SLVS_FREE_IN_3D,
        diameter, 0, 0, circle, 0);
    
    return 0;
}

// Add same orientation constraint
int real_slvs_add_same_orientation_constraint(RealSlvsSystem* s, int id,
                                              int entity1_id, int entity2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity entity1 = 1000 + entity1_id;
    Slvs_hEntity entity2 = 1000 + entity2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_SAME_ORIENTATION, SLVS_FREE_IN_3D,
        0, 0, 0, entity1, entity2);
    
    return 0;
}

// Add projected point distance constraint
int real_slvs_add_projected_point_distance_constraint(RealSlvsSystem* s, int id,
                                                       int point1_id, int point2_id,
                                                       int workplane_id, double distance) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity point1 = 1000 + point1_id;
    Slvs_hEntity point2 = 1000 + point2_id;
    Slvs_hEntity wp = 1000 + workplane_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PROJ_PT_DISTANCE, wp,
        distance, point1, point2, 0, 0);
    
    return 0;
}

// Add length difference constraint
int real_slvs_add_length_difference_constraint(RealSlvsSystem* s, int id,
                                                int line1_id, int line2_id,
                                                double difference) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity line1 = 1000 + line1_id;
    Slvs_hEntity line2 = 1000 + line2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_LENGTH_DIFFERENCE, SLVS_FREE_IN_3D,
        difference, 0, 0, line1, line2);
    
    return 0;
}