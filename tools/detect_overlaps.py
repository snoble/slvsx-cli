#!/usr/bin/env python3
"""
SVG Gear Overlap Detector

Analyzes an SVG file containing gears and detects overlapping teeth.
"""

import re
import math
import sys
from dataclasses import dataclass
from typing import List, Tuple, Optional
import xml.etree.ElementTree as ET

@dataclass
class Point:
    x: float
    y: float
    
    def distance_to(self, other: 'Point') -> float:
        return math.sqrt((self.x - other.x)**2 + (self.y - other.y)**2)
    
    def angle_from_origin(self) -> float:
        return math.atan2(self.y, self.x)

@dataclass
class LineSegment:
    p1: Point
    p2: Point
    gear_id: str
    
    def intersects(self, other: 'LineSegment') -> bool:
        """Check if two line segments intersect"""
        # Skip if same gear
        if self.gear_id == other.gear_id:
            return False
            
        # Use cross product method
        def ccw(A, B, C):
            return (C.y - A.y) * (B.x - A.x) > (B.y - A.y) * (C.x - A.x)
        
        A, B = self.p1, self.p2
        C, D = other.p1, other.p2
        
        return ccw(A, C, D) != ccw(B, C, D) and ccw(A, B, C) != ccw(A, B, D)
    
    def min_distance_to(self, other: 'LineSegment') -> float:
        """Calculate minimum distance between two line segments"""
        # Skip if same gear
        if self.gear_id == other.gear_id:
            return float('inf')
            
        # Check all point-to-segment distances
        distances = [
            self._point_to_segment_distance(other.p1, self.p1, self.p2),
            self._point_to_segment_distance(other.p2, self.p1, self.p2),
            self._point_to_segment_distance(self.p1, other.p1, other.p2),
            self._point_to_segment_distance(self.p2, other.p1, other.p2),
        ]
        return min(distances)
    
    def _point_to_segment_distance(self, point: Point, seg_start: Point, seg_end: Point) -> float:
        """Calculate distance from point to line segment"""
        # Vector from seg_start to seg_end
        dx = seg_end.x - seg_start.x
        dy = seg_end.y - seg_start.y
        
        if dx == 0 and dy == 0:
            # Segment is a point
            return point.distance_to(seg_start)
        
        # Parameter t of closest point on line
        t = max(0, min(1, ((point.x - seg_start.x) * dx + (point.y - seg_start.y) * dy) / (dx * dx + dy * dy)))
        
        # Closest point on segment
        closest = Point(seg_start.x + t * dx, seg_start.y + t * dy)
        
        return point.distance_to(closest)

def parse_svg_path(path_data: str, gear_id: str) -> List[LineSegment]:
    """Parse SVG path data and extract line segments"""
    segments = []
    
    # Parse path commands
    commands = re.findall(r'([MLAZ])\s*([^MLAZ]*)', path_data)
    
    current_pos = None
    first_pos = None
    
    for cmd, args in commands:
        if cmd == 'M':  # Move to
            coords = list(map(float, args.split()))
            current_pos = Point(coords[0], coords[1])
            first_pos = current_pos
            
        elif cmd == 'L':  # Line to
            coords = list(map(float, args.split()))
            new_pos = Point(coords[0], coords[1])
            if current_pos:
                segments.append(LineSegment(current_pos, new_pos, gear_id))
            current_pos = new_pos
            
        elif cmd == 'A':  # Arc (approximate with lines)
            parts = args.split()
            if len(parts) >= 5:
                # Skip arc parameters, just get end point
                end_x = float(parts[-2])
                end_y = float(parts[-1])
                new_pos = Point(end_x, end_y)
                if current_pos:
                    # Approximate arc with straight line for now
                    segments.append(LineSegment(current_pos, new_pos, gear_id))
                current_pos = new_pos
                
        elif cmd == 'Z':  # Close path
            if current_pos and first_pos:
                segments.append(LineSegment(current_pos, first_pos, gear_id))
    
    return segments

def analyze_svg(filename: str) -> Tuple[List[LineSegment], List[Tuple[str, str, float]]]:
    """Analyze SVG file for gear overlaps"""
    tree = ET.parse(filename)
    root = tree.getroot()
    
    all_segments = []
    gear_ids = []
    
    # Find all gear groups
    for g in root.findall('.//{http://www.w3.org/2000/svg}g'):
        gear_id = g.get('id', '')
        if gear_id:
            gear_ids.append(gear_id)
            # Find path within group
            for path in g.findall('.//{http://www.w3.org/2000/svg}path'):
                path_data = path.get('d', '')
                if path_data:
                    segments = parse_svg_path(path_data, gear_id)
                    all_segments.extend(segments)
    
    # Check for overlaps
    overlaps = []
    checked_pairs = set()
    
    for i, seg1 in enumerate(all_segments):
        for j, seg2 in enumerate(all_segments[i+1:], i+1):
            # Skip if same gear
            if seg1.gear_id == seg2.gear_id:
                continue
                
            # Skip if we already checked this gear pair
            pair = tuple(sorted([seg1.gear_id, seg2.gear_id]))
            if pair in checked_pairs:
                continue
                
            # Check for intersection
            if seg1.intersects(seg2):
                distance = seg1.min_distance_to(seg2)
                overlaps.append((seg1.gear_id, seg2.gear_id, distance))
                checked_pairs.add(pair)
                break  # One overlap per gear pair is enough
    
    return all_segments, overlaps

def calculate_mesh_quality(segments: List[LineSegment]) -> dict:
    """Calculate mesh quality metrics"""
    gear_segments = {}
    for seg in segments:
        if seg.gear_id not in gear_segments:
            gear_segments[seg.gear_id] = []
        gear_segments[seg.gear_id].append(seg)
    
    metrics = {}
    
    # Check each pair of gears
    for gear1 in gear_segments:
        for gear2 in gear_segments:
            if gear1 >= gear2:
                continue
                
            # Find minimum distance between gears
            min_dist = float('inf')
            for seg1 in gear_segments[gear1]:
                for seg2 in gear_segments[gear2]:
                    dist = seg1.min_distance_to(seg2)
                    min_dist = min(min_dist, dist)
            
            if min_dist < float('inf'):
                pair_name = f"{gear1}-{gear2}"
                metrics[pair_name] = {
                    'min_distance': min_dist,
                    'status': 'overlap' if min_dist < 0.1 else 'clearance'
                }
    
    return metrics

def main():
    if len(sys.argv) != 2:
        print("Usage: python detect_overlaps.py <svg_file>")
        sys.exit(1)
    
    svg_file = sys.argv[1]
    
    try:
        segments, overlaps = analyze_svg(svg_file)
        metrics = calculate_mesh_quality(segments)
        
        print(f"Analyzed {svg_file}")
        print(f"Found {len(segments)} line segments")
        print()
        
        if overlaps:
            print(f"⚠️  Found {len(overlaps)} overlapping gear pairs:")
            for gear1, gear2, distance in overlaps:
                print(f"  - {gear1} overlaps with {gear2}")
        else:
            print("✓ No overlaps detected")
        
        print()
        print("Gear pair distances:")
        for pair, info in sorted(metrics.items()):
            status_icon = "❌" if info['status'] == 'overlap' else "✓"
            print(f"  {status_icon} {pair}: {info['min_distance']:.3f}mm ({info['status']})")
        
    except Exception as e:
        print(f"Error analyzing SVG: {e}")
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python detect_overlaps.py <svg_file>")
        sys.exit(1)
    
    svg_file = sys.argv[1]
    
    try:
        segments, overlaps = analyze_svg(svg_file)
        metrics = calculate_mesh_quality(segments)
        
        print(f"Analyzed {svg_file}")
        print(f"Found {len(segments)} line segments")
        print()
        
        if overlaps:
            print(f"⚠️  Found {len(overlaps)} overlapping gear pairs:")
            for gear1, gear2, distance in overlaps:
                print(f"  - {gear1} overlaps with {gear2}")
        else:
            print("✓ No overlaps detected")
        
        print()
        print("Gear pair distances:")
        for pair, info in sorted(metrics.items()):
            status_icon = "❌" if info['status'] == 'overlap' else "✓"
            print(f"  {status_icon} {pair}: {info['min_distance']:.3f}mm ({info['status']})")
        
        # Exit with error code if any overlaps found (for CI)
        # This is CRITICAL for 3D printing - ANY overlap will cause print failure
        if overlaps:
            print(f"\n❌ FAILURE: {len(overlaps)} overlaps detected - 3D print will fail!")
            sys.exit(1)  # Failure
        else:
            print(f"\n✅ SUCCESS: No overlaps detected - safe for 3D printing")
            sys.exit(0)  # Success
            
    except Exception as e:
        print(f"Error analyzing SVG: {e}")
        sys.exit(1)