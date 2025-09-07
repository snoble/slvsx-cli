#!/usr/bin/env python3
"""
Find triangular meshing configurations where:
1. Inner planets don't touch the ring
2. Outer planets properly mesh with ring
3. All phase demands are compatible
"""

import math
import numpy as np

def calculate_phase_propagation(S, I, O, R):
    """Calculate phase demands for ring from all outer planets."""
    inner_angles = [i * 60 for i in range(6)]
    
    inner_phases = []
    for angle in inner_angles:
        phase = (angle * S / I + 180 / I) % 360
        inner_phases.append(phase)
    
    outer_angles = [(i * 60 + 30) for i in range(6)]
    
    outer_phase_demands = []
    for i in range(6):
        inner1_idx = i
        inner2_idx = (i + 1) % 6
        
        angle_diff1 = outer_angles[i] - inner_angles[inner1_idx]
        phase1 = (inner_phases[inner1_idx] + angle_diff1 * I / O + 180 / O) % 360
        
        angle_diff2 = outer_angles[i] - inner_angles[inner2_idx]
        phase2 = (inner_phases[inner2_idx] + angle_diff2 * I / O + 180 / O) % 360
        
        avg_phase = circular_mean([phase1, phase2])
        outer_phase_demands.append(avg_phase)
    
    ring_demands = []
    for i, outer_phase in enumerate(outer_phase_demands):
        ring_phase = (outer_phase - outer_angles[i] * O / R + 180 / R) % 360
        ring_demands.append(ring_phase)
    
    tooth_period = 360 / R
    normalized = [(d % tooth_period) for d in ring_demands]
    variance = np.std(normalized)
    
    return variance < 0.1, variance, ring_demands

def circular_mean(angles):
    """Calculate circular mean of angles in degrees."""
    angles_rad = [a * math.pi / 180 for a in angles]
    sin_sum = sum(math.sin(a) for a in angles_rad)
    cos_sum = sum(math.cos(a) for a in angles_rad)
    mean_rad = math.atan2(sin_sum, cos_sum)
    mean_deg = mean_rad * 180 / math.pi
    return mean_deg % 360

def check_safe_triangular_geometry(S, I, O, R, module=2.0):
    """
    Check if triangular meshing geometry works AND inner planets clear the ring.
    """
    # Inner planets mesh with sun
    inner_radius = (S + I) * module / 2
    
    # Inner planet outer edge
    inner_outer_edge = inner_radius + I * module / 2 + module  # Pitch radius + addendum
    
    # Ring inner edge 
    ring_inner_edge = R * module / 2 - module  # Ring pitch radius - dedendum
    
    # Check inner-ring clearance (need at least 2mm)
    inner_ring_clearance = ring_inner_edge - inner_outer_edge
    if inner_ring_clearance < 2.0:
        return False, 0, 0, 0, 0, inner_ring_clearance
    
    # For triangular meshing: outer between two inners
    inner_spacing = 2 * inner_radius * math.sin(math.pi / 6)
    mesh_distance = (I + O) * module / 2
    
    if mesh_distance <= inner_spacing / 2:
        return False, 0, 0, 0, 0, 0
    
    # Height of triangle
    h = math.sqrt(mesh_distance**2 - (inner_spacing/2)**2)
    
    # Distance from origin to midpoint between inners
    mid_radius = inner_radius * math.cos(math.pi / 6)
    
    # Outer planet distance from origin
    outer_radius_actual = mid_radius + h
    outer_radius_for_ring = (R - O) * module / 2
    
    # Min clearance from sun
    sun_outer_radius = S * module / 2 + module
    outer_outer_radius = O * module / 2 + module
    min_sun_clearance = sun_outer_radius + outer_outer_radius
    
    ring_error = abs(outer_radius_actual - outer_radius_for_ring)
    geometry_ok = ring_error < 1.0 and outer_radius_actual > min_sun_clearance
    
    return geometry_ok, inner_radius, outer_radius_actual, outer_radius_for_ring, min_sun_clearance, inner_ring_clearance

print("Searching for safe triangular meshing configurations...")
print("(Inner planets must clear the ring)")
print("=" * 80)

valid_configs = []

# Search through tooth combinations - focus on smaller inner planets
for S in range(24, 49, 6):  # Sun teeth - start larger
    for I in range(6, 13, 2):  # Inner teeth - keep small!
        for O in range(12, 25, 6):  # Outer teeth
            for R in range(66, 121, 6):  # Ring teeth - larger rings
                
                # Check assembly constraint
                if (S + R) % 6 != 0:
                    continue
                
                # Check geometry with safety
                result = check_safe_triangular_geometry(S, I, O, R)
                if not result[0]:
                    continue
                    
                geom_ok, inner_r, outer_r, ring_r, min_clear, inner_ring_clear = result
                
                # Check phase compatibility
                phase_ok, variance, ring_demands = calculate_phase_propagation(S, I, O, R)
                
                if phase_ok and inner_ring_clear > 5.0:  # Want good clearance
                    ring_error = abs(outer_r - ring_r)
                    sun_clearance = outer_r - min_clear
                    valid_configs.append({
                        'S': S, 'I': I, 'O': O, 'R': R,
                        'variance': variance,
                        'ring_error': ring_error,
                        'sun_clearance': sun_clearance,
                        'inner_ring_clearance': inner_ring_clear,
                        'inner_radius': inner_r,
                        'outer_radius': outer_r
                    })
                    
                    print(f"VALID: S={S}, I={I}, O={O}, R={R}")
                    print(f"  Inner radius: {inner_r:.1f}mm, Outer radius: {outer_r:.1f}mm")
                    print(f"  Inner-ring clearance: {inner_ring_clear:.1f}mm")
                    print(f"  Phase variance: {variance:.4f}")
                    print(f"  Ring error: {ring_error:.3f}mm")
                    print(f"  Sun clearance: {sun_clearance:.2f}mm")
                    print()

print("=" * 80)
print(f"Found {len(valid_configs)} valid configurations")

if valid_configs:
    print("\nBest configurations (sorted by inner-ring clearance):")
    valid_configs.sort(key=lambda x: -x['inner_ring_clearance'])  # Sort by clearance descending
    
    for i, cfg in enumerate(valid_configs[:5]):
        print(f"{i+1}. S={cfg['S']}, I={cfg['I']}, O={cfg['O']}, R={cfg['R']}")
        print(f"   Inner-ring clearance: {cfg['inner_ring_clearance']:.1f}mm")
        print(f"   Inner radius: {cfg['inner_radius']:.1f}mm, Outer radius: {cfg['outer_radius']:.1f}mm")
        print(f"   Phase variance: {cfg['variance']:.6f}")
        print(f"   Ring error: {cfg['ring_error']:.3f}mm")