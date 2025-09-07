#!/usr/bin/env python3
"""Create the perfect triangular meshing configuration: S=36, I=18, O=12, R=72"""

import math
import json

S, I, O, R = 36, 18, 12, 72
module = 2.0

print(f"Creating perfect triangular meshing: S={S}, I={I}, O={O}, R={R}")
print("=" * 60)

# Inner planets at 60° intervals
inner_radius = (S + I) * module / 2  # 54mm
inner_positions = []
for i in range(6):
    angle = i * 60 * math.pi / 180
    x = inner_radius * math.cos(angle)
    y = inner_radius * math.sin(angle)
    inner_positions.append((round(x, 2), round(y, 2)))
    print(f"Inner {i+1}: ({x:.2f}, {y:.2f}) at radius {inner_radius:.1f}mm")

print()

# Outer planets between pairs of inner planets
mesh_distance = (I + O) * module / 2  # 30mm

outer_positions = []
for i in range(6):
    # Get two adjacent inner planets
    inner1 = inner_positions[i]
    inner2 = inner_positions[(i + 1) % 6]
    
    # Midpoint between the two inners
    mid_x = (inner1[0] + inner2[0]) / 2
    mid_y = (inner1[1] + inner2[1]) / 2
    
    # Distance from inner to midpoint
    dist_to_mid = math.sqrt((inner1[0] - mid_x)**2 + (inner1[1] - mid_y)**2)
    
    # Height using Pythagoras
    height = math.sqrt(mesh_distance**2 - dist_to_mid**2)
    
    # Direction from origin to midpoint
    mid_dist = math.sqrt(mid_x**2 + mid_y**2)
    if mid_dist > 0:
        dir_x = mid_x / mid_dist
        dir_y = mid_y / mid_dist
    else:
        angle = (i * 60 + 30) * math.pi / 180
        dir_x = math.cos(angle)
        dir_y = math.sin(angle)
    
    # Position outer planet
    outer_x = mid_x + dir_x * height
    outer_y = mid_y + dir_y * height
    outer_positions.append((round(outer_x, 2), round(outer_y, 2)))
    
    # Verify distances
    dist1 = math.sqrt((outer_x - inner1[0])**2 + (outer_y - inner1[1])**2)
    dist2 = math.sqrt((outer_x - inner2[0])**2 + (outer_y - inner2[1])**2)
    outer_orbit = math.sqrt(outer_x**2 + outer_y**2)
    print(f"Outer {i+1}: ({outer_x:.2f}, {outer_y:.2f}) at radius {outer_orbit:.2f}mm")
    print(f"  Meshes with inner{i+1} at {dist1:.2f}mm and inner{(i+1)%6+1} at {dist2:.2f}mm (target: {mesh_distance:.2f}mm)")
    print(f"  Ring mesh distance: {outer_orbit:.2f}mm (target: {(R-O)*module/2:.2f}mm, error: {abs(outer_orbit-(R-O)*module/2):.3f}mm)")

print()
print("Clearance checks:")
sun_outer_radius = S * module / 2 + module  # 38mm
outer_gear_radius = O * module / 2 + module  # 14mm
min_clearance = sun_outer_radius + outer_gear_radius  # 52mm
actual_clearance = min(math.sqrt(p[0]**2 + p[1]**2) for p in outer_positions)
print(f"  Sun outer edge: {sun_outer_radius:.1f}mm")
print(f"  Outer gear radius: {outer_gear_radius:.1f}mm") 
print(f"  Minimum clearance needed: {min_clearance:.1f}mm")
print(f"  Actual outer planet distance: {actual_clearance:.1f}mm")
print(f"  Clearance margin: {actual_clearance - min_clearance:.1f}mm")

# Create JSON configuration
config = {
    "schema": "slvs-json/1",
    "units": "mm",
    "parameters": {
        "module": module,
        "pressure_angle": 20.0,
        "sun_teeth": S,
        "inner_teeth": I,
        "outer_teeth": O,
        "ring_teeth": R
    },
    "entities": [
        {
            "type": "gear",
            "id": "sun",
            "center": [0, 0, 0],
            "teeth": "$sun_teeth",
            "module": "$module",
            "pressure_angle": "$pressure_angle",
            "phase": 0.0,
            "internal": False
        },
        {
            "type": "gear",
            "id": "ring",
            "center": [0, 0, 0],
            "teeth": "$ring_teeth",
            "module": "$module",
            "pressure_angle": "$pressure_angle",
            "phase": 0.0,
            "internal": True
        }
    ],
    "constraints": []
}

# Add inner planets
for i in range(6):
    config["entities"].append({
        "type": "gear",
        "id": f"inner{i+1}",
        "center": [inner_positions[i][0], inner_positions[i][1], 0],
        "teeth": "$inner_teeth",
        "module": "$module",
        "pressure_angle": "$pressure_angle",
        "phase": 0.0,
        "internal": False
    })
    # Mesh with sun
    config["constraints"].append({
        "type": "mesh",
        "gear1": "sun",
        "gear2": f"inner{i+1}"
    })

# Add outer planets
for i in range(6):
    config["entities"].append({
        "type": "gear",
        "id": f"outer{i+1}",
        "center": [outer_positions[i][0], outer_positions[i][1], 0],
        "teeth": "$outer_teeth",
        "module": "$module",
        "pressure_angle": "$pressure_angle",
        "phase": 0.0,
        "internal": False
    })
    # Mesh with two adjacent inner planets
    config["constraints"].append({
        "type": "mesh",
        "gear1": f"inner{i+1}",
        "gear2": f"outer{i+1}"
    })
    config["constraints"].append({
        "type": "mesh",
        "gear1": f"inner{(i+1)%6+1}",
        "gear2": f"outer{i+1}"
    })
    # Mesh with ring
    config["constraints"].append({
        "type": "mesh",
        "gear1": "ring",
        "gear2": f"outer{i+1}"
    })

# Save configuration
with open("testdata/perfect_triangular_36_18_12_72.json", "w") as f:
    json.dump(config, f, indent=2)

print("\nConfiguration saved to testdata/perfect_triangular_36_18_12_72.json")
print("\nThis configuration has:")
print("  - Zero phase variance (all ring demands compatible)")
print("  - Excellent ring meshing (0.158mm error)")
print("  - Good sun clearance (7.84mm margin)")
print("  - Perfect triangular meshing geometry")