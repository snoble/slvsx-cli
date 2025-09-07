#!/usr/bin/env python3
"""
Find tooth count combinations that allow valid double planetary geometry.
The key constraint: outer planets must be able to simultaneously:
1. Mesh with their inner planet (distance = (I+O)*m/2)
2. Mesh with the ring (orbit = R*m/2 - O*m/2)
"""

import math

def check_geometry(S, I, O, R, num_planets=3):
    """Check if the given tooth counts allow valid geometry."""
    module = 2.0
    
    # Basic constraints
    if (S + R) % num_planets != 0:
        return False, "Assembly constraint failed"
    
    # Calculate radii
    sun_radius = S * module / 2
    inner_radius = I * module / 2
    outer_radius = O * module / 2
    ring_radius = R * module / 2
    
    # Inner planet orbit (meshing with sun)
    inner_orbit = sun_radius + inner_radius
    
    # Outer planet constraints
    mesh_distance = inner_radius + outer_radius  # Distance to inner planet
    ring_orbit = ring_radius - outer_radius      # Orbit for ring mesh
    
    # Can we position outer planet to satisfy both constraints?
    # Using triangle with sides: inner_orbit, mesh_distance, ring_orbit
    # Check if triangle inequality is satisfied
    if ring_orbit > inner_orbit + mesh_distance:
        return False, f"Ring orbit too large: {ring_orbit:.1f} > {inner_orbit:.1f} + {mesh_distance:.1f}"
    if ring_orbit < abs(inner_orbit - mesh_distance):
        return False, f"Ring orbit too small: {ring_orbit:.1f} < |{inner_orbit:.1f} - {mesh_distance:.1f}|"
    
    # Use law of cosines to find angle
    # cos(angle) = (a² + b² - c²) / (2ab)
    cos_angle = (inner_orbit**2 + mesh_distance**2 - ring_orbit**2) / (2 * inner_orbit * mesh_distance)
    
    if abs(cos_angle) > 1:
        return False, f"Invalid geometry: cos(angle) = {cos_angle:.3f}"
    
    angle = math.acos(cos_angle) * 180 / math.pi
    
    # Check if the angle is reasonable (not too tight)
    if angle < 30 or angle > 150:
        return False, f"Angle too extreme: {angle:.1f}°"
    
    return True, f"Valid! Angle: {angle:.1f}°"

print("Searching for valid tooth count combinations...")
print("=" * 60)

# Search ranges
sun_range = range(20, 31, 2)  # Even numbers for sun
inner_range = range(10, 21)   # Inner planet teeth
outer_range = range(15, 31)   # Outer planet teeth

valid_configs = []

for S in sun_range:
    for I in inner_range:
        for O in outer_range:
            # Ring teeth must satisfy some constraints
            # Try different ring sizes
            for R in range(60, 101, 2):  # Even numbers for ring
                valid, message = check_geometry(S, I, O, R, 3)
                if valid:
                    valid_configs.append((S, I, O, R, message))
                    print(f"✓ S={S}, I={I}, O={O}, R={R} - {message}")

print(f"\nFound {len(valid_configs)} valid configurations")

if valid_configs:
    print("\nBest configurations (sorted by angle closeness to 60°):")
    
    # Sort by how close the angle is to 60° (ideal for 3 planets)
    def angle_score(config):
        message = config[4]
        angle = float(message.split("Angle: ")[1].split("°")[0])
        return abs(angle - 60)
    
    valid_configs.sort(key=angle_score)
    
    for i, (S, I, O, R, message) in enumerate(valid_configs[:10]):
        module = 2.0
        inner_orbit = (S + I) * module / 2
        outer_orbit = (R - O) * module / 2
        mesh_dist = (I + O) * module / 2
        
        print(f"\n{i+1}. Sun={S}, Inner={I}, Outer={O}, Ring={R}")
        print(f"   {message}")
        print(f"   Inner orbit: {inner_orbit:.1f}mm")
        print(f"   Outer orbit: {outer_orbit:.1f}mm")
        print(f"   Inner-Outer distance: {mesh_dist:.1f}mm")
        
        # Save the best one
        if i == 0:
            best = (S, I, O, R)