#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "slvs.h"

typedef struct {
    Slvs_System sys;
    int next_param;
    int next_entity;  
    int next_constraint;
    // Track circle radii separately since we're using point entities for now
    double circle_radii[1000];  // Simple array to store radii by entity ID
    // Arrays for dragged entities
    Slvs_hParam dragged[4];
    // Array for failed constraints
    Slvs_hConstraint failed[1000];
} RealSlvsSystem;

RealSlvsSystem* real_slvs_create() {
    RealSlvsSystem* s = (RealSlvsSystem*)calloc(1, sizeof(RealSlvsSystem));
    
    // Allocate arrays
    s->sys.param = (Slvs_Param*)calloc(1000, sizeof(Slvs_Param));
    s->sys.entity = (Slvs_Entity*)calloc(1000, sizeof(Slvs_Entity));
    s->sys.constraint = (Slvs_Constraint*)calloc(1000, sizeof(Slvs_Constraint));
    
    // Set up dragged array (NULL means no dragged parameters)
    s->sys.dragged = NULL;
    s->sys.ndragged = 0;
    
    // Set up failed array (NULL means no space for failed constraints)
    s->sys.failed = NULL;
    s->sys.faileds = 0;
    s->sys.calculateFaileds = 0;
    
    // Start with higher IDs to avoid conflicts
    s->next_param = 100;
    s->next_entity = 100;
    s->next_constraint = 100;
    
    return s;
}

void real_slvs_destroy(RealSlvsSystem* s) {
    if (s) {
        free(s->sys.param);
        free(s->sys.entity);
        free(s->sys.constraint);
        free(s);
    }
}

int real_slvs_add_circle(RealSlvsSystem* s, int id, double cx, double cy, double cz, double radius) {
    Slvs_hGroup g = 1;
    
    // For now, just create a 3D point at the center
    // This simplifies things and avoids workplane issues
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

int real_slvs_add_distance_constraint(RealSlvsSystem* s, int id, int entity1, int entity2, double distance) {
    Slvs_hGroup g = 1;
    
    // Convert entity IDs to our internal IDs
    Slvs_hEntity point1 = (Slvs_hEntity)(1000 + entity1);
    Slvs_hEntity point2 = (Slvs_hEntity)(1000 + entity2);
    
    // Use a unique constraint ID
    Slvs_hConstraint constraint_id = 10000 + id;
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g,
        SLVS_C_PT_PT_DISTANCE,
        SLVS_FREE_IN_3D,
        distance,
        point1, point2, 0, 0);
    
    return 0;
}

int real_slvs_solve(RealSlvsSystem* s) {
    Slvs_Solve(&s->sys, 1);
    return s->sys.result;
}

int real_slvs_add_point(RealSlvsSystem* s, int id, double x, double y, double z) {
    Slvs_hGroup g = 1;
    
    // Create parameters for the point
    int px = s->next_param++;
    int py = s->next_param++;
    int pz = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(px, g, x);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(py, g, y);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pz, g, z);
    
    // Use a unique entity ID based on input id
    Slvs_hEntity entity_id = 1000 + id;
    s->sys.entity[s->sys.entities++] = Slvs_MakePoint3d(entity_id, g, px, py, pz);
    
    return 0;
}

int real_slvs_add_line(RealSlvsSystem* s, int id, int point1_id, int point2_id) {
    Slvs_hGroup g = 1;
    
    // Convert point IDs to our internal entity IDs
    Slvs_hEntity point1 = 1000 + point1_id;
    Slvs_hEntity point2 = 1000 + point2_id;
    
    // Use a unique entity ID for the line
    Slvs_hEntity line_id = 1000 + id;
    s->sys.entity[s->sys.entities++] = Slvs_MakeLineSegment(line_id, g, SLVS_FREE_IN_3D, point1, point2);
    
    return 0;
}

int real_slvs_add_fixed_constraint(RealSlvsSystem* s, int id, int entity_id) {
    Slvs_hGroup g = 1;
    
    // Convert entity ID to our internal ID  
    Slvs_hEntity entity = 1000 + entity_id;
    
    // Use a unique constraint ID
    Slvs_hConstraint constraint_id = 10000 + id;
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g,
        SLVS_C_WHERE_DRAGGED,
        SLVS_FREE_IN_3D,
        0.0,
        entity, 0, 0, 0);
    
    return 0;
}

int real_slvs_get_point_position(RealSlvsSystem* s, int id, double* x, double* y, double* z) {
    // Find the 3D point entity
    Slvs_hEntity entity_id = (Slvs_hEntity)(1000 + id);
    
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entity[i].h == entity_id && s->sys.entity[i].type == SLVS_E_POINT_IN_3D) {
            // Get the parameter handles
            Slvs_hParam px = s->sys.entity[i].param[0];
            Slvs_hParam py = s->sys.entity[i].param[1];
            Slvs_hParam pz = s->sys.entity[i].param[2];
            
            // Find the parameter values
            for (int j = 0; j < s->sys.params; j++) {
                if (s->sys.param[j].h == px) *x = s->sys.param[j].val;
                if (s->sys.param[j].h == py) *y = s->sys.param[j].val;
                if (s->sys.param[j].h == pz) *z = s->sys.param[j].val;
            }
            return 0;
        }
    }
    
    // Couldn't find the entity - return defaults
    *x = 0;
    *y = 0;
    *z = 0;
    return 0;
}

int real_slvs_get_circle_position(RealSlvsSystem* s, int id, double* cx, double* cy, double* cz, double* radius) {
    // Get the stored radius
    if (id < 1000) {
        *radius = s->circle_radii[id];
    } else {
        *radius = 0;
    }
    
    // Find the 3D point entity
    Slvs_hEntity entity_id = (Slvs_hEntity)(1000 + id);
    
    // Find the 3D point entity
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entity[i].h == entity_id && s->sys.entity[i].type == SLVS_E_POINT_IN_3D) {
            // Get the parameter handles
            Slvs_hParam px = s->sys.entity[i].param[0];
            Slvs_hParam py = s->sys.entity[i].param[1];
            Slvs_hParam pz = s->sys.entity[i].param[2];
            
            // Find the parameter values
            for (int j = 0; j < s->sys.params; j++) {
                if (s->sys.param[j].h == px) *cx = s->sys.param[j].val;
                if (s->sys.param[j].h == py) *cy = s->sys.param[j].val;
                if (s->sys.param[j].h == pz) *cz = s->sys.param[j].val;
            }
            return 0;
        }
    }
    
    // Couldn't find the entity - return defaults
    *cx = 0;
    *cy = 0;
    *cz = 0;
    return 0;
}