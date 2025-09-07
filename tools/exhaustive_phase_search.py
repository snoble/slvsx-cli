#!/usr/bin/env python3
"""
Exhaustive search for tooth count combinations where the phase demands
align modulo the tooth period, making a valid double planetary system.
"""

import math
import json
from itertools import product

def calculate_gear_phase(teeth1, teeth2, angle_deg):
    """Calculate phase for gear2 meshing with gear1."""
    ratio = teeth1 / teeth2
    tooth_angle = 360 / teeth2
    phase = angle_deg * ratio + tooth_angle / 2
    return phase % 360

def simulate_phase_propagation(S, I, O, R, num_planets=6):
    """Simulate phase propagation through the system."""
    
    # Module doesn't affect phase calculations
    module = 2.0
    
    # Check basic assembly constraints
    if (S + R) % num_planets != 0:
        return None, "Assembly constraint failed"
    
    # Calculate orbits
    sun_radius = S * module / 2
    inner_radius = I * module / 2
    outer_radius = O * module / 2
    ring_radius = R * module / 2
    
    inner_orbit = sun_radius + inner_radius
    outer_orbit_for_ring = ring_radius - outer_radius
    mesh_distance = inner_radius + outer_radius
    
    # Check geometric feasibility using law of cosines
    cos_offset = (inner_orbit**2 + mesh_distance**2 - outer_orbit_for_ring**2) / (2 * inner_orbit * mesh_distance)
    
    if abs(cos_offset) > 1:
        return None, f"Geometry impossible"
    
    offset_angle = math.acos(cos_offset) * 180 / math.pi
    
    # Calculate phases for all planets
    inner_angles = [i * 360/num_planets for i in range(num_planets)]
    
    phases = {
        'sun': 0.0,
        'inner': [],
        'outer': [],
        'ring_demands': []
    }
    
    for i, inner_angle in enumerate(inner_angles):
        # Inner planet phase from sun
        inner_phase = calculate_gear_phase(S, I, inner_angle)
        phases['inner'].append(inner_phase)
        
        # Outer planet position and phase
        outer_angle = inner_angle + offset_angle  # Could also be minus
        outer_phase_from_inner = calculate_gear_phase(I, O, offset_angle)
        outer_phase = (inner_phase + outer_phase_from_inner) % 360
        phases['outer'].append(outer_phase)
        
        # Ring phase demand from this outer planet
        ring_demand = (outer_phase - calculate_gear_phase(O, R, outer_angle)) % 360
        phases['ring_demands'].append(ring_demand)
    
    return phases, offset_angle

def check_phase_alignment(phases, R, tolerance=0.1):
    """Check if ring phase demands align modulo tooth period."""
    
    if not phases:
        return False, 0, []
    
    tooth_period = 360 / R
    ring_demands = phases['ring_demands']
    
    # Normalize to tooth period
    normalized = [(p % 360) % tooth_period for p in ring_demands]
    
    # Check variance
    variance = max(normalized) - min(normalized)
    
    # Also check if they're all at half-tooth positions
    half_tooth = tooth_period / 2
    half_tooth_aligned = all(abs(n - half_tooth) < tolerance or abs(n) < tolerance for n in normalized)
    
    return variance < tolerance or half_tooth_aligned, variance, normalized

print("Exhaustive Search for Valid Double Planetary Configurations")
print("=" * 70)
print()

# Search parameters
sun_range = range(12, 49, 2)  # Even numbers
inner_range = range(6, 25)    # All sizes
outer_range = range(6, 31)    # All sizes
num_planets_options = [3, 6]  # Try both

valid_configs = []
tested = 0
max_tests = 50000

print(f"Searching up to {max_tests} configurations...")
print()

for num_planets in num_planets_options:
    for S, I, O in product(sun_range, inner_range, outer_range):
        # Try different ring sizes
        for R_multiplier in range(2, 8):
            R = S + R_multiplier * I  # Heuristic for reasonable ring size
            
            if R > 120:  # Skip very large rings
                continue
            
            tested += 1
            if tested > max_tests:
                break
            
            # Simulate the system
            phases, geometry_result = simulate_phase_propagation(S, I, O, R, num_planets)
            
            if phases:
                # Check phase alignment
                aligned, variance, normalized = check_phase_alignment(phases, R)
                
                if aligned:
                    # Calculate some useful metrics
                    inner_orbit = (S + I)
                    outer_orbit = (R - O)
                    ratio = outer_orbit / inner_orbit
                    
                    config = {
                        'S': S, 'I': I, 'O': O, 'R': R,
                        'n': num_planets,
                        'offset_angle': geometry_result,
                        'variance': variance,
                        'orbit_ratio': ratio,
                        'tooth_period': 360/R,
                        'phases': phases
                    }
                    
                    valid_configs.append(config)
                    
                    print(f"✓ FOUND: S={S:2d}, I={I:2d}, O={O:2d}, R={R:3d}, n={num_planets}")
                    print(f"  Offset: {geometry_result:.1f}°, Phase variance: {variance:.4f}°")
                    print(f"  Ring demands: {[f'{p:.1f}°' for p in phases['ring_demands'][:3]]}...")
                    print()
        
        if tested > max_tests:
            break
    if tested > max_tests:
        break

print(f"\nTested {tested} configurations")
print(f"Found {len(valid_configs)} valid configurations")

if valid_configs:
    print("\n" + "=" * 70)
    print("Best Configurations (sorted by phase alignment quality)")
    print("=" * 70)
    
    # Sort by variance (lower is better)
    valid_configs.sort(key=lambda c: c['variance'])
    
    for i, config in enumerate(valid_configs[:10]):
        print(f"\n{i+1}. S={config['S']}, I={config['I']}, O={config['O']}, R={config['R']} ({config['n']} planets)")
        print(f"   Geometry: {config['offset_angle']:.1f}° offset between inner-outer")
        print(f"   Phase variance: {config['variance']:.6f}° (tooth period: {config['tooth_period']:.3f}°)")
        print(f"   Orbit ratio: {config['orbit_ratio']:.3f}")
        
        # Save the best one
        if i == 0:
            best = config
            
            # Create JSON configuration
            json_config = {
                "schema": "slvs-json/1",
                "units": "mm",
                "parameters": {
                    "module": 2.0,
                    "pressure_angle": 20.0,
                    "sun_teeth": best['S'],
                    "inner_teeth": best['I'],
                    "outer_teeth": best['O'],
                    "ring_teeth": best['R']
                },
                "entities": [],
                "constraints": []
            }
            
            # Add sun and ring
            json_config["entities"].extend([
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
                    "phase": best['phases']['ring_demands'][0],  # Use first demand
                    "internal": True
                }
            ])
            
            # Add planets
            module = 2.0
            inner_orbit = (best['S'] + best['I']) * module / 2
            outer_orbit = (best['R'] - best['O']) * module / 2
            
            for j in range(best['n']):
                inner_angle = j * 360 / best['n']
                outer_angle = inner_angle + best['offset_angle']
                
                # Inner planet
                ix = inner_orbit * math.cos(math.radians(inner_angle))
                iy = inner_orbit * math.sin(math.radians(inner_angle))
                
                json_config["entities"].append({
                    "type": "gear",
                    "id": f"inner{j+1}",
                    "center": [round(ix, 2), round(iy, 2), 0],
                    "teeth": "$inner_teeth",
                    "module": "$module",
                    "pressure_angle": "$pressure_angle",
                    "phase": best['phases']['inner'][j],
                    "internal": False
                })
                
                # Outer planet
                ox = outer_orbit * math.cos(math.radians(outer_angle))
                oy = outer_orbit * math.sin(math.radians(outer_angle))
                
                json_config["entities"].append({
                    "type": "gear",
                    "id": f"outer{j+1}",
                    "center": [round(ox, 2), round(oy, 2), 0],
                    "teeth": "$outer_teeth",
                    "module": "$module",
                    "pressure_angle": "$pressure_angle",
                    "phase": best['phases']['outer'][j],
                    "internal": False
                })
                
                # Constraints
                json_config["constraints"].extend([
                    {"type": "mesh", "gear1": "sun", "gear2": f"inner{j+1}"},
                    {"type": "mesh", "gear1": f"inner{j+1}", "gear2": f"outer{j+1}"},
                    {"type": "mesh", "gear1": "ring", "gear2": f"outer{j+1}"}
                ])
            
            # Save configuration
            filename = f"testdata/optimal_double_planetary_{best['n']}p.json"
            with open(filename, 'w') as f:
                json.dump(json_config, f, indent=2)
            
            print(f"\n   Saved to: {filename}")
            
            # Detail the phase alignment
            print(f"\n   Phase Analysis:")
            print(f"   Ring demands: {[f'{p:.2f}°' for p in best['phases']['ring_demands']]}")
            tooth_period = 360 / best['R']
            normalized = [(p % 360) % tooth_period for p in best['phases']['ring_demands']]
            print(f"   Normalized within tooth: {[f'{n:.3f}°' for n in normalized]}")

else:
    print("\nNo valid configurations found in search space!")
    print("This suggests the double planetary problem may require:")
    print("1. Different mechanical arrangement (outer meshing with 2 inner)")
    print("2. Non-standard tooth profiles")
    print("3. Accepting some phase error with compliance")