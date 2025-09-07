#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "../solvespace/include/slvs.h"

// Wrapper to interface with the real SLVS library

typedef struct {
    Slvs_System sys;
    int param_id_counter;
    int entity_id_counter;
    int constraint_id_counter;
} RealSolverSystem;

RealSolverSystem* slvs_create_system() {
    RealSolverSystem* s = (RealSolverSystem*)calloc(1, sizeof(RealSolverSystem));
    
    s->sys.params = calloc(1000, sizeof(Slvs_Param));
    s->sys.entities = calloc(1000, sizeof(Slvs_Entity));
    s->sys.constraints = calloc(1000, sizeof(Slvs_Constraint));
    
    s->param_id_counter = 1;
    s->entity_id_counter = 1;
    s->constraint_id_counter = 1;
    
    return s;
}

void slvs_destroy_system(RealSolverSystem* s) {
    if (s) {
        free(s->sys.params);
        free(s->sys.entities);
        free(s->sys.constraints);
        free(s);
    }
}

int slvs_add_circle(RealSolverSystem* s, int id, double cx, double cy, double cz, double radius) {
    // Add parameters for center point
    int px_id = s->param_id_counter++;
    int py_id = s->param_id_counter++;
    int pz_id = s->param_id_counter++;
    
    s->sys.params[s->sys.params] = Slvs_MakeParam(px_id, 1, cx);
    s->sys.params++;
    s->sys.params[s->sys.params] = Slvs_MakeParam(py_id, 1, cy);
    s->sys.params++;
    s->sys.params[s->sys.params] = Slvs_MakeParam(pz_id, 1, cz);
    s->sys.params++;
    
    // Add center point entity
    int center_id = s->entity_id_counter++;
    s->sys.entities[s->sys.entities] = Slvs_MakePoint3d(center_id, 1, px_id, py_id, pz_id);
    s->sys.entities++;
    
    // Add normal (perpendicular to XY plane)
    int qw_id = s->param_id_counter++;
    int qx_id = s->param_id_counter++;
    int qy_id = s->param_id_counter++;
    int qz_id = s->param_id_counter++;
    
    s->sys.params[s->sys.params] = Slvs_MakeParam(qw_id, 1, 1.0); // quaternion w
    s->sys.params++;
    s->sys.params[s->sys.params] = Slvs_MakeParam(qx_id, 1, 0.0); // quaternion x
    s->sys.params++;
    s->sys.params[s->sys.params] = Slvs_MakeParam(qy_id, 1, 0.0); // quaternion y
    s->sys.params++;
    s->sys.params[s->sys.params] = Slvs_MakeParam(qz_id, 1, 0.0); // quaternion z
    s->sys.params++;
    
    int normal_id = s->entity_id_counter++;
    s->sys.entities[s->sys.entities] = Slvs_MakeNormal3d(normal_id, 1, qw_id, qx_id, qy_id, qz_id);
    s->sys.entities++;
    
    // Add workplane
    int workplane_id = s->entity_id_counter++;
    Slvs_Entity workplane;
    memset(&workplane, 0, sizeof(workplane));
    workplane.h = workplane_id;
    workplane.group = 1;
    workplane.type = SLVS_E_WORKPLANE;
    workplane.wrkpl = SLVS_FREE_IN_3D;
    workplane.point[0] = center_id;
    workplane.normal = normal_id;
    s->sys.entities[s->sys.entities] = workplane;
    s->sys.entities++;
    
    // Add radius parameter
    int radius_id = s->param_id_counter++;
    s->sys.params[s->sys.params] = Slvs_MakeParam(radius_id, 1, radius);
    s->sys.params++;
    
    // Add distance entity for radius
    int distance_id = s->entity_id_counter++;
    Slvs_Entity distance;
    memset(&distance, 0, sizeof(distance));
    distance.h = distance_id;
    distance.group = 1;
    distance.type = SLVS_E_DISTANCE;
    distance.wrkpl = SLVS_FREE_IN_3D;
    distance.param[0] = radius_id;
    s->sys.entities[s->sys.entities] = distance;
    s->sys.entities++;
    
    // Add circle entity
    Slvs_Entity circle;
    memset(&circle, 0, sizeof(circle));
    circle.h = id;
    circle.group = 1;
    circle.type = SLVS_E_CIRCLE;
    circle.wrkpl = workplane_id;
    circle.point[0] = center_id;
    circle.normal = normal_id;
    circle.distance = distance_id;
    s->sys.entities[s->sys.entities] = circle;
    s->sys.entities++;
    
    return 0;
}

int slvs_add_distance_constraint(RealSolverSystem* s, int id, int entity1, int entity2, double distance) {
    // For circles, we need to constrain the distance between their centers
    // This is a PT_PT_DISTANCE constraint
    
    // Find the center points of the two circles
    int center1 = -1, center2 = -1;
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entities[i].h == entity1 && s->sys.entities[i].type == SLVS_E_CIRCLE) {
            center1 = s->sys.entities[i].point[0];
        }
        if (s->sys.entities[i].h == entity2 && s->sys.entities[i].type == SLVS_E_CIRCLE) {
            center2 = s->sys.entities[i].point[0];
        }
    }
    
    if (center1 == -1 || center2 == -1) {
        return -1; // Entities not found
    }
    
    Slvs_Constraint constraint;
    memset(&constraint, 0, sizeof(constraint));
    constraint.h = id;
    constraint.group = 1;
    constraint.type = SLVS_C_PT_PT_DISTANCE;
    constraint.wrkpl = SLVS_FREE_IN_3D;
    constraint.valA = distance;
    constraint.ptA = center1;
    constraint.ptB = center2;
    
    s->sys.constraints[s->sys.constraints] = constraint;
    s->sys.constraints++;
    
    return 0;
}

int slvs_solve(RealSolverSystem* s) {
    Slvs_Solve(&s->sys, 1);
    return s->sys.result;
}

int slvs_get_circle_position(RealSolverSystem* s, int id, double* cx, double* cy, double* cz, double* radius) {
    // Find the circle entity
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entities[i].h == id && s->sys.entities[i].type == SLVS_E_CIRCLE) {
            // Get center point
            int center_id = s->sys.entities[i].point[0];
            
            // Find center point entity
            for (int j = 0; j < s->sys.entities; j++) {
                if (s->sys.entities[j].h == center_id && s->sys.entities[j].type == SLVS_E_POINT_IN_3D) {
                    // Get parameter IDs
                    int px = s->sys.entities[j].param[0];
                    int py = s->sys.entities[j].param[1];
                    int pz = s->sys.entities[j].param[2];
                    
                    // Get parameter values
                    for (int k = 0; k < s->sys.params; k++) {
                        if (s->sys.params[k].h == px) *cx = s->sys.params[k].val;
                        if (s->sys.params[k].h == py) *cy = s->sys.params[k].val;
                        if (s->sys.params[k].h == pz) *cz = s->sys.params[k].val;
                    }
                }
            }
            
            // Get radius
            int distance_id = s->sys.entities[i].distance;
            for (int j = 0; j < s->sys.entities; j++) {
                if (s->sys.entities[j].h == distance_id && s->sys.entities[j].type == SLVS_E_DISTANCE) {
                    int radius_param = s->sys.entities[j].param[0];
                    for (int k = 0; k < s->sys.params; k++) {
                        if (s->sys.params[k].h == radius_param) {
                            *radius = s->sys.params[k].val;
                            break;
                        }
                    }
                }
            }
            
            return 0;
        }
    }
    
    return -1; // Entity not found
}