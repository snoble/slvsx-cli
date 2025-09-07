#include <stdio.h>
#include "slvs.h"

int main() {
    printf("Testing libslvs FFI...\n");
    
    // Create a simple system
    Slvs_System sys = {0};
    Slvs_Param params[10];
    Slvs_Entity entities[10];
    Slvs_Constraint constraints[10];
    
    sys.param = params;
    sys.entity = entities;
    sys.constraint = constraints;
    
    // Add two points
    sys.param[sys.params++] = Slvs_MakeParam(1, 1, 0.0);
    sys.param[sys.params++] = Slvs_MakeParam(2, 1, 0.0);
    sys.param[sys.params++] = Slvs_MakeParam(3, 1, 0.0);
    
    sys.param[sys.params++] = Slvs_MakeParam(4, 1, 10.0);
    sys.param[sys.params++] = Slvs_MakeParam(5, 1, 0.0);
    sys.param[sys.params++] = Slvs_MakeParam(6, 1, 0.0);
    
    sys.entity[sys.entities++] = Slvs_MakePoint3d(101, 1, 1, 2, 3);
    sys.entity[sys.entities++] = Slvs_MakePoint3d(102, 1, 4, 5, 6);
    
    // Add distance constraint
    sys.constraint[sys.constraints++] = Slvs_MakeConstraint(
        201, 1, SLVS_C_PT_PT_DISTANCE, SLVS_FREE_IN_3D,
        5.0, 101, 102, 0, 0
    );
    
    printf("System created with %d params, %d entities, %d constraints\n", 
           sys.params, sys.entities, sys.constraints);
    
    // Solve
    printf("Solving...\n");
    Slvs_Solve(&sys, 1);
    
    printf("Result: %d\n", sys.result);
    printf("DOF: %d\n", sys.dof);
    
    // Check new positions
    printf("Point 1: (%f, %f, %f)\n", params[0].val, params[1].val, params[2].val);
    printf("Point 2: (%f, %f, %f)\n", params[3].val, params[4].val, params[5].val);
    
    return 0;
}