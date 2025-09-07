#include <stdio.h>
#include <stdlib.h>

// Direct FFI declarations matching the Rust side
typedef struct RealSlvsSystem RealSlvsSystem;

RealSlvsSystem* real_slvs_create();
void real_slvs_destroy(RealSlvsSystem* s);
int real_slvs_add_point(RealSlvsSystem* s, int id, double x, double y, double z);
int real_slvs_add_distance_constraint(RealSlvsSystem* s, int id, int entity1, int entity2, double distance);
int real_slvs_add_fixed_constraint(RealSlvsSystem* s, int id, int entity_id);
int real_slvs_solve(RealSlvsSystem* s);
int real_slvs_get_point_position(RealSlvsSystem* s, int id, double* x, double* y, double* z);

int main() {
    printf("Creating system...\n");
    RealSlvsSystem* sys = real_slvs_create();
    if (!sys) {
        printf("Failed to create system\n");
        return 1;
    }
    
    printf("Adding point 1 at (0,0,0)...\n");
    if (real_slvs_add_point(sys, 1, 0.0, 0.0, 0.0) != 0) {
        printf("Failed to add point 1\n");
        return 1;
    }
    
    printf("Adding point 2 at (10,0,0)...\n");
    if (real_slvs_add_point(sys, 2, 10.0, 0.0, 0.0) != 0) {
        printf("Failed to add point 2\n");
        return 1;
    }
    
    printf("Adding distance constraint of 5.0 between points...\n");
    if (real_slvs_add_distance_constraint(sys, 1, 1, 2, 5.0) != 0) {
        printf("Failed to add distance constraint\n");
        return 1;
    }
    
    printf("Solving...\n");
    int result = real_slvs_solve(sys);
    printf("Solve result: %d\n", result);
    
    double x, y, z;
    real_slvs_get_point_position(sys, 1, &x, &y, &z);
    printf("Point 1: (%f, %f, %f)\n", x, y, z);
    
    real_slvs_get_point_position(sys, 2, &x, &y, &z);
    printf("Point 2: (%f, %f, %f)\n", x, y, z);
    
    printf("Destroying system...\n");
    real_slvs_destroy(sys);
    
    printf("Done!\n");
    return 0;
}