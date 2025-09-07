#!/usr/bin/env python3
"""
Validate planetary gear system constraints mathematically.
This checks that all the geometric and assembly constraints are satisfied.
"""

import math
import json
import sys

def validate_planetary_gear_system(config_file):
    """Validate a planetary gear configuration."""
    
    with open(config_file, 'r') as f:
        config = json.load(f)
    
    params = config['parameters']
    
    # Extract parameters
    module = params['module']
    sun_teeth = params['sun_teeth']
    inner_teeth = params['inner_teeth']
    outer_teeth = params['outer_teeth']
    ring_teeth = params['ring_teeth']
    num_planets = params['num_planets']
    
    print(f"Validating planetary gear system:")
    print(f"  Sun: {sun_teeth} teeth")
    print(f"  Inner planets: {inner_teeth} teeth × {num_planets}")
    print(f"  Outer planets: {outer_teeth} teeth × {num_planets}")
    print(f"  Ring: {ring_teeth} teeth")
    print(f"  Module: {module}mm")
    print()
    
    # Calculate pitch radii
    sun_r = (sun_teeth * module) / 2
    inner_r = (inner_teeth * module) / 2
    outer_r = (outer_teeth * module) / 2
    ring_r = (ring_teeth * module) / 2
    
    print(f"Pitch radii:")
    print(f"  Sun: {sun_r}mm")
    print(f"  Inner: {inner_r}mm")
    print(f"  Outer: {outer_r}mm")
    print(f"  Ring: {ring_r}mm")
    print()
    
    # Check assembly constraint
    assembly_value = (sun_teeth + ring_teeth) / num_planets
    is_integer = assembly_value == int(assembly_value)
    print(f"Assembly constraint: (S + R) / n = ({sun_teeth} + {ring_teeth}) / {num_planets} = {assembly_value}")
    if is_integer:
        print("  ✓ Assembly constraint satisfied (integer)")
    else:
        print("  ✗ Assembly constraint NOT satisfied (must be integer)")
    print()
    
    # Check orbit radii
    inner_orbit = sun_r + inner_r
    outer_orbit = ring_r - outer_r
    
    print(f"Required orbit radii:")
    print(f"  Inner orbit: {inner_orbit}mm (sun + inner)")
    print(f"  Outer orbit: {outer_orbit}mm (ring - outer)")
    print()
    
    # Check if inner and outer can mesh
    inner_outer_distance = math.sqrt((outer_orbit - inner_orbit)**2)
    required_mesh_distance = inner_r + outer_r
    
    print(f"Inner-outer meshing:")
    print(f"  Radial separation: {abs(outer_orbit - inner_orbit):.2f}mm")
    print(f"  Required for mesh: {required_mesh_distance}mm")
    
    # For planets to mesh, they need to be at different radii but close enough
    # Check specific positions
    print()
    print(f"Checking specific mesh points:")
    
    # Inner at 0°, outer at 30°
    inner_x = inner_orbit
    inner_y = 0
    outer_x = outer_orbit * math.cos(math.radians(30))
    outer_y = outer_orbit * math.sin(math.radians(30))
    
    distance = math.sqrt((outer_x - inner_x)**2 + (outer_y - inner_y)**2)
    print(f"  Inner at 0° to Outer at 30°: {distance:.2f}mm (need {required_mesh_distance}mm)")
    if abs(distance - required_mesh_distance) < 0.1:
        print("    ✓ Can mesh")
    else:
        print(f"    ✗ Cannot mesh (off by {abs(distance - required_mesh_distance):.2f}mm)")
    
    # Check neighbor distances
    print()
    print(f"Neighbor distances (equal spacing):")
    angle_between = 360 / num_planets
    
    # Inner neighbors
    inner_neighbor_dist = 2 * inner_orbit * math.sin(math.radians(angle_between/2))
    min_safe_inner = 2 * (inner_r + module)  # With addendum
    print(f"  Inner neighbors: {inner_neighbor_dist:.2f}mm")
    print(f"    Min safe distance: {min_safe_inner:.2f}mm")
    if inner_neighbor_dist > min_safe_inner:
        print(f"    ✓ Clearance: {inner_neighbor_dist - min_safe_inner:.2f}mm")
    else:
        print(f"    ✗ Collision: {min_safe_inner - inner_neighbor_dist:.2f}mm overlap")
    
    # Outer neighbors  
    outer_neighbor_dist = 2 * outer_orbit * math.sin(math.radians(angle_between/2))
    min_safe_outer = 2 * (outer_r + module)  # With addendum
    print(f"  Outer neighbors: {outer_neighbor_dist:.2f}mm")
    print(f"    Min safe distance: {min_safe_outer:.2f}mm")
    if outer_neighbor_dist > min_safe_outer:
        print(f"    ✓ Clearance: {outer_neighbor_dist - min_safe_outer:.2f}mm")
    else:
        print(f"    ✗ Collision: {min_safe_outer - outer_neighbor_dist:.2f}mm overlap")
    
    print()
    print("Validation complete!")
    
    return is_integer and inner_neighbor_dist > min_safe_inner and outer_neighbor_dist > min_safe_outer

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python validate_gear_system.py <config.json>")
        sys.exit(1)
    
    config_file = sys.argv[1]
    valid = validate_planetary_gear_system(config_file)
    sys.exit(0 if valid else 1)