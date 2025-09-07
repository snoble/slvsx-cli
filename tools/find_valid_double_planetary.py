#!/usr/bin/env python3
"""
Find valid double planetary gear configuration where all outer planets
agree on the ring gear's phase.
"""

import math
import json

# System parameters
S = 24  # Sun teeth
I = 12  # Inner planet teeth
O = 18  # Outer planet teeth
R = 72  # Ring teeth
module = 2.0

# Derived parameters
sun_radius = S * module / 2
inner_radius = I * module / 2
outer_radius = O * module / 2
ring_radius = R * module / 2

# Inner planet orbit and positions (equally spaced)
inner_orbit = sun_radius + inner_radius  # 36mm
inner_angles = [0, 120, 240]  # degrees

def deg_to_rad(deg):
    return deg * math.pi / 180

def rad_to_deg(rad):
    return rad * 180 / math.pi

def calculate_position(radius, angle_deg):
    angle_rad = deg_to_rad(angle_deg)
    return [radius * math.cos(angle_rad), radius * math.sin(angle_rad)]

def calculate_gear_phase(gear1_teeth, gear2_teeth, angle_deg, is_internal=False):
    """Calculate phase for gear2 meshing with gear1."""
    ratio = gear1_teeth / gear2_teeth
    angle_rad = deg_to_rad(angle_deg)
    
    # Half tooth offset for proper meshing
    tooth_angle = 360 / gear2_teeth
    
    if is_internal:
        # For external-internal meshing
        phase = -angle_deg * ratio + tooth_angle / 2
    else:
        # For external-external meshing
        phase = angle_deg * ratio + tooth_angle / 2
    
    return phase % 360

def find_outer_planet_position(inner_pos, inner_angle_deg):
    """Find where outer planet should be to mesh with inner planet."""
    mesh_distance = inner_radius + outer_radius  # 30mm
    
    # Try different angles around the inner planet
    best_config = None
    
    for offset_angle in range(0, 360, 10):
        # Position outer planet at offset angle from inner
        outer_angle = inner_angle_deg + offset_angle
        outer_orbit = math.sqrt(inner_orbit**2 + mesh_distance**2 - 
                                2*inner_orbit*mesh_distance*math.cos(deg_to_rad(offset_angle)))
        
        # Calculate angle from origin to outer planet
        ix, iy = calculate_position(inner_orbit, inner_angle_deg)
        ox_rel = mesh_distance * math.cos(deg_to_rad(outer_angle))
        oy_rel = mesh_distance * math.sin(deg_to_rad(outer_angle))
        ox = ix + ox_rel
        oy = iy + oy_rel
        
        outer_angle_from_origin = rad_to_deg(math.atan2(oy, ox))
        outer_orbit_actual = math.sqrt(ox**2 + oy**2)
        
        # Check if this outer planet can mesh with ring
        ring_mesh_distance = ring_radius - outer_radius  # 54mm
        if abs(outer_orbit_actual - ring_mesh_distance) < 0.1:
            return {
                'position': [ox, oy],
                'angle_from_origin': outer_angle_from_origin,
                'orbit': outer_orbit_actual
            }
    
    return None

print("Double Planetary Configuration Solver")
print("=" * 50)
print(f"Sun: {S} teeth")
print(f"Inner planets: {I} teeth")
print(f"Outer planets: {O} teeth")
print(f"Ring: {R} teeth")
print(f"Module: {module}mm")
print()

# Calculate inner planet phases (meshing with sun)
inner_phases = []
for i, angle in enumerate(inner_angles):
    phase = calculate_gear_phase(S, I, angle)
    inner_phases.append(phase)
    print(f"Inner planet {i+1}: angle={angle}°, phase={phase:.1f}°")

print()

# Find valid outer planet positions
outer_configs = []
for i, inner_angle in enumerate(inner_angles):
    # For double planetary, outer planets need to be positioned to:
    # 1. Mesh with their inner planet
    # 2. Mesh with the ring
    
    # The key insight: outer planets should be on a circle of radius = ring_radius - outer_radius
    outer_orbit_for_ring = ring_radius - outer_radius  # 54mm
    
    # But they also need to be at distance (I + O)*m/2 from their inner planet
    mesh_distance = (I + O) * module / 2  # 30mm
    
    # Find the angle that satisfies both constraints
    # Using law of cosines: c² = a² + b² - 2ab*cos(C)
    # outer_orbit² = inner_orbit² + mesh_distance² - 2*inner_orbit*mesh_distance*cos(offset)
    
    # Rearranging: cos(offset) = (inner_orbit² + mesh_distance² - outer_orbit²) / (2*inner_orbit*mesh_distance)
    cos_offset = (inner_orbit**2 + mesh_distance**2 - outer_orbit_for_ring**2) / (2 * inner_orbit * mesh_distance)
    
    print(f"\nFinding position for outer planet {i+1}:")
    print(f"  Inner at angle {inner_angle}°, orbit {inner_orbit:.1f}mm")
    print(f"  Need mesh distance {mesh_distance:.1f}mm to inner")
    print(f"  Need orbit {outer_orbit_for_ring:.1f}mm for ring mesh")
    print(f"  cos(offset) = {cos_offset:.3f}")
    
    if abs(cos_offset) <= 1:
        offset_angle = rad_to_deg(math.acos(cos_offset))
        print(f"  Offset angle: ±{offset_angle:.1f}°")
        
        # There are two solutions (+ and -)
        for sign in [1, -1]:
            actual_offset = sign * offset_angle
            outer_angle = inner_angle + actual_offset
            
            ox = outer_orbit_for_ring * math.cos(deg_to_rad(outer_angle))
            oy = outer_orbit_for_ring * math.sin(deg_to_rad(outer_angle))
            
            # Verify distance to inner planet
            ix, iy = calculate_position(inner_orbit, inner_angle)
            dist_to_inner = math.sqrt((ox-ix)**2 + (oy-iy)**2)
            
            print(f"  Trying offset {actual_offset:.1f}°: distance to inner = {dist_to_inner:.2f}mm")
            
            if abs(dist_to_inner - mesh_distance) < 0.5:  # Increased tolerance
                outer_configs.append({
                    'index': i,
                    'position': [ox, oy],
                    'angle': outer_angle % 360,
                    'orbit': outer_orbit_for_ring,
                    'offset_from_inner': actual_offset
                })
                print(f"  ✓ Found: Outer planet {i+1} at angle={outer_angle:.1f}°")
                break
    else:
        print(f"  ✗ No solution: cos_offset = {cos_offset:.3f} is out of range [-1, 1]")
        print(f"  This means the geometry is impossible with these parameters")

print()

# Calculate outer planet phases
outer_phases = []
for i, config in enumerate(outer_configs):
    inner_phase = inner_phases[config['index']]
    
    # Phase propagation from inner to outer
    relative_angle = config['offset_from_inner']
    outer_phase = calculate_gear_phase(I, O, relative_angle)
    outer_phase = (inner_phase + outer_phase) % 360
    outer_phases.append(outer_phase)
    print(f"Outer planet {i+1}: phase={outer_phase:.1f}°")

print()

# Check ring phase agreement
ring_phase_demands = []
for i, config in enumerate(outer_configs):
    # Each outer planet demands a specific ring phase
    outer_phase = outer_phases[i]
    outer_angle = config['angle']
    
    # For internal-external meshing
    ring_phase_demand = calculate_gear_phase(O, R, outer_angle, is_internal=True)
    ring_phase_demand = (outer_phase - ring_phase_demand) % 360
    ring_phase_demands.append(ring_phase_demand)
    print(f"Ring phase demanded by outer {i+1}: {ring_phase_demand:.1f}°")

# Check if all demands are close
phase_variance = max(ring_phase_demands) - min(ring_phase_demands)
print()
print(f"Phase variance: {phase_variance:.1f}°")

if phase_variance < 5:  # Allow 5 degrees tolerance
    print("✓ Configuration is valid! All outer planets agree on ring phase.")
    
    # Create JSON configuration
    avg_ring_phase = sum(ring_phase_demands) / len(ring_phase_demands)
    
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
                "phase": avg_ring_phase,
                "internal": True
            }
        ],
        "constraints": []
    }
    
    # Add inner planets
    for i, angle in enumerate(inner_angles):
        pos = calculate_position(inner_orbit, angle)
        config["entities"].append({
            "type": "gear",
            "id": f"inner{i+1}",
            "center": [round(pos[0], 2), round(pos[1], 2), 0],
            "teeth": "$inner_teeth",
            "module": "$module",
            "pressure_angle": "$pressure_angle",
            "phase": inner_phases[i],
            "internal": False
        })
        config["constraints"].append({
            "type": "mesh",
            "gear1": "sun",
            "gear2": f"inner{i+1}"
        })
    
    # Add outer planets
    for i, outer_config in enumerate(outer_configs):
        config["entities"].append({
            "type": "gear",
            "id": f"outer{i+1}",
            "center": [round(outer_config['position'][0], 2), round(outer_config['position'][1], 2), 0],
            "teeth": "$outer_teeth",
            "module": "$module",
            "pressure_angle": "$pressure_angle",
            "phase": outer_phases[i],
            "internal": False
        })
        config["constraints"].append({
            "type": "mesh",
            "gear1": f"inner{outer_config['index']+1}",
            "gear2": f"outer{i+1}"
        })
        config["constraints"].append({
            "type": "mesh",
            "gear1": "ring",
            "gear2": f"outer{i+1}"
        })
    
    # Save configuration
    with open('valid_double_planetary.json', 'w') as f:
        json.dump(config, f, indent=2)
    
    print("\nConfiguration saved to valid_double_planetary.json")
else:
    print("✗ Configuration is invalid. Outer planets disagree on ring phase.")
    print("Need to adjust tooth counts or use different strategy.")