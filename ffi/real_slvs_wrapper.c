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