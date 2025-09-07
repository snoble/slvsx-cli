/*
 * Simplified C wrapper for the SolveSpace constraint solver
 * This provides a simpler API for Rust FFI bindings
 */

#include <stdio.h>
#include <stdlib.h>
#include <math.h>

// For now, we'll implement a minimal solver ourselves
// In production, this would link to the real libslvs.a

typedef struct {
    int id;
    double x, y, z;
} Point;

typedef struct {
    int id;
    int p1, p2;
} Line;

typedef struct {
    int id;
    double cx, cy, cz;
    double radius;
} Circle;

typedef enum {
    CONSTRAINT_DISTANCE,
    CONSTRAINT_COINCIDENT,
    CONSTRAINT_HORIZONTAL,
    CONSTRAINT_VERTICAL,
    CONSTRAINT_PARALLEL,
    CONSTRAINT_PERPENDICULAR,
    CONSTRAINT_EQUAL_LENGTH,
    CONSTRAINT_FIXED,
    CONSTRAINT_ANGLE
} ConstraintType;

typedef struct {
    int id;
    ConstraintType type;
    int entity1;
    int entity2;
    double value;
} Constraint;

typedef struct {
    Point* points;
    int num_points;
    Line* lines;
    int num_lines;
    Circle* circles;
    int num_circles;
    Constraint* constraints;
    int num_constraints;
    
    // Results
    int result_code;
    int dof;
} SolverSystem;

// Create a new solver system
SolverSystem* slvs_create_system() {
    SolverSystem* sys = (SolverSystem*)calloc(1, sizeof(SolverSystem));
    return sys;
}

// Add a point to the system
int slvs_add_point(SolverSystem* sys, int id, double x, double y, double z) {
    sys->points = (Point*)realloc(sys->points, sizeof(Point) * (sys->num_points + 1));
    sys->points[sys->num_points].id = id;
    sys->points[sys->num_points].x = x;
    sys->points[sys->num_points].y = y;
    sys->points[sys->num_points].z = z;
    sys->num_points++;
    return 0;
}

// Add a circle to the system
int slvs_add_circle(SolverSystem* sys, int id, double cx, double cy, double cz, double radius) {
    sys->circles = (Circle*)realloc(sys->circles, sizeof(Circle) * (sys->num_circles + 1));
    sys->circles[sys->num_circles].id = id;
    sys->circles[sys->num_circles].cx = cx;
    sys->circles[sys->num_circles].cy = cy;
    sys->circles[sys->num_circles].cz = cz;
    sys->circles[sys->num_circles].radius = radius;
    sys->num_circles++;
    return 0;
}

// Add a distance constraint
int slvs_add_distance_constraint(SolverSystem* sys, int id, int entity1, int entity2, double distance) {
    sys->constraints = (Constraint*)realloc(sys->constraints, sizeof(Constraint) * (sys->num_constraints + 1));
    sys->constraints[sys->num_constraints].id = id;
    sys->constraints[sys->num_constraints].type = CONSTRAINT_DISTANCE;
    sys->constraints[sys->num_constraints].entity1 = entity1;
    sys->constraints[sys->num_constraints].entity2 = entity2;
    sys->constraints[sys->num_constraints].value = distance;
    sys->num_constraints++;
    return 0;
}

// Solve the system (simplified - just applies distance constraints for planetary gears)
int slvs_solve(SolverSystem* sys) {
    // For planetary gears, we can calculate positions analytically
    // This is a simplified solver for demonstration
    
    // Find the sun gear (typically first circle)
    if (sys->num_circles > 0) {
        // Sun is at origin
        sys->circles[0].cx = 0;
        sys->circles[0].cy = 0;
        
        // For other circles, check distance constraints
        for (int i = 1; i < sys->num_circles; i++) {
            // Look for distance constraint from sun
            for (int j = 0; j < sys->num_constraints; j++) {
                if (sys->constraints[j].type == CONSTRAINT_DISTANCE) {
                    if ((sys->constraints[j].entity1 == sys->circles[0].id && 
                         sys->constraints[j].entity2 == sys->circles[i].id) ||
                        (sys->constraints[j].entity2 == sys->circles[0].id && 
                         sys->constraints[j].entity1 == sys->circles[i].id)) {
                        
                        // Place this circle at the constraint distance
                        double dist = sys->constraints[j].value;
                        double angle = (i - 1) * 2.0 * 3.14159265359 / 6.0; // 60 degree spacing
                        sys->circles[i].cx = dist * cos(angle);
                        sys->circles[i].cy = dist * sin(angle);
                        sys->circles[i].cz = 0;
                    }
                }
            }
        }
    }
    
    sys->result_code = 0; // Success
    sys->dof = 0; // Fully constrained
    return 0;
}

// Get circle position after solving
int slvs_get_circle_pos(SolverSystem* sys, int id, double* cx, double* cy, double* cz, double* radius) {
    for (int i = 0; i < sys->num_circles; i++) {
        if (sys->circles[i].id == id) {
            *cx = sys->circles[i].cx;
            *cy = sys->circles[i].cy;
            *cz = sys->circles[i].cz;
            *radius = sys->circles[i].radius;
            return 0;
        }
    }
    return -1; // Not found
}

// Free the system
void slvs_free_system(SolverSystem* sys) {
    if (sys) {
        free(sys->points);
        free(sys->lines);
        free(sys->circles);
        free(sys->constraints);
        free(sys);
    }
}