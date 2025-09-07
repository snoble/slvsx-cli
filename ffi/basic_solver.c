// A basic constraint solver for circles with distance constraints
// This is a simplified implementation to demonstrate the concept

#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdio.h>

typedef struct {
    int id;
    double x, y, z;
    double radius;
    int fixed;
} Circle;

typedef struct {
    int id;
    int circle1_id;
    int circle2_id;
    double distance;
} DistanceConstraint;

typedef struct {
    Circle* circles;
    int num_circles;
    int max_circles;
    
    DistanceConstraint* constraints;
    int num_constraints;
    int max_constraints;
} BasicSolverSystem;

BasicSolverSystem* basic_solver_create() {
    BasicSolverSystem* sys = (BasicSolverSystem*)calloc(1, sizeof(BasicSolverSystem));
    sys->max_circles = 100;
    sys->circles = (Circle*)calloc(sys->max_circles, sizeof(Circle));
    sys->max_constraints = 200;
    sys->constraints = (DistanceConstraint*)calloc(sys->max_constraints, sizeof(DistanceConstraint));
    return sys;
}

void basic_solver_destroy(BasicSolverSystem* sys) {
    if (sys) {
        free(sys->circles);
        free(sys->constraints);
        free(sys);
    }
}

int basic_solver_add_circle(BasicSolverSystem* sys, int id, double x, double y, double z, double radius) {
    if (sys->num_circles >= sys->max_circles) return -1;
    
    Circle* c = &sys->circles[sys->num_circles++];
    c->id = id;
    c->x = x;
    c->y = y;
    c->z = z;
    c->radius = radius;
    c->fixed = 0;
    
    return 0;
}

int basic_solver_add_distance_constraint(BasicSolverSystem* sys, int id, int circle1_id, int circle2_id, double distance) {
    if (sys->num_constraints >= sys->max_constraints) return -1;
    
    DistanceConstraint* con = &sys->constraints[sys->num_constraints++];
    con->id = id;
    con->circle1_id = circle1_id;
    con->circle2_id = circle2_id;
    con->distance = distance;
    
    return 0;
}

Circle* find_circle(BasicSolverSystem* sys, int id) {
    for (int i = 0; i < sys->num_circles; i++) {
        if (sys->circles[i].id == id) {
            return &sys->circles[i];
        }
    }
    return NULL;
}

// Simple iterative solver using gradient descent
int basic_solver_solve(BasicSolverSystem* sys) {
    const int max_iterations = 1000;
    const double tolerance = 1e-6;
    const double step_size = 0.1;
    
    for (int iter = 0; iter < max_iterations; iter++) {
        double total_error = 0.0;
        
        // For each constraint, adjust positions
        for (int i = 0; i < sys->num_constraints; i++) {
            DistanceConstraint* con = &sys->constraints[i];
            Circle* c1 = find_circle(sys, con->circle1_id);
            Circle* c2 = find_circle(sys, con->circle2_id);
            
            if (!c1 || !c2) continue;
            
            // Calculate current distance
            double dx = c2->x - c1->x;
            double dy = c2->y - c1->y;
            double dz = c2->z - c1->z;
            double current_dist = sqrt(dx*dx + dy*dy + dz*dz);
            
            // Calculate error
            double error = con->distance - current_dist;
            total_error += fabs(error);
            
            if (fabs(error) > tolerance && current_dist > 0.001) {
                // Normalize direction
                dx /= current_dist;
                dy /= current_dist;
                dz /= current_dist;
                
                // Apply correction
                double correction = error * step_size * 0.5;
                
                if (!c1->fixed) {
                    c1->x -= dx * correction;
                    c1->y -= dy * correction;
                    c1->z -= dz * correction;
                }
                
                if (!c2->fixed) {
                    c2->x += dx * correction;
                    c2->y += dy * correction;
                    c2->z += dz * correction;
                }
            }
        }
        
        if (total_error < tolerance) {
            return 0; // Converged
        }
    }
    
    return 1; // Did not converge
}

int basic_solver_get_circle_position(BasicSolverSystem* sys, int id, double* x, double* y, double* z, double* radius) {
    Circle* c = find_circle(sys, id);
    if (!c) return -1;
    
    *x = c->x;
    *y = c->y;
    *z = c->z;
    *radius = c->radius;
    
    return 0;
}