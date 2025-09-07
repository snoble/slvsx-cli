#!/usr/bin/env python3
"""
Find tooth counts where triangular meshing keeps outer planets clear of the sun.
For triangular meshing, outer planets must be far enough from sun.
"""

import math

def check_triangular_geometry(S, I, O, R, module=2.0):
    """
    Check if triangular meshing geometry works.
    Returns (valid, inner_radius, outer_radius_actual, outer_radius_needed_for_ring, min_sun_clearance)
    """
    # Inner planets mesh with sun
    inner_radius = (S + I) * module / 2
    
    # For triangular meshing: outer between two inners
    # Distance between adjacent inners at 60° spacing
    inner_spacing = 2 * inner_radius * math.sin(math.pi / 6)  # 60° = π/3, half angle = π/6
    
    # Mesh distance from inner to outer
    mesh_distance = (I + O) * module / 2
    
    # Using triangle geometry: outer is at height h from line between inners
    # We have isosceles triangle with two sides = mesh_distance, base = inner_spacing
    half_base = inner_spacing / 2
    
    # Check if triangle is valid
    if mesh_distance <= half_base:
        return False, 0, 0, 0, 0
    
    # Height of triangle
    h = math.sqrt(mesh_distance**2 - half_base**2)
    
    # Distance from origin to midpoint between inners
    mid_radius = inner_radius * math.cos(math.pi / 6)  # cos(30°)
    
    # Outer planet distance from origin
    outer_radius_actual = mid_radius + h
    
    # Required distance for ring meshing
    outer_radius_for_ring = (R - O) * module / 2
    
    # Minimum clearance from sun (sun outer radius + outer outer radius + safety)
    sun_outer_radius = S * module / 2 + module  # Pitch radius + addendum
    outer_outer_radius = O * module / 2 + module  # Pitch radius + addendum
    min_sun_clearance = sun_outer_radius + outer_outer_radius
    
    # Check all conditions
    ring_error = abs(outer_radius_actual - outer_radius_for_ring)
    valid = (
        ring_error < 2.0 and  # Allow more tolerance for ring meshing
        outer_radius_actual > min_sun_clearance  # Clear of sun
    )
    
    return valid, inner_radius, outer_radius_actual, outer_radius_for_ring, min_sun_clearance

print("Searching for valid triangular meshing configurations...")
print("=" * 80)

valid_configs = []

# Search space - try larger gears for better clearance
for S in range(24, 73, 6):  # Sun teeth
    for I in range(6, 25, 2):  # Inner teeth
        for O in range(6, 37, 2):  # Outer teeth
            # Ring calculation - must be larger to accommodate triangular layout
            # The outer planets are further out in triangular meshing
            for R in range(S + 30, S + 100, 6):
                # Check assembly constraint
                if (S + R) % 6 != 0:
                    continue
                
                valid, inner_r, outer_r_actual, outer_r_ring, min_sun_clear = check_triangular_geometry(S, I, O, R)
                
                if valid:
                    ring_error = abs(outer_r_actual - outer_r_ring)
                    sun_clearance = outer_r_actual - min_sun_clear
                    valid_configs.append((S, I, O, R, ring_error, sun_clearance, inner_r, outer_r_actual))
                    print(f"VALID: S={S}, I={I}, O={O}, R={R}")
                    print(f"  Inner radius: {inner_r:.2f}mm")
                    print(f"  Outer radius: {outer_r_actual:.2f}mm (ring needs {outer_r_ring:.2f}mm)")
                    print(f"  Sun clearance: {sun_clearance:.2f}mm margin")
                    print(f"  Ring error: {ring_error:.3f}mm")
                    print()

print("\n" + "=" * 80)
print(f"Found {len(valid_configs)} valid configurations")

if valid_configs:
    print("\nBest configurations (sorted by ring error):")
    valid_configs.sort(key=lambda x: x[4])  # Sort by ring error
    
    for i, (S, I, O, R, ring_err, sun_clear, inner_r, outer_r) in enumerate(valid_configs[:10]):
        print(f"{i+1}. S={S}, I={I}, O={O}, R={R}")
        print(f"   Ring error: {ring_err:.3f}mm, Sun clearance margin: {sun_clear:.2f}mm")
        print(f"   Radii: inner={inner_r:.1f}mm, outer={outer_r:.1f}mm")