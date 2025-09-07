#!/usr/bin/env python3
"""
Find configurations with perfect tooth-valley meshing.
Focus on low tooth counts and simple ratios.
"""

import math
import numpy as np

def calculate_mesh_alignment(S, I, O, R):
    """
    Calculate if all gears mesh with perfect tooth-valley alignment.
    Returns alignment quality score (0 = perfect).
    """
    scores = []
    
    # Check sun-inner meshing (6 meshes)
    for i in range(6):
        angle = i * 60  # degrees
        # For external-external mesh, teeth should interleave
        # Phase difference should be half a tooth period
        inner_phase = (angle * S / I) % (360 / I)
        ideal_phase = (360 / I) / 2  # Half tooth period
        error = min(abs(inner_phase - ideal_phase), 
                   abs(inner_phase - 0),
                   abs(inner_phase - (360/I)))
        scores.append(error)
    
    # For triangular meshing, check inner-outer alignment
    # This is more complex but critical
    
    return np.mean(scores)

def find_simple_ratios():
    """Find configurations with simple tooth ratios that promote alignment."""
    
    print("Searching for perfect meshing configurations...")
    print("Focusing on low tooth counts and simple ratios")
    print("=" * 70)
    
    configs = []
    
    # Focus on very small tooth counts for better alignment
    for S in [12, 18, 24, 30, 36]:  # Sun teeth
        for I in [6, 8, 9, 10, 12]:  # Inner teeth - small!
            if I >= S:
                continue
                
            for O in [6, 8, 9, 10, 12, 15, 16, 18]:  # Outer teeth
                # Calculate required ring teeth for different arrangements
                
                # For triangular meshing geometry
                inner_radius = (S + I) / 2
                
                # Approximate outer radius for triangular meshing
                inner_spacing = 2 * inner_radius * math.sin(math.pi / 6)
                mesh_dist = (I + O) / 2
                
                if mesh_dist <= inner_spacing / 2:
                    continue
                    
                h = math.sqrt(mesh_dist**2 - (inner_spacing/2)**2) if mesh_dist > inner_spacing/2 else 0
                if h == 0:
                    continue
                    
                mid_radius = inner_radius * math.cos(math.pi / 6)
                outer_radius = mid_radius + h
                
                # Ring teeth to mesh with outer at this radius
                R_needed = round(2 * outer_radius + O)
                
                # Try nearby even values
                for R in [R_needed - 6, R_needed - 3, R_needed, R_needed + 3, R_needed + 6]:
                    if R <= 0 or R % 3 != 0:  # Ring teeth should be divisible by 3 for symmetry
                        continue
                    
                    # Check assembly constraint
                    if (S + R) % 6 != 0:
                        continue
                    
                    # Check if ratios are simple (promotes good meshing)
                    ratio_SI = S / math.gcd(S, I)
                    ratio_IO = I / math.gcd(I, O)
                    ratio_OR = O / math.gcd(O, R)
                    
                    # Prefer simple ratios
                    complexity = ratio_SI + ratio_IO + ratio_OR
                    
                    if complexity <= 15:  # Simple enough
                        actual_ring_radius = (R - O) / 2
                        ring_error = abs(outer_radius - actual_ring_radius)
                        
                        if ring_error < 2.0:  # Reasonable fit
                            # Check clearances
                            inner_edge = inner_radius + I/2 + 1
                            ring_inner = R/2 - 1
                            clearance = ring_inner - inner_edge
                            
                            if clearance > 5:  # Good clearance
                                configs.append({
                                    'S': S, 'I': I, 'O': O, 'R': R,
                                    'complexity': complexity,
                                    'ring_error': ring_error,
                                    'clearance': clearance,
                                    'ratios': (ratio_SI, ratio_IO, ratio_OR)
                                })
    
    # Sort by simplicity and ring error
    configs.sort(key=lambda x: (x['complexity'], x['ring_error']))
    
    print(f"Found {len(configs)} configurations")
    print("\nTop configurations (simplest ratios, best fit):")
    print()
    
    for i, cfg in enumerate(configs[:10]):
        S, I, O, R = cfg['S'], cfg['I'], cfg['O'], cfg['R']
        print(f"{i+1}. S={S}, I={I}, O={O}, R={R}")
        print(f"   Ratios: S:I={cfg['ratios'][0]:.1f}, I:O={cfg['ratios'][1]:.1f}, O:R={cfg['ratios'][2]:.1f}")
        print(f"   Complexity: {cfg['complexity']:.1f}")
        print(f"   Ring error: {cfg['ring_error']:.2f} teeth")
        print(f"   Inner-ring clearance: {cfg['clearance']:.1f} teeth")
        
        # Special cases that often work well
        if S % I == 0:
            print(f"   ✓ S/I = {S//I} (integer ratio - good for phase alignment)")
        if I % 2 == 0 and O % 2 == 0:
            print(f"   ✓ Even I and O (symmetric meshing)")
        if R % S == 0:
            print(f"   ✓ R/S = {R//S} (integer ratio)")
        print()

find_simple_ratios()

print("\n" + "=" * 70)
print("Recommendation: Try configurations with:")
print("- Integer ratios between S and I")
print("- Even tooth counts for symmetry")
print("- Small tooth counts for larger angular tolerance")