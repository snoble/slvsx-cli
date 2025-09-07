#ifndef SLVS_WRAPPER_H
#define SLVS_WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SolverSystem SolverSystem;

// System management
SolverSystem* slvs_create_system(void);
void slvs_free_system(SolverSystem* sys);

// Add entities
int slvs_add_point(SolverSystem* sys, int id, double x, double y, double z);
int slvs_add_circle(SolverSystem* sys, int id, double cx, double cy, double cz, double radius);

// Add constraints
int slvs_add_distance_constraint(SolverSystem* sys, int id, int entity1, int entity2, double distance);

// Solve
int slvs_solve(SolverSystem* sys);

// Get results
int slvs_get_circle_pos(SolverSystem* sys, int id, double* cx, double* cy, double* cz, double* radius);

#ifdef __cplusplus
}
#endif

#endif // SLVS_WRAPPER_H