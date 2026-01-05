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

// Forward declaration
static void normal_to_quaternion(double nx, double ny, double nz, double* qw, double* qx, double* qy, double* qz);

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
    
    // Allocate space for dragged parameters array
    s->sys.dragged = (Slvs_hParam*)calloc(1000, sizeof(Slvs_hParam));
    if (!s->sys.dragged) {
        free(s->sys.param);
        free(s->sys.entity);
        free(s->sys.constraint);
        free(s);
        return NULL;
    }
    s->sys.ndragged = 0;
    
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
        if (s->sys.dragged) free(s->sys.dragged);
        free(s);
    }
}

// Add WHERE_DRAGGED constraint (locks point to current position)
int real_slvs_add_where_dragged_constraint(RealSlvsSystem* s, int id,
                                           int point_id, int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity wrkpl = (workplane_id >= 0) ? (1000 + workplane_id) : SLVS_FREE_IN_3D;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_WHERE_DRAGGED, wrkpl,
        0, point, 0, 0, 0);
    
    return 0;
}

// Add a 3D point
int real_slvs_add_point(RealSlvsSystem* s, int id, double x, double y, double z, int is_dragged) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Create parameters for the point coordinates
    int px = s->next_param++;
    int py = s->next_param++;
    int pz = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(px, g, x);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(py, g, y);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pz, g, z);
    
    // If dragged, mark these parameters as dragged
    if (is_dragged && s->sys.ndragged < 997) {  // Leave room for 3 params
        s->sys.dragged[s->sys.ndragged++] = px;
        s->sys.dragged[s->sys.ndragged++] = py;
        s->sys.dragged[s->sys.ndragged++] = pz;
    }
    
    // Create the point entity with 1000+ offset like working version
    Slvs_hEntity entity_id = 1000 + id;
    s->sys.entity[s->sys.entities++] = Slvs_MakePoint3d(entity_id, g, px, py, pz);
    
    return 0;
}

// Add a line between two points (3D line)
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

// Add a 2D line between two 2D points in a workplane
int real_slvs_add_line_2d(RealSlvsSystem* s, int id, int point1_id, int point2_id, int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Create 2D line segment entity with proper ID mapping
    Slvs_hEntity line_id = 1000 + id;
    Slvs_hEntity p1 = 1000 + point1_id;
    Slvs_hEntity p2 = 1000 + point2_id;
    Slvs_hEntity wrkpl = (workplane_id > 0) ? (1000 + workplane_id) : SLVS_FREE_IN_3D;
    s->sys.entity[s->sys.entities++] = Slvs_MakeLineSegment(line_id, g, wrkpl, p1, p2);
    
    return 0;
}

// Add a 2D point in a workplane
int real_slvs_add_point_2d(RealSlvsSystem* s, int id, int workplane_id, double u, double v, int is_dragged) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Create parameters for 2D coordinates
    int pu = s->next_param++;
    int pv = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pu, g, u);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pv, g, v);
    
    // If dragged, mark these parameters as dragged
    if (is_dragged && s->sys.ndragged < 998) {  // Leave room for 2 params
        s->sys.dragged[s->sys.ndragged++] = pu;
        s->sys.dragged[s->sys.ndragged++] = pv;
    }
    
    // Create 2D point entity
    Slvs_hEntity entity_id = 1000 + id;
    Slvs_hEntity wp = 1000 + workplane_id;
    s->sys.entity[s->sys.entities++] = Slvs_MakePoint2d(entity_id, g, wp, pu, pv);
    
    return 0;
}

// Add a circle (simplified - just stores center point and radius)
int real_slvs_add_circle(RealSlvsSystem* s, int id, double cx, double cy, double cz, double radius) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Create normal entity first (default to Z-axis normal)
    double qw, qx, qy, qz;
    normal_to_quaternion(0.0, 0.0, 1.0, &qw, &qx, &qy, &qz);
    
    int pqw = s->next_param++;
    int pqx = s->next_param++;
    int pqy = s->next_param++;
    int pqz = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pqw, g, qw);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pqx, g, qx);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pqy, g, qy);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pqz, g, qz);
    
    Slvs_hEntity normal_id = 3000 + id; // Use different range for normal
    s->sys.entity[s->sys.entities++] = Slvs_MakeNormal3d(normal_id, g, pqw, pqx, pqy, pqz);
    
    // Create origin point for workplane (3D point)
    int pox = s->next_param++;
    int poy = s->next_param++;
    int poz = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pox, g, cx);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(poy, g, cy);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(poz, g, cz);
    
    Slvs_hEntity origin_id = 6000 + id; // Use different range for origin
    s->sys.entity[s->sys.entities++] = Slvs_MakePoint3d(origin_id, g, pox, poy, poz);
    
    // Create workplane for the circle (required for circles)
    Slvs_hEntity workplane_id = 5000 + id; // Use different range for workplane
    s->sys.entity[s->sys.entities++] = Slvs_MakeWorkplane(workplane_id, g, origin_id, normal_id);
    
    // Create 2D center point in the workplane (u, v coordinates)
    // For simplicity, use (0, 0) as the center in the workplane coordinate system
    int pu = s->next_param++;
    int pv = s->next_param++;
    
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pu, g, 0.0);
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pv, g, 0.0);
    
    Slvs_hEntity center_id = 2000 + id; // Use different range for center point
    s->sys.entity[s->sys.entities++] = Slvs_MakePoint2d(center_id, g, workplane_id, pu, pv);
    
    // Create distance entity for radius
    int pr = s->next_param++;
    s->sys.param[s->sys.params++] = Slvs_MakeParam(pr, g, radius);
    
    Slvs_hEntity radius_id = 4000 + id; // Use different range for distance
    s->sys.entity[s->sys.entities++] = Slvs_MakeDistance(radius_id, g, SLVS_FREE_IN_3D, pr);
    
    // Create circle entity (use high offset to avoid collision with regular entities)
    Slvs_hEntity circle_id = 8000 + id;
    s->sys.entity[s->sys.entities++] = Slvs_MakeCircle(circle_id, g, workplane_id, center_id, normal_id, radius_id);
    
    // Store radius for later retrieval (for backward compatibility)
    if (id < 1000) {
        s->circle_radii[id] = radius;
    }
    
    return 0;
}

// Add a proper arc of circle
int real_slvs_add_arc(RealSlvsSystem* s, int id, int center_point_id, int start_point_id, 
                     int end_point_id, double nx, double ny, double nz, int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Convert normal vector to quaternion for normal entity
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
    Slvs_hEntity normal_id = 3000 + id; // Use different range
    s->sys.entity[s->sys.entities++] = Slvs_MakeNormal3d(normal_id, g, pqw, pqx, pqy, pqz);
    
    // Create arc entity
    Slvs_hEntity arc_id = 1000 + id;
    Slvs_hEntity center = 1000 + center_point_id;
    Slvs_hEntity start = 1000 + start_point_id;
    Slvs_hEntity end = 1000 + end_point_id;
    Slvs_hEntity wrkpl = (workplane_id >= 0) ? (1000 + workplane_id) : SLVS_FREE_IN_3D;
    
    s->sys.entity[s->sys.entities++] = Slvs_MakeArcOfCircle(arc_id, g, wrkpl, normal_id, center, start, end);
    
    return 0;
}

// Add a cubic Bezier curve
int real_slvs_add_cubic(RealSlvsSystem* s, int id, int pt0_id, int pt1_id, 
                       int pt2_id, int pt3_id, int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Create cubic entity
    Slvs_hEntity cubic_id = 1000 + id;
    Slvs_hEntity pt0 = 1000 + pt0_id;
    Slvs_hEntity pt1 = 1000 + pt1_id;
    Slvs_hEntity pt2 = 1000 + pt2_id;
    Slvs_hEntity pt3 = 1000 + pt3_id;
    Slvs_hEntity wrkpl = (workplane_id >= 0) ? (1000 + workplane_id) : SLVS_FREE_IN_3D;
    
    s->sys.entity[s->sys.entities++] = Slvs_MakeCubic(cubic_id, g, wrkpl, pt0, pt1, pt2, pt3);
    
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
// For 3D points, pass workplane_id <= 0 to use FREE_IN_3D
// For 2D points, pass the workplane ID
int real_slvs_add_fixed_constraint(RealSlvsSystem* s, int id, int entity_id, int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Convert entity ID to internal ID with 1000+ offset
    Slvs_hEntity e = 1000 + entity_id;
    Slvs_hEntity workplane = (workplane_id > 0) ? (1000 + workplane_id) : SLVS_FREE_IN_3D;
    
    // Use a unique constraint ID with large offset
    Slvs_hConstraint constraint_id = 10000 + id;
    
    // Where the point is constrained to be
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_WHERE_DRAGGED, workplane,
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
int real_slvs_add_horizontal_constraint(RealSlvsSystem* s, int id, int line_id, int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entity
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity line = 1000 + line_id;
    Slvs_hEntity workplane = (workplane_id > 0) ? (1000 + workplane_id) : SLVS_FREE_IN_3D;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_HORIZONTAL, workplane,
        0, 0, 0, line, 0);
    
    return 0;
}

// Add vertical constraint
int real_slvs_add_vertical_constraint(RealSlvsSystem* s, int id, int line_id, int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entity
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity line = 1000 + line_id;
    Slvs_hEntity workplane = (workplane_id > 0) ? (1000 + workplane_id) : SLVS_FREE_IN_3D;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_VERTICAL, workplane,
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
    Slvs_hEntity circle1 = 8000 + circle1_id;
    Slvs_hEntity circle2 = 8000 + circle2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_EQUAL_RADIUS, SLVS_FREE_IN_3D,
        0, 0, 0, circle1, circle2);
    
    return 0;
}

// Add tangent constraint (between two curves)
// Note: CURVE_CURVE_TANGENT only works for Arc+Arc, Arc+Cubic, Cubic+Cubic
// For Arc+Line use SLVS_C_ARC_LINE_TANGENT
// For Cubic+Line use SLVS_C_CUBIC_LINE_TANGENT
int real_slvs_add_tangent_constraint(RealSlvsSystem* s, int id, int entity1_id, int entity2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity entity1 = 1000 + entity1_id;
    Slvs_hEntity entity2 = 1000 + entity2_id;
    
    // Detect entity types to choose the correct constraint type
    int entity1_type = -1, entity2_type = -1;
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entity[i].h == entity1) entity1_type = s->sys.entity[i].type;
        if (s->sys.entity[i].h == entity2) entity2_type = s->sys.entity[i].type;
    }
    
    // Determine the correct constraint type based on entity types
    int constraint_type;
    Slvs_hEntity arc_entity, line_entity;
    
    bool is1_arc = (entity1_type == SLVS_E_ARC_OF_CIRCLE);
    bool is2_arc = (entity2_type == SLVS_E_ARC_OF_CIRCLE);
    bool is1_line = (entity1_type == SLVS_E_LINE_SEGMENT);
    bool is2_line = (entity2_type == SLVS_E_LINE_SEGMENT);
    bool is1_cubic = (entity1_type == SLVS_E_CUBIC);
    bool is2_cubic = (entity2_type == SLVS_E_CUBIC);
    
    if ((is1_arc && is2_line) || (is1_line && is2_arc)) {
        // Arc + Line: use ARC_LINE_TANGENT
        constraint_type = SLVS_C_ARC_LINE_TANGENT;
        if (is1_arc) {
            arc_entity = entity1;
            line_entity = entity2;
        } else {
            arc_entity = entity2;
            line_entity = entity1;
        }
        // SolveSpace expects: entityA = arc, entityB = line
        s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
            constraint_id, g, constraint_type, SLVS_FREE_IN_3D,
            0, 0, 0, arc_entity, line_entity);
    } else if ((is1_cubic && is2_line) || (is1_line && is2_cubic)) {
        // Cubic + Line: use CUBIC_LINE_TANGENT
        constraint_type = SLVS_C_CUBIC_LINE_TANGENT;
        Slvs_hEntity cubic_entity;
        if (is1_cubic) {
            cubic_entity = entity1;
            line_entity = entity2;
        } else {
            cubic_entity = entity2;
            line_entity = entity1;
        }
        // SolveSpace expects: entityA = cubic, entityB = line
        s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
            constraint_id, g, constraint_type, SLVS_FREE_IN_3D,
            0, 0, 0, cubic_entity, line_entity);
    } else {
        // Arc+Arc, Arc+Cubic, Cubic+Cubic: use CURVE_CURVE_TANGENT
        s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
            constraint_id, g, SLVS_C_CURVE_CURVE_TANGENT, SLVS_FREE_IN_3D,
            0, 0, 0, entity1, entity2);
    }
    
    return 0;
}

// Add point on circle constraint
int real_slvs_add_point_on_circle_constraint(RealSlvsSystem* s, int id, int point_id, int circle_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    
    // Use proper ID mapping for constraint and entities
    Slvs_hConstraint constraint_id = 10000 + id;
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity circle = 8000 + circle_id;
    
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
        if (s->sys.entity[i].h == internal_id) {
            if (s->sys.entity[i].type == SLVS_E_POINT_IN_3D) {
                // 3D point - get x, y, z directly from parameters
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
            } else if (s->sys.entity[i].type == SLVS_E_POINT_IN_2D) {
                // 2D point - get u, v from parameters, set z = 0
                // Note: For proper 3D coordinates, we'd need to transform through the workplane
                // For now, we return u as x, v as y, and z = 0
                double u = 0.0, v = 0.0;
                for (int j = 0; j < s->sys.params; j++) {
                    if (s->sys.param[j].h == s->sys.entity[i].param[0]) {
                        u = s->sys.param[j].val;
                    }
                    if (s->sys.param[j].h == s->sys.entity[i].param[1]) {
                        v = s->sys.param[j].val;
                    }
                }
                *x = u;
                *y = v;
                *z = 0.0;
                return 0;
            }
        }
    }
    
    return -1;
}

// Get circle position and radius after solving
int real_slvs_get_circle_position(RealSlvsSystem* s, int circle_id, double* cx, double* cy, double* cz, double* radius) {
    if (!s || !cx || !cy || !cz || !radius) return -1;
    
    // Circle entity structure (from real_slvs_add_circle):
    // - Normal at 3000 + id
    // - Origin point (workplane) at 6000 + id (this is the 3D center!)
    // - Workplane at 5000 + id
    // - 2D center point at 2000 + id (relative to workplane, always 0,0)
    // - Distance (radius) at 4000 + id
    // - Circle at 8000 + id
    
    Slvs_hEntity origin_id = 6000 + circle_id;  // Origin point is the 3D center
    Slvs_hEntity radius_entity_id = 4000 + circle_id;  // Distance entity for radius
    
    int found_center = 0;
    int found_radius = 0;
    
    // Find the origin point (which is the 3D center of the circle)
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entity[i].h == origin_id && 
            s->sys.entity[i].type == SLVS_E_POINT_IN_3D) {
            
            // Get the parameter values for the center
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
            found_center = 1;
            break;
        }
    }
    
    // Find the distance entity (radius)
    for (int i = 0; i < s->sys.entities; i++) {
        if (s->sys.entity[i].h == radius_entity_id && 
            s->sys.entity[i].type == SLVS_E_DISTANCE) {
            
            // Get the radius parameter value
            for (int j = 0; j < s->sys.params; j++) {
                if (s->sys.param[j].h == s->sys.entity[i].param[0]) {
                    *radius = s->sys.param[j].val;
                    found_radius = 1;
                    break;
                }
            }
            break;
        }
    }
    
    // Also try the stored radius as fallback
    if (!found_radius && circle_id < 1000) {
        *radius = s->circle_radii[circle_id];
        found_radius = 1;
    }
    
    if (found_center && found_radius) {
        return 0;
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
    
    // PT_IN_PLANE: point (ptA) must lie in workplane (entityA)
    // The constraint's coordinate system is FREE_IN_3D
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_IN_PLANE, SLVS_FREE_IN_3D,
        0, point, 0, wp, 0);
    
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
                                                   int entity1_id, int entity2_id,
                                                   int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity entity1 = 1000 + entity1_id;
    Slvs_hEntity entity2 = 1000 + entity2_id;
    Slvs_hEntity wp = 1000 + workplane_id;
    
    // SYMMETRIC_HORIZ requires a workplane
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_SYMMETRIC_HORIZ, wp,
        0, entity1, entity2, 0, 0);
    
    return 0;
}

// Add symmetric vertical constraint
int real_slvs_add_symmetric_vertical_constraint(RealSlvsSystem* s, int id,
                                                 int entity1_id, int entity2_id,
                                                 int workplane_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity entity1 = 1000 + entity1_id;
    Slvs_hEntity entity2 = 1000 + entity2_id;
    Slvs_hEntity wp = 1000 + workplane_id;
    
    // SYMMETRIC_VERT requires a workplane
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_SYMMETRIC_VERT, wp,
        0, entity1, entity2, 0, 0);
    
    return 0;
}

// Add diameter constraint
int real_slvs_add_diameter_constraint(RealSlvsSystem* s, int id,
                                      int circle_id, double diameter) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity circle = 8000 + circle_id;  // Match circle entity ID offset
    
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
    
    // PROJ_PT_DISTANCE: constrains distance between point1 and point2 
    // when projected onto the workplane (entityA)
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PROJ_PT_DISTANCE, SLVS_FREE_IN_3D,
        distance, point1, point2, wp, 0);
    
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

// Add point-on-face constraint (requires face entity)
int real_slvs_add_point_on_face_constraint(RealSlvsSystem* s, int id,
                                            int point_id, int face_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity face = 1000 + face_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_ON_FACE, SLVS_FREE_IN_3D,
        0, point, 0, face, 0);
    
    return 0;
}

// Add point-to-face distance constraint (requires face entity)
int real_slvs_add_point_face_distance_constraint(RealSlvsSystem* s, int id,
                                                  int point_id, int face_id,
                                                  double distance) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity face = 1000 + face_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_PT_FACE_DISTANCE, SLVS_FREE_IN_3D,
        distance, point, 0, face, 0);
    
    return 0;
}

// Add equal line-arc length constraint
int real_slvs_add_equal_line_arc_length_constraint(RealSlvsSystem* s, int id,
                                                     int line_id, int arc_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity line = 1000 + line_id;
    Slvs_hEntity arc = 1000 + arc_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_EQUAL_LINE_ARC_LEN, SLVS_FREE_IN_3D,
        0, 0, 0, line, arc);
    
    return 0;
}

// Add equal length and point-line distance constraint
int real_slvs_add_equal_length_point_line_distance_constraint(RealSlvsSystem* s, int id,
                                                                int line_id, int point_id,
                                                                int reference_line_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity line = 1000 + line_id;
    Slvs_hEntity point = 1000 + point_id;
    Slvs_hEntity ref_line = 1000 + reference_line_id;
    
    Slvs_Constraint c = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_EQ_LEN_PT_LINE_D, SLVS_FREE_IN_3D,
        0, point, 0, line, ref_line);
    s->sys.constraint[s->sys.constraints++] = c;
    
    return 0;
}

// Add equal point-line distances constraint
int real_slvs_add_equal_point_line_distances_constraint(RealSlvsSystem* s, int id,
                                                          int point1_id, int line1_id,
                                                          int point2_id, int line2_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity point1 = 1000 + point1_id;
    Slvs_hEntity line1 = 1000 + line1_id;
    Slvs_hEntity point2 = 1000 + point2_id;
    Slvs_hEntity line2 = 1000 + line2_id;
    
    Slvs_Constraint c = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_EQ_PT_LN_DISTANCES, SLVS_FREE_IN_3D,
        0, point1, point2, line1, line2);
    s->sys.constraint[s->sys.constraints++] = c;
    
    return 0;
}

// Add cubic-line tangent constraint (requires cubic entity)
int real_slvs_add_cubic_line_tangent_constraint(RealSlvsSystem* s, int id,
                                                  int cubic_id, int line_id) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity cubic = 1000 + cubic_id;
    Slvs_hEntity line = 1000 + line_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_CUBIC_LINE_TANGENT, SLVS_FREE_IN_3D,
        0, 0, 0, cubic, line);
    
    return 0;
}

// Add arc-arc length ratio constraint
int real_slvs_add_arc_arc_length_ratio_constraint(RealSlvsSystem* s, int id,
                                                   int arc1_id, int arc2_id,
                                                   double ratio) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity arc1 = 1000 + arc1_id;
    Slvs_hEntity arc2 = 1000 + arc2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_ARC_ARC_LEN_RATIO, SLVS_FREE_IN_3D,
        ratio, 0, 0, arc1, arc2);
    
    return 0;
}

// Add arc-line length ratio constraint
int real_slvs_add_arc_line_length_ratio_constraint(RealSlvsSystem* s, int id,
                                                     int arc_id, int line_id,
                                                     double ratio) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity arc = 1000 + arc_id;
    Slvs_hEntity line = 1000 + line_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_ARC_LINE_LEN_RATIO, SLVS_FREE_IN_3D,
        ratio, 0, 0, line, arc);
    
    return 0;
}

// Add arc-arc length difference constraint
int real_slvs_add_arc_arc_length_difference_constraint(RealSlvsSystem* s, int id,
                                                         int arc1_id, int arc2_id,
                                                         double difference) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity arc1 = 1000 + arc1_id;
    Slvs_hEntity arc2 = 1000 + arc2_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_ARC_ARC_DIFFERENCE, SLVS_FREE_IN_3D,
        difference, 0, 0, arc1, arc2);
    
    return 0;
}

// Add arc-line length difference constraint
int real_slvs_add_arc_line_length_difference_constraint(RealSlvsSystem* s, int id,
                                                          int arc_id, int line_id,
                                                          double difference) {
    if (!s) return -1;
    
    Slvs_hGroup g = 1;
    Slvs_hConstraint constraint_id = 10000 + id;
    
    Slvs_hEntity arc = 1000 + arc_id;
    Slvs_hEntity line = 1000 + line_id;
    
    s->sys.constraint[s->sys.constraints++] = Slvs_MakeConstraint(
        constraint_id, g, SLVS_C_ARC_LINE_DIFFERENCE, SLVS_FREE_IN_3D,
        difference, 0, 0, line, arc);
    
    return 0;
}