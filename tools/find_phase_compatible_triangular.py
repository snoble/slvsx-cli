#!/usr/bin/env python3
"""
Find triangular meshing configurations where all outer planets 
produce compatible phase demands for the ring gear.
"""

import math
import numpy as np

def calculate_phase_propagation(S, I, O, R):
    """
    Calculate phase demands for ring from all outer planets.
    Returns True if all demands are compatible modulo tooth period.
    """
    # Inner planet positions at 60° intervals
    inner_angles = [i * 60 for i in range(6)]
    
    # Calculate inner planet phases from sun
    inner_phases = []
    for angle in inner_angles:
        # Phase propagation: sun to inner
        # φ_inner = (angle * S/I + 180°/I) % 360°
        phase = (angle * S / I + 180 / I) % 360
        inner_phases.append(phase)
    
    # Outer planets mesh with TWO adjacent inners
    # They're positioned between pairs
    outer_angles = [(i * 60 + 30) for i in range(6)]  # Offset by 30°
    
    # Each outer gets two phase demands from its two inner meshes
    outer_phase_demands = []
    for i in range(6):
        inner1_idx = i
        inner2_idx = (i + 1) % 6
        
        # Calculate both phase demands for this outer
        # From inner1
        angle_diff1 = outer_angles[i] - inner_angles[inner1_idx]
        phase1 = (inner_phases[inner1_idx] + angle_diff1 * I / O + 180 / O) % 360
        
        # From inner2  
        angle_diff2 = outer_angles[i] - inner_angles[inner2_idx]
        phase2 = (inner_phases[inner2_idx] + angle_diff2 * I / O + 180 / O) % 360
        
        # Average the two demands (circular mean)
        avg_phase = circular_mean([phase1, phase2])
        outer_phase_demands.append(avg_phase)
    
    # Now calculate ring phase demands from all outers
    ring_demands = []
    for i, outer_phase in enumerate(outer_phase_demands):
        # Ring phase from outer
        # φ_ring = φ_outer - (angle * O/R) + 180°/R
        ring_phase = (outer_phase - outer_angles[i] * O / R + 180 / R) % 360
        ring_demands.append(ring_phase)
    
    # Check if all ring demands are compatible modulo tooth period
    tooth_period = 360 / R
    
    # Normalize all demands to within one tooth period
    normalized = [(d % tooth_period) for d in ring_demands]
    
    # Check variance
    variance = np.std(normalized)
    max_diff = max(normalized) - min(normalized)
    
    # Also check if they align at tooth boundaries or half-tooth
    first_norm = normalized[0]
    all_same = all(abs(n - first_norm) < 0.5 for n in normalized)
    at_boundary = abs(first_norm) < 0.5 or abs(first_norm - tooth_period/2) < 0.5
    
    return variance < 0.1 and all_same, variance, ring_demands

def circular_mean(angles):
    """Calculate circular mean of angles in degrees."""
    angles_rad = [a * math.pi / 180 for a in angles]
    sin_sum = sum(math.sin(a) for a in angles_rad)
    cos_sum = sum(math.cos(a) for a in angles_rad)
    mean_rad = math.atan2(sin_sum, cos_sum)
    mean_deg = mean_rad * 180 / math.pi
    return mean_deg % 360

def check_triangular_geometry(S, I, O, R, module=2.0):
    """Check if triangular meshing geometry works."""
    # Inner planets mesh with sun
    inner_radius = (S + I) * module / 2
    
    # For triangular meshing: outer between two inners
    inner_spacing = 2 * inner_radius * math.sin(math.pi / 6)
    mesh_distance = (I + O) * module / 2
    
    if mesh_distance <= inner_spacing / 2:
        return False, 0, 0, 0, 0
    
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
    
    return geometry_ok, inner_radius, outer_radius_actual, outer_radius_for_ring, min_sun_clearance

print("Searching for phase-compatible triangular meshing configurations...")
print("=" * 80)

valid_configs = []

# Search through tooth combinations
for S in range(12, 49, 6):  # Sun teeth
    for I in range(6, 19, 2):  # Inner teeth  
        for O in range(6, 31, 6):  # Outer teeth
            for R in range(S + 30, S + 90, 6):  # Ring teeth
                
                # Check assembly constraint
                if (S + R) % 6 != 0:
                    continue
                
                # Check geometry
                geom_ok, inner_r, outer_r, ring_r, min_clear = check_triangular_geometry(S, I, O, R)
                if not geom_ok:
                    continue
                
                # Check phase compatibility
                phase_ok, variance, ring_demands = calculate_phase_propagation(S, I, O, R)
                
                if phase_ok:
                    ring_error = abs(outer_r - ring_r)
                    sun_clearance = outer_r - min_clear
                    valid_configs.append({
                        'S': S, 'I': I, 'O': O, 'R': R,
                        'variance': variance,
                        'ring_error': ring_error,
                        'sun_clearance': sun_clearance,
                        'ring_demands': ring_demands
                    })
                    
                    print(f"VALID: S={S}, I={I}, O={O}, R={R}")
                    print(f"  Phase variance: {variance:.4f}")
                    print(f"  Ring error: {ring_error:.3f}mm")
                    print(f"  Sun clearance: {sun_clearance:.2f}mm")
                    print(f"  Ring phase demands: {[f'{d:.1f}°' for d in ring_demands]}")
                    print()

print("=" * 80)
print(f"Found {len(valid_configs)} valid configurations")

if valid_configs:
    print("\nBest configurations (sorted by phase variance):")
    valid_configs.sort(key=lambda x: x['variance'])
    
    for i, cfg in enumerate(valid_configs[:5]):
        print(f"{i+1}. S={cfg['S']}, I={cfg['I']}, O={cfg['O']}, R={cfg['R']}")
        print(f"   Phase variance: {cfg['variance']:.6f}")
        print(f"   Ring error: {cfg['ring_error']:.3f}mm, Sun clearance: {cfg['sun_clearance']:.2f}mm")