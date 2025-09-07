#!/usr/bin/env python3
"""
Calculate proper positions for outer planets in triangular meshing.
Each outer planet meshes with TWO adjacent inner planets.
"""

import math
import json

def calculate_positions(S, I, O, R, module=2.0):
    """Calculate positions for triangular meshing double planetary."""
    
    # Inner planets mesh with sun at 60° intervals
    inner_radius = (S + I) * module / 2
    inner_positions = []
    for i in range(6):
        angle = i * 60 * math.pi / 180
        x = inner_radius * math.cos(angle)
        y = inner_radius * math.sin(angle)
        inner_positions.append((x, y))
        print(f"Inner {i+1}: ({x:.2f}, {y:.2f})")
    
    # Outer planets positioned between pairs of inner planets
    # Each outer meshes with TWO adjacent inners
    mesh_distance = (I + O) * module / 2
    
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
        
        # Use law of cosines to find the height
        # We have an isosceles triangle with two sides = mesh_distance
        # and base = distance between inners
        height = math.sqrt(mesh_distance**2 - dist_to_mid**2)
        
        # Direction perpendicular to the line between inners (outward)
        # Vector from origin to midpoint, normalized
        mid_dist = math.sqrt(mid_x**2 + mid_y**2)
        if mid_dist > 0:
            dir_x = mid_x / mid_dist
            dir_y = mid_y / mid_dist
        else:
            # Special case if midpoint is at origin
            angle = (i * 60 + 30) * math.pi / 180
            dir_x = math.cos(angle)
            dir_y = math.sin(angle)
        
        # Position outer planet
        outer_x = mid_x + dir_x * height
        outer_y = mid_y + dir_y * height
        outer_positions.append((outer_x, outer_y))
        
        # Verify distances
        dist1 = math.sqrt((outer_x - inner1[0])**2 + (outer_y - inner1[1])**2)
        dist2 = math.sqrt((outer_x - inner2[0])**2 + (outer_y - inner2[1])**2)
        print(f"Outer {i+1}: ({outer_x:.2f}, {outer_y:.2f}) - distances: {dist1:.2f}, {dist2:.2f} (should be {mesh_distance:.2f})")
        
        # Check distance from origin (for ring meshing)
        outer_orbit = math.sqrt(outer_x**2 + outer_y**2)
        ring_mesh_radius = (R - O) * module / 2
        print(f"  Distance from origin: {outer_orbit:.2f} (ring expects: {ring_mesh_radius:.2f})")
        
        # Check clearance from sun
        sun_outer_dist = math.sqrt(outer_x**2 + outer_y**2)
        sun_radius = S * module / 2
        outer_radius = O * module / 2
        min_clearance = sun_radius + outer_radius + module  # Add some clearance
        print(f"  Clearance from sun: {sun_outer_dist:.2f} (min needed: {min_clearance:.2f})")
    
    return inner_positions, outer_positions

def check_inner_ring_clearance(I, R, module, inner_positions):
    """Check that inner planets don't touch the ring."""
    inner_radius = I * module / 2
    ring_inner_radius = R * module / 2 - module  # Ring's inner edge
    
    for i, (x, y) in enumerate(inner_positions):
        dist_from_origin = math.sqrt(x**2 + y**2)
        max_reach = dist_from_origin + inner_radius
        print(f"Inner {i+1} max reach: {max_reach:.2f}, ring inner edge: {ring_inner_radius:.2f}")
        if max_reach >= ring_inner_radius:
            print(f"  WARNING: Inner {i+1} might touch ring!")

# Test with 24-12-12-72 configuration
print("Testing 24-12-12-72 configuration:")
print("="*50)
S, I, O, R = 24, 12, 12, 72
inner_pos, outer_pos = calculate_positions(S, I, O, R)
print("\nChecking inner-ring clearance:")
check_inner_ring_clearance(I, R, 2.0, inner_pos)

print("\n" + "="*50)
print("Testing other configurations:")
print("="*50)

# Try other tooth combinations
configs = [
    (36, 6, 18, 66),  # From the corrected document
    (24, 6, 12, 48),  # Another from corrected document
    (30, 10, 10, 60), # Symmetric
    (24, 8, 16, 64),  # Powers of 2
]

for S, I, O, R in configs:
    print(f"\n{S}-{I}-{O}-{R}:")
    try:
        inner_pos, outer_pos = calculate_positions(S, I, O, R)
        check_inner_ring_clearance(I, R, 2.0, inner_pos)
    except Exception as e:
        print(f"  ERROR: {e}")