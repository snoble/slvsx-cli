#!/usr/bin/env python3
"""
Analyze if the 6 different phase demands for the ring are equivalent
modulo the rotational symmetry of the teeth.
"""

def analyze_phase_equivalence(phase_demands, num_teeth):
    """Check if phase demands are equivalent modulo tooth period."""
    
    tooth_period = 360.0 / num_teeth
    
    print(f"Ring gear: {num_teeth} teeth")
    print(f"Tooth period: {tooth_period:.3f}°")
    print(f"Phase demands: {phase_demands}")
    print()
    
    # Normalize all phases to [0, tooth_period)
    normalized = []
    for phase in phase_demands:
        # First normalize to [0, 360)
        phase_norm = phase % 360
        # Then find position within tooth period
        within_tooth = phase_norm % tooth_period
        normalized.append(within_tooth)
        
        tooth_number = int(phase_norm / tooth_period)
        print(f"  {phase:6.1f}° -> {phase_norm:6.1f}° -> Tooth #{tooth_number:2d} + {within_tooth:5.3f}°")
    
    print()
    
    # Check variance
    min_norm = min(normalized)
    max_norm = max(normalized)
    variance = max_norm - min_norm
    
    print(f"Normalized phases within tooth: {[f'{n:.3f}°' for n in normalized]}")
    print(f"Variance: {variance:.3f}°")
    
    # Check if they're close enough (within 10% of tooth period)
    tolerance = tooth_period * 0.1
    
    if variance < tolerance:
        print(f"✓ EQUIVALENT! All phases align within {tolerance:.3f}° tolerance")
        return True
    else:
        print(f"✗ NOT EQUIVALENT. Variance {variance:.3f}° exceeds {tolerance:.3f}° tolerance")
        
        # Check if there's a systematic pattern
        # For 6 planets, check if they're at regular intervals
        if len(phase_demands) == 6:
            # Sort normalized phases
            sorted_norm = sorted(normalized)
            intervals = []
            for i in range(len(sorted_norm)):
                next_i = (i + 1) % len(sorted_norm)
                interval = (sorted_norm[next_i] - sorted_norm[i]) % tooth_period
                intervals.append(interval)
            
            print(f"\nIntervals between phases: {[f'{i:.3f}°' for i in intervals]}")
            
            # Check if intervals are regular
            avg_interval = sum(intervals) / len(intervals)
            interval_variance = max(abs(i - avg_interval) for i in intervals)
            
            if interval_variance < tolerance:
                print(f"✓ Phases are regularly spaced with {avg_interval:.3f}° intervals")
                return True
    
    return False

# Test Case 1: 6-planet system with R=84
print("=" * 60)
print("Test 1: 6-planet system with 84-tooth ring")
print("=" * 60)

# From our earlier test output
phase_demands_84 = [145.0, 57.9, 276.4, 63.6, 133.6, 352.1]
result1 = analyze_phase_equivalence(phase_demands_84, 84)

print("\n" + "=" * 60)
print("Test 2: Original 3-planet system with 72-tooth ring")
print("=" * 60)

# From the 3-planet working_double_planetary test
phase_demands_72 = [70.0, 360.0, 50.0]  # Note: 360° = 0°
result2 = analyze_phase_equivalence(phase_demands_72, 72)

print("\n" + "=" * 60)
print("Test 3: What if phases were perfectly distributed?")
print("=" * 60)

# Ideal case: 6 phases evenly distributed
ideal_base = 15.0  # Starting phase
ideal_phases = [(ideal_base + i * 60) % 360 for i in range(6)]
result3 = analyze_phase_equivalence(ideal_phases, 84)

print("\n" + "=" * 60)
print("Analysis Summary")
print("=" * 60)

if not result1:
    print("\nThe phase conflicts are REAL - they don't cancel out modulo tooth period.")
    print("This explains why the validation fails.")
else:
    print("\nThe phases ARE equivalent modulo tooth period!")
    print("The system should work despite different absolute phases.")

# Additional analysis: What's the actual pattern?
print("\n" + "=" * 60)
print("Pattern Analysis for 84-tooth ring")
print("=" * 60)

tooth_period_84 = 360.0 / 84  # 4.286°

print(f"Tooth period: {tooth_period_84:.3f}°")
print("\nPhase demands grouped by tooth alignment:")

for phase in phase_demands_84:
    tooth_num = int((phase % 360) / tooth_period_84)
    offset = (phase % 360) % tooth_period_84
    print(f"  {phase:6.1f}° -> Tooth {tooth_num:2d} + {offset:.3f}°")

# Check if there's a half-tooth offset pattern
print("\nChecking for half-tooth offset pattern:")
half_tooth = tooth_period_84 / 2

for phase in phase_demands_84:
    offset = (phase % 360) % tooth_period_84
    if abs(offset - half_tooth) < 0.1:
        print(f"  {phase:6.1f}° is at half-tooth position!")
    elif offset < 0.1:
        print(f"  {phase:6.1f}° is at tooth boundary!")
    else:
        print(f"  {phase:6.1f}° is at arbitrary position {offset:.3f}°")