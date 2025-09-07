#!/usr/bin/env python3
"""Create S=36, I=9, O=18, R=72 - perfect integer ratios"""

import math
import json

S, I, O, R = 36, 9, 18, 72
module = 2.0

print(f"Creating perfect ratio configuration: S={S}, I={I}, O={O}, R={R}")
print("=" * 60)
print(f"Key ratios:")
print(f"  S/I = {S/I:.0f} (perfect integer)")
print(f"  R/S = {R/S:.0f} (perfect integer)")
print(f"  I:O = 1:2 (simple ratio)")
print()

# Inner planets at 60° intervals
inner_radius = (S + I) * module / 2  # 45mm
inner_positions = []
for i in range(6):
    angle = i * 60 * math.pi / 180
    x = inner_radius * math.cos(angle)
    y = inner_radius * math.sin(angle)
    inner_positions.append((round(x, 2), round(y, 2)))

# Calculate triangular meshing positions for outer planets
inner_spacing = 2 * inner_radius * math.sin(math.pi / 6)  # 45mm
mesh_distance = (I + O) * module / 2  # 27mm

# Height from midpoint line
h = math.sqrt(mesh_distance**2 - (inner_spacing/2)**2)  # ~14.8mm

# Distance to midpoint
mid_radius = inner_radius * math.cos(math.pi / 6)  # ~39mm

# Outer radius
outer_radius = mid_radius + h  # ~54mm

print(f"Geometry:")
print(f"  Inner radius: {inner_radius:.1f}mm")
print(f"  Mesh distance: {mesh_distance:.1f}mm")
print(f"  Outer radius: {outer_radius:.1f}mm")
print(f"  Ring mesh radius: {(R-O)*module/2:.1f}mm")
print(f"  Ring error: {abs(outer_radius - (R-O)*module/2):.3f}mm")
print()

outer_positions = []
for i in range(6):
    inner1 = inner_positions[i]
    inner2 = inner_positions[(i + 1) % 6]
    
    mid_x = (inner1[0] + inner2[0]) / 2
    mid_y = (inner1[1] + inner2[1]) / 2
    
    mid_dist = math.sqrt(mid_x**2 + mid_y**2) if (mid_x != 0 or mid_y != 0) else 0.001
    dir_x = mid_x / mid_dist
    dir_y = mid_y / mid_dist
    
    outer_x = mid_x + dir_x * h
    outer_y = mid_y + dir_y * h
    outer_positions.append((round(outer_x, 2), round(outer_y, 2)))

# Create JSON
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

# Add planets
for i in range(6):
    # Inner planet
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
    config["constraints"].append({
        "type": "mesh",
        "gear1": "sun",
        "gear2": f"inner{i+1}"
    })
    
    # Outer planet
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
    
    # Mesh with two inners
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

with open("testdata/perfect_36_9_18_72.json", "w") as f:
    json.dump(config, f, indent=2)

print("Configuration saved to testdata/perfect_36_9_18_72.json")
print("\nThis configuration features:")
print("- Perfect integer ratios (4:1 sun:inner, 2:1 ring:sun)")
print("- 9-tooth inner planets (odd number can help with phase alignment)")
print("- Excellent ring fit (< 0.1 tooth error)")