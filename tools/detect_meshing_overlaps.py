#!/usr/bin/env python3
"""
Detect overlaps between gears that should be meshing properly.
These overlaps indicate phase calculation errors.
"""

import json
import sys
import math
from typing import Dict, List, Tuple, Set

def parse_solved_json(filepath: str) -> dict:
    """Parse the solved JSON output"""
    with open(filepath, 'r') as f:
        return json.load(f)

def get_gear_params(gear: dict) -> dict:
    """Extract gear parameters from entity"""
    return {
        'center': gear['center'],
        'teeth': gear['teeth'],
        'module': gear['module'],
        'pressure_angle': gear.get('pressure_angle', 20.0),
        'phase': gear.get('phase', 0.0),
        'internal': gear.get('internal', False),
        'id': gear['id']
    }

def pitch_radius(gear: dict) -> float:
    """Calculate pitch radius"""
    return (gear['teeth'] * gear['module']) / 2.0

def outer_radius(gear: dict) -> float:
    """Calculate outer radius (addendum circle)"""
    r = pitch_radius(gear)
    if gear['internal']:
        return r - gear['module']  # Internal gear teeth point inward
    else:
        return r + gear['module']  # External gear teeth point outward

def root_radius(gear: dict) -> float:
    """Calculate root radius (dedendum circle)"""
    r = pitch_radius(gear)
    if gear['internal']:
        return r + 1.25 * gear['module']  # Internal gear root is larger
    else:
        return r - 1.25 * gear['module']  # External gear root is smaller

def center_distance(g1: dict, g2: dict) -> float:
    """Calculate distance between gear centers"""
    dx = g1['center'][0] - g2['center'][0]
    dy = g1['center'][1] - g2['center'][1]
    return math.sqrt(dx*dx + dy*dy)

def theoretical_center_distance(g1: dict, g2: dict) -> float:
    """Calculate theoretical center distance for proper meshing"""
    r1 = pitch_radius(g1)
    r2 = pitch_radius(g2)
    
    if g1['internal'] and not g2['internal']:
        # Internal gear with external gear - subtract radii
        return abs(r1 - r2)
    elif not g1['internal'] and g2['internal']:
        # External gear with internal gear - subtract radii
        return abs(r2 - r1)
    else:
        # Both external or both internal - add radii
        return r1 + r2

def get_tooth_positions(gear: dict, clearance: float = 0.7) -> List[Tuple[float, float]]:
    """Get approximate positions of tooth tips considering phase"""
    tooth_tips = []
    n_teeth = gear['teeth']
    r = outer_radius(gear) - clearance  # Account for 3D printing clearance
    cx, cy = gear['center'][0], gear['center'][1]
    phase_rad = math.radians(gear['phase'])
    
    for i in range(n_teeth):
        angle = (2 * math.pi * i / n_teeth) + phase_rad
        x = cx + r * math.cos(angle)
        y = cy + r * math.sin(angle)
        tooth_tips.append((x, y))
    
    return tooth_tips

def check_tooth_overlap(g1: dict, g2: dict, min_clearance: float = 0.5) -> bool:
    """
    Check if teeth from two meshing gears overlap incorrectly.
    Returns True if there's problematic overlap.
    """
    # Get tooth positions
    teeth1 = get_tooth_positions(g1)
    teeth2 = get_tooth_positions(g2)
    
    # Check minimum distance between any teeth
    min_dist = float('inf')
    for t1 in teeth1:
        for t2 in teeth2:
            dx = t1[0] - t2[0]
            dy = t1[1] - t2[1]
            dist = math.sqrt(dx*dx + dy*dy)
            min_dist = min(min_dist, dist)
    
    # For meshing gears, teeth should interleave, not overlap
    # If minimum distance is less than clearance, we have overlap
    return min_dist < min_clearance

def calculate_expected_phase(g1: dict, g2: dict) -> float:
    """Calculate expected phase for g2 when meshing with g1"""
    # Angle from g1 center to g2 center
    dx = g2['center'][0] - g1['center'][0]
    dy = g2['center'][1] - g1['center'][1]
    angle = math.atan2(dy, dx)
    
    # Phase offset for meshing
    # For external-external: teeth should interleave
    # For internal-external: similar but inverted for internal
    tooth_angle1 = 2 * math.pi / g1['teeth']
    tooth_angle2 = 2 * math.pi / g2['teeth']
    
    # Base phase from angle
    base_phase = math.degrees(angle)
    
    # Add half tooth angle for interleaving
    if g2['internal']:
        # Internal gear needs opposite phase
        phase_offset = math.degrees(tooth_angle2 / 2)
    else:
        phase_offset = math.degrees(tooth_angle2 / 2)
    
    return (base_phase + phase_offset) % 360

def detect_meshing_overlaps(json_file: str) -> List[str]:
    """Detect overlaps between gears that should be meshing"""
    data = parse_solved_json(json_file)
    
    # Extract gears
    gears = {}
    for entity in data.get('entities', []):
        if entity['type'] == 'gear':
            gear = get_gear_params(entity)
            gears[gear['id']] = gear
    
    # Extract meshing constraints
    meshing_pairs = []
    for constraint in data.get('constraints', []):
        if constraint['type'] == 'mesh':
            meshing_pairs.append((constraint['gear1'], constraint['gear2']))
    
    overlaps = []
    phase_errors = []
    
    # Check each meshing pair
    for g1_id, g2_id in meshing_pairs:
        if g1_id not in gears or g2_id not in gears:
            continue
            
        g1 = gears[g1_id]
        g2 = gears[g2_id]
        
        # Check center distance
        actual_dist = center_distance(g1, g2)
        expected_dist = theoretical_center_distance(g1, g2)
        dist_error = abs(actual_dist - expected_dist)
        
        if dist_error > 0.1:  # More than 0.1mm error
            overlaps.append(f"Center distance error between {g1_id} and {g2_id}: {dist_error:.3f}mm")
        
        # Check for tooth overlap (indicates phase error)
        if check_tooth_overlap(g1, g2):
            overlaps.append(f"PHASE ERROR: Tooth overlap between meshing gears {g1_id} and {g2_id}")
            
            # Calculate what phase should be
            expected_phase = calculate_expected_phase(g1, g2)
            actual_phase = g2['phase']
            phase_diff = abs(expected_phase - actual_phase)
            if phase_diff > 180:
                phase_diff = 360 - phase_diff
                
            phase_errors.append(
                f"  {g2_id}: actual phase={actual_phase:.1f}°, "
                f"expected≈{expected_phase:.1f}°, error={phase_diff:.1f}°"
            )
    
    # Combine results
    all_issues = overlaps + phase_errors
    
    return all_issues

def main():
    if len(sys.argv) != 2:
        print("Usage: python detect_meshing_overlaps.py <solved.json>")
        sys.exit(1)
    
    json_file = sys.argv[1]
    overlaps = detect_meshing_overlaps(json_file)
    
    if overlaps:
        print(f"❌ MESHING FAILURE: {len(overlaps)} issues with meshing gears:")
        for overlap in overlaps:
            print(f"  - {overlap}")
        sys.exit(1)  # Failure for CI
    else:
        print("✅ All meshing gears properly aligned (no phase errors)")
        sys.exit(0)  # Success

if __name__ == "__main__":
    main()