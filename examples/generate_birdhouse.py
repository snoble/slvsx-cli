#!/usr/bin/env python3
"""
Generate a birdhouse JSON file programmatically.
Applying the Design Methodology from docs/DESIGN_METHODOLOGY.md

Phase 1: Research - Classic birdhouse proportions
Phase 2: Foundation - Build base box
Phase 3: Features - Add roof, entrance, perch
Phase 4: Refinement - Perfect proportions and constraints
"""

import json
import math
import subprocess
import sys

def generate_birdhouse():
    # Parameters - classic birdhouse proportions (6x6 inch base, ~8 inch height)
    params = {
        "width": 152.0,      # 6 inches in mm
        "depth": 152.0,      # 6 inches in mm  
        "height": 200.0,     # ~8 inches in mm
        "roof_height": 100.0,
        "roof_overhang": 25.0,
        "entrance_height": 150.0,  # Height from base to entrance center
        "entrance_diameter": 38.0,  # 1.5 inches
        "perch_length": 20.0
    }
    
    entities = []
    constraints = []
    
    # Helper functions for cleaner code
    def add_point(id_name, x, y, z):
        entities.append({
            "type": "point",
            "id": id_name,
            "at": [x, y, z]
        })
    
    def add_line(id_name, p1, p2):
        entities.append({
            "type": "line",
            "id": id_name,
            "p1": p1,
            "p2": p2
        })
    
    w, d, h = params["width"], params["depth"], params["height"]
    
    # ============================================
    # PHASE 2: FOUNDATION - Base Box
    # ============================================
    
    # Base rectangle corners
    add_point("base_front_left", 0, 0, 0)
    add_point("base_front_right", w, 0, 0)
    add_point("base_back_left", 0, d, 0)
    add_point("base_back_right", w, d, 0)
    
    # Top rectangle corners
    add_point("top_front_left", 0, 0, h)
    add_point("top_front_right", w, 0, h)
    add_point("top_back_left", 0, d, h)
    add_point("top_back_right", w, d, h)
    
    # Base edges
    add_line("base_front", "base_front_left", "base_front_right")
    add_line("base_back", "base_back_left", "base_back_right")
    add_line("base_left", "base_front_left", "base_back_left")
    add_line("base_right", "base_front_right", "base_back_right")
    
    # Vertical edges
    add_line("front_left_edge", "base_front_left", "top_front_left")
    add_line("front_right_edge", "base_front_right", "top_front_right")
    add_line("back_left_edge", "base_back_left", "top_back_left")
    add_line("back_right_edge", "base_back_right", "top_back_right")
    
    # Top edges
    add_line("top_front", "top_front_left", "top_front_right")
    add_line("top_back", "top_back_left", "top_back_right")
    add_line("top_left", "top_front_left", "top_back_left")
    add_line("top_right", "top_front_right", "top_back_right")
    
    # ============================================
    # PHASE 3: FEATURES - Roof
    # ============================================
    
    # Roof peak - centered horizontally, forward of front edge for overhang
    roof_h = params["roof_height"]
    overhang = params["roof_overhang"]
    peak_x = w / 2
    peak_y = -overhang  # Forward overhang
    peak_z = h + roof_h
    add_point("roof_peak", peak_x, peak_y, peak_z)
    
    # Roof ridges (from top corners to peak)
    add_line("roof_front_left_ridge", "top_front_left", "roof_peak")
    add_line("roof_front_right_ridge", "top_front_right", "roof_peak")
    add_line("roof_back_left_ridge", "top_back_left", "roof_peak")
    add_line("roof_back_right_ridge", "top_back_right", "roof_peak")
    
    # ============================================
    # PHASE 3: FEATURES - Entrance and Perch
    # ============================================
    
    # Entrance hole - centered horizontally, at specified height
    entrance_h = params["entrance_height"]
    entrance_x = w / 2
    add_point("entrance_center", entrance_x, 0, entrance_h)
    entities.append({
        "type": "circle",
        "id": "entrance_hole",
        "center": [entrance_x, 0, entrance_h],
        "diameter": "$entrance_diameter"
    })
    
    # Perch below entrance
    perch_len = params["perch_length"]
    add_point("perch_end", entrance_x, -perch_len, entrance_h)
    add_line("perch", "entrance_center", "perch_end")
    
    # ============================================
    # CONSTRAINTS
    # ============================================
    
    # Foundation: Fix base corner
    constraints.append({
        "type": "fixed",
        "entity": "base_front_left"
    })
    
    # Foundation: Base rectangle dimensions
    constraints.append({"type": "distance", "between": ["base_front_left", "base_front_right"], "value": "$width"})
    constraints.append({"type": "distance", "between": ["base_front_left", "base_back_left"], "value": "$depth"})
    constraints.append({"type": "distance", "between": ["base_front_right", "base_back_right"], "value": "$depth"})
    constraints.append({"type": "distance", "between": ["base_back_left", "base_back_right"], "value": "$width"})
    
    # Foundation: Vertical edges (height)
    constraints.append({"type": "distance", "between": ["base_front_left", "top_front_left"], "value": "$height"})
    constraints.append({"type": "distance", "between": ["base_front_right", "top_front_right"], "value": "$height"})
    constraints.append({"type": "distance", "between": ["base_back_left", "top_back_left"], "value": "$height"})
    constraints.append({"type": "distance", "between": ["base_back_right", "top_back_right"], "value": "$height"})
    
    # Foundation: Top rectangle dimensions
    constraints.append({"type": "distance", "between": ["top_front_left", "top_front_right"], "value": "$width"})
    constraints.append({"type": "distance", "between": ["top_front_left", "top_back_left"], "value": "$depth"})
    constraints.append({"type": "distance", "between": ["top_back_left", "top_back_right"], "value": "$width"})
    constraints.append({"type": "distance", "between": ["top_front_right", "top_back_right"], "value": "$depth"})
    
    # Foundation: Alignment constraints
    constraints.append({"type": "horizontal", "entity": "base_front"})
    constraints.append({"type": "horizontal", "entity": "base_back"})
    constraints.append({"type": "horizontal", "entity": "top_front"})
    constraints.append({"type": "horizontal", "entity": "top_back"})
    constraints.append({"type": "vertical", "entity": "front_left_edge"})
    constraints.append({"type": "vertical", "entity": "front_right_edge"})
    constraints.append({"type": "vertical", "entity": "back_left_edge"})
    constraints.append({"type": "vertical", "entity": "back_right_edge"})
    
    # Features: Roof peak height (distance from front corners)
    constraints.append({"type": "distance", "between": ["top_front_left", "roof_peak"], "value": "$roof_height"})
    constraints.append({"type": "distance", "between": ["top_front_right", "roof_peak"], "value": "$roof_height"})
    
    # Features: Make roof symmetric (all ridges equal length)
    constraints.append({"type": "equal_length", "entities": ["roof_front_left_ridge", "roof_front_right_ridge", "roof_back_left_ridge", "roof_back_right_ridge"]})
    
    # Features: Entrance hole position
    # Constrain to front face (Y=0) using point_on_line with front_left_edge
    constraints.append({"type": "point_on_line", "point": "entrance_center", "line": "front_left_edge"})
    # Height from base
    constraints.append({"type": "distance", "between": ["base_front_left", "entrance_center"], "value": "$entrance_height"})
    # Center horizontally - distance from top_front_left along top_front
    constraints.append({"type": "distance", "between": ["top_front_left", "entrance_center"], "value": 76.0})
    
    # Features: Perch
    constraints.append({"type": "horizontal", "entity": "perch"})
    constraints.append({"type": "distance", "between": ["entrance_center", "perch_end"], "value": "$perch_length"})
    
    return {
        "schema": "slvs-json/1",
        "units": "mm",
        "parameters": {k: v for k, v in params.items()},
        "entities": entities,
        "constraints": constraints
    }

if __name__ == "__main__":
    birdhouse = generate_birdhouse()
    print(json.dumps(birdhouse, indent=2))
