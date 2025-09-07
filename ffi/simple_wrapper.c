#include <stdlib.h>
#include <string.h>
#include "slvs.h"

#define MAX_PARAMS 1000
#define MAX_ENTITIES 1000  
#define MAX_CONSTRAINTS 1000

typedef struct {
    Slvs_System sys;
    Slvs_Param params[MAX_PARAMS];
    Slvs_Entity entities[MAX_ENTITIES];
    Slvs_Constraint constraints[MAX_CONSTRAINTS];
    int next_param_id;
    int next_entity_id;
    int next_constraint_id;
} SimpleSystem;

SimpleSystem* real_slvs_create() {
    SimpleSystem* s = (SimpleSystem*)calloc(1, sizeof(SimpleSystem));
    
    // Point system to our arrays
    s->sys.param = s->params;
    s->sys.entity = s->entities;
    s->sys.constraint = s->constraints;
    
    // Start IDs at 1
    s->next_param_id = 1;
    s->next_entity_id = 1;
    s->next_constraint_id = 1;
    
    return s;
}

void real_slvs_destroy(SimpleSystem* s) {
    if (s) {
        free(s);
    }
}

int real_slvs_add_point(SimpleSystem* s, int id, double x, double y, double z) {
    if (s->sys.params + 3 > MAX_PARAMS || s->sys.entities + 1 > MAX_ENTITIES) {
        return -1;
    }
    
    Slvs_hGroup g = 1;
    
    // Add parameters for the point
    int px = s->next_param_id++;
    int py = s->next_param_id++;
    int pz = s->next_param_id++;
    
    s->params[s->sys.params++] = Slvs_MakeParam(px, g, x);
    s->params[s->sys.params++] = Slvs_MakeParam(py, g, y);
    s->params[s->sys.params++] = Slvs_MakeParam(pz, g, z);
    
    // Add the point entity
    s->entities[s->sys.entities++] = Slvs_MakePoint3d(id, g, px, py, pz);
    
    return 0;
}

int real_slvs_add_line(SimpleSystem* s, int id, int point1_id, int point2_id) {
    if (s->sys.entities + 1 > MAX_ENTITIES) {
        return -1;
    }
    
    Slvs_hGroup g = 1;
    s->entities[s->sys.entities++] = Slvs_MakeLineSegment(id, g, SLVS_FREE_IN_3D, point1_id, point2_id);
    
    return 0;
}

int real_slvs_add_circle(SimpleSystem* s, int id, double cx, double cy, double cz, double radius) {
    // For now, just add as a point with the radius ignored
    return real_slvs_add_point(s, id, cx, cy, cz);
}

int real_slvs_add_fixed_constraint(SimpleSystem* s, int id, int entity_id) {
    if (s->sys.constraints + 1 > MAX_CONSTRAINTS) {
        return -1;
    }
    
    Slvs_hGroup g = 1;
    s->constraints[s->sys.constraints++] = Slvs_MakeConstraint(
        id, g, SLVS_C_WHERE_DRAGGED, SLVS_FREE_IN_3D,
        0.0, entity_id, 0, 0, 0
    );
    
    return 0;
}

int real_slvs_add_distance_constraint(SimpleSystem* s, int id, int entity1, int entity2, double distance) {
    if (s->sys.constraints + 1 > MAX_CONSTRAINTS) {
        return -1;
    }
    
    Slvs_hGroup g = 1;
    s->constraints[s->sys.constraints++] = Slvs_MakeConstraint(
        id, g, SLVS_C_PT_PT_DISTANCE, SLVS_FREE_IN_3D,
        distance, entity1, entity2, 0, 0
    );
    
    return 0;
}

int real_slvs_solve(SimpleSystem* s) {
    Slvs_Solve(&s->sys, 1);
    return s->sys.result;
}

int real_slvs_get_point_position(SimpleSystem* s, int id, double* x, double* y, double* z) {
    // Find the entity
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->entities[i].h == id && s->entities[i].type == SLVS_E_POINT_IN_3D) {
            // Get parameter IDs
            Slvs_hParam px = s->entities[i].param[0];
            Slvs_hParam py = s->entities[i].param[1];
            Slvs_hParam pz = s->entities[i].param[2];
            
            // Find parameter values
            for (int j = 0; j < s->sys.params; j++) {
                if (s->params[j].h == px) *x = s->params[j].val;
                if (s->params[j].h == py) *y = s->params[j].val;
                if (s->params[j].h == pz) *z = s->params[j].val;
            }
            return 0;
        }
    }
    
    // Not found - return zeros
    *x = *y = *z = 0.0;
    return -1;
}

int real_slvs_get_circle_position(SimpleSystem* s, int id, double* cx, double* cy, double* cz, double* radius) {
    // Just get point position, radius is not supported yet
    *radius = 0.0;
    return real_slvs_get_point_position(s, id, cx, cy, cz);
}