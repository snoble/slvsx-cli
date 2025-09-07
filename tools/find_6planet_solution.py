#!/usr/bin/env python3
"""
Find valid double planetary configuration with 6 inner and 6 outer planets.
Key insight: With 6 planets, we can pair each inner with an outer at a consistent offset.
"""

import math
import json

def analyze_6planet_system(S, I, O, R, module=2.0):
    """Analyze a 6-planet double planetary system."""
    
    # Check assembly constraint for 6 planets
    if (S + R) % 6 != 0:
        return None, "Assembly constraint failed for 6 planets"
    
    # Calculate radii
    sun_radius = S * module / 2
    inner_radius = I * module / 2
    outer_radius = O * module / 2
    ring_radius = R * module / 2
    
    # Orbits
    inner_orbit = sun_radius + inner_radius
    outer_orbit_for_ring = ring_radius - outer_radius
    mesh_distance = inner_radius + outer_radius
    
    # Check triangle inequality
    if outer_orbit_for_ring > inner_orbit + mesh_distance:
        return None, f"Triangle impossible: {outer_orbit_for_ring:.1f} > {inner_orbit:.1f} + {mesh_distance:.1f}"
    if outer_orbit_for_ring < abs(inner_orbit - mesh_distance):
        return None, f"Triangle impossible: {outer_orbit_for_ring:.1f} < |{inner_orbit:.1f} - {mesh_distance:.1f}|"
    
    # Calculate offset angle using law of cosines
    cos_offset = (inner_orbit**2 + mesh_distance**2 - outer_orbit_for_ring**2) / (2 * inner_orbit * mesh_distance)
    
    if abs(cos_offset) > 1:
        return None, f"Invalid geometry: cos(offset) = {cos_offset:.3f}"
    
    offset_angle = math.acos(cos_offset) * 180 / math.pi
    
    # With 6 planets, inner planets are at 60° intervals
    # If outer planets are all at the same offset, they maintain symmetry
    
    result = {
        'S': S, 'I': I, 'O': O, 'R': R,
        'inner_orbit': inner_orbit,
        'outer_orbit': outer_orbit_for_ring,
        'mesh_distance': mesh_distance,
        'offset_angle': offset_angle,
        'symmetry': '6-fold'
    }
    
    return result, f"Valid! Offset angle: {offset_angle:.1f}°"

print("6-Planet Double Planetary System Analysis")
print("=" * 60)
print()

# Original configuration with 6 planets
S, I, O, R = 24, 12, 18, 72
result, message = analyze_6planet_system(S, I, O, R)

if result:
    print(f"Configuration: S={S}, I={I}, O={O}, R={R}")
    print(message)
    print(f"Inner orbit: {result['inner_orbit']:.1f}mm")
    print(f"Outer orbit: {result['outer_orbit']:.1f}mm")
    print(f"Mesh distance: {result['mesh_distance']:.1f}mm")
    print(f"Offset angle: {result['offset_angle']:.1f}°")
    print()
    
    # Key insight: With 6 planets at 60° spacing and offset of ~109°,
    # we get a different pattern than with 3 planets
    
    print("Planet positions:")
    print("-" * 40)
    
    inner_angles = [i * 60 for i in range(6)]
    config = {
        "schema": "slvs-json/1",
        "units": "mm",
        "parameters": {
            "module": 2.0,
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
    for i, angle in enumerate(inner_angles):
        x = result['inner_orbit'] * math.cos(math.radians(angle))
        y = result['inner_orbit'] * math.sin(math.radians(angle))
        
        print(f"Inner {i+1}: angle={angle:3.0f}°, pos=({x:6.2f}, {y:6.2f})")
        
        config["entities"].append({
            "type": "gear",
            "id": f"inner{i+1}",
            "center": [round(x, 2), round(y, 2), 0],
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
    
    print()
    
    # Add outer planets - each offset from its inner planet
    # Use consistent offset direction for symmetry
    offset = result['offset_angle']
    
    for i, inner_angle in enumerate(inner_angles):
        # Try positive offset for all
        outer_angle = inner_angle + offset
        x = result['outer_orbit'] * math.cos(math.radians(outer_angle))
        y = result['outer_orbit'] * math.sin(math.radians(outer_angle))
        
        # Verify distance to inner planet
        ix = result['inner_orbit'] * math.cos(math.radians(inner_angle))
        iy = result['inner_orbit'] * math.sin(math.radians(inner_angle))
        dist = math.sqrt((x-ix)**2 + (y-iy)**2)
        
        print(f"Outer {i+1}: angle={outer_angle:6.1f}°, pos=({x:6.2f}, {y:6.2f}), dist to inner={dist:.2f}mm")
        
        config["entities"].append({
            "type": "gear",
            "id": f"outer{i+1}",
            "center": [round(x, 2), round(y, 2), 0],
            "teeth": "$outer_teeth",
            "module": "$module",
            "pressure_angle": "$pressure_angle",
            "phase": 0.0,
            "internal": False
        })
        
        config["constraints"].append({
            "type": "mesh",
            "gear1": f"inner{i+1}",
            "gear2": f"outer{i+1}"
        })
        
        config["constraints"].append({
            "type": "mesh",
            "gear1": "ring",
            "gear2": f"outer{i+1}"
        })
    
    # Save configuration
    with open('testdata/double_planetary_6planets.json', 'w') as f:
        json.dump(config, f, indent=2)
    
    print("\nConfiguration saved to testdata/double_planetary_6planets.json")
    
    print("\nKey advantages of 6-planet configuration:")
    print("1. Better load distribution")
    print("2. More symmetric phase relationships")
    print("3. Each outer planet has consistent offset from its inner partner")
    print("4. Ring sees 6 phase votes instead of 3 - may average out better")
    
else:
    print(f"Configuration failed: {message}")

print("\n" + "=" * 60)
print("Searching for optimal 6-planet configurations...")
print()

# Search for better tooth counts
best_configs = []

for S in range(24, 37, 6):  # Multiples of 6 for symmetry
    for I in range(12, 25):
        for O in range(18, 31):
            for R in range(72, 97, 6):  # Multiples of 6
                result, message = analyze_6planet_system(S, I, O, R)
                if result:
                    # Prefer offset angles close to 60° or 120° for better symmetry
                    angle = result['offset_angle']
                    score = min(abs(angle - 60), abs(angle - 120), abs(angle - 180))
                    best_configs.append((score, S, I, O, R, angle))

best_configs.sort()

if best_configs:
    print("Top configurations (sorted by symmetry):")
    for i, (score, S, I, O, R, angle) in enumerate(best_configs[:5]):
        print(f"{i+1}. S={S}, I={I}, O={O}, R={R} - offset={angle:.1f}°")