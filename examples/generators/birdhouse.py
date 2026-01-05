#!/usr/bin/env python3
"""
Birdhouse Generator - High-level geometry builder for SLVSX

This script demonstrates how to build complex geometry using
reusable components like walls, boxes, and holes.

Usage:
    python birdhouse.py > ../21_birdhouse.json
    # or
    python birdhouse.py | slvsx solve -
"""

import json
import math
from dataclasses import dataclass, field
from typing import List, Tuple, Optional

# ============================================================================
# Core Types
# ============================================================================

@dataclass
class Point3D:
    x: float
    y: float
    z: float
    
    def offset(self, dx: float = 0, dy: float = 0, dz: float = 0) -> 'Point3D':
        return Point3D(self.x + dx, self.y + dy, self.z + dz)
    
    def to_list(self) -> List[float]:
        return [self.x, self.y, self.z]


@dataclass 
class Entity:
    """Base entity that will be converted to JSON"""
    type: str
    id: str
    
    def to_dict(self) -> dict:
        raise NotImplementedError


@dataclass
class PointEntity(Entity):
    at: Point3D
    
    def __init__(self, id: str, at: Point3D):
        super().__init__("point", id)
        self.at = at
    
    def to_dict(self) -> dict:
        return {"type": "point", "id": self.id, "at": self.at.to_list()}


@dataclass
class LineEntity(Entity):
    p1: str
    p2: str
    
    def __init__(self, id: str, p1: str, p2: str):
        super().__init__("line", id)
        self.p1 = p1
        self.p2 = p2
    
    def to_dict(self) -> dict:
        return {"type": "line", "id": self.id, "p1": self.p1, "p2": self.p2}


@dataclass
class CircleEntity(Entity):
    center: Point3D
    diameter: float
    
    def __init__(self, id: str, center: Point3D, diameter: float):
        super().__init__("circle", id)
        self.center = center
        self.diameter = diameter
    
    def to_dict(self) -> dict:
        return {"type": "circle", "id": self.id, "center": self.center.to_list(), "diameter": self.diameter}


@dataclass
class Constraint:
    """Base constraint"""
    type: str
    
    def to_dict(self) -> dict:
        raise NotImplementedError


@dataclass
class FixedConstraint(Constraint):
    entity: str
    
    def __init__(self, entity: str):
        super().__init__("fixed")
        self.entity = entity
    
    def to_dict(self) -> dict:
        return {"type": "fixed", "entity": self.entity}


@dataclass
class DistanceConstraint(Constraint):
    between: Tuple[str, str]
    value: float
    
    def __init__(self, p1: str, p2: str, value: float):
        super().__init__("distance")
        self.between = (p1, p2)
        self.value = value
    
    def to_dict(self) -> dict:
        return {"type": "distance", "between": list(self.between), "value": self.value}


# ============================================================================
# Geometry Builder
# ============================================================================

class GeometryBuilder:
    """High-level geometry builder with CAD-like abstractions"""
    
    def __init__(self):
        self.entities: List[Entity] = []
        self.constraints: List[Constraint] = []
        self.parameters: dict = {}
        self._point_counter = 0
        self._line_counter = 0
    
    def set_parameter(self, name: str, value: float):
        """Define a named parameter"""
        self.parameters[name] = value
    
    def _next_point_id(self, prefix: str = "p") -> str:
        self._point_counter += 1
        return f"{prefix}_{self._point_counter}"
    
    def _next_line_id(self, prefix: str = "l") -> str:
        self._line_counter += 1
        return f"{prefix}_{self._line_counter}"
    
    def add_point(self, id: str, pos: Point3D, fixed: bool = False) -> str:
        """Add a single point"""
        self.entities.append(PointEntity(id, pos))
        if fixed:
            self.constraints.append(FixedConstraint(id))
        return id
    
    def add_line(self, id: str, p1: str, p2: str) -> str:
        """Add a line between two points"""
        self.entities.append(LineEntity(id, p1, p2))
        return id
    
    def add_circle(self, id: str, center: Point3D, diameter: float) -> str:
        """Add a circle"""
        self.entities.append(CircleEntity(id, center, diameter))
        return id
    
    # ========================================================================
    # High-Level Components
    # ========================================================================
    
    def add_rectangle(self, prefix: str, 
                      origin: Point3D, 
                      width: float, 
                      depth: float,
                      fixed: bool = False) -> dict:
        """
        Add a rectangle in the XY plane at given Z height.
        Returns dict with corner point IDs.
        """
        corners = {
            'fl': self.add_point(f"{prefix}_fl", origin, fixed),
            'fr': self.add_point(f"{prefix}_fr", origin.offset(dx=width), fixed),
            'br': self.add_point(f"{prefix}_br", origin.offset(dx=width, dy=depth), fixed),
            'bl': self.add_point(f"{prefix}_bl", origin.offset(dy=depth), fixed),
        }
        
        # Add edges
        self.add_line(f"{prefix}_front", corners['fl'], corners['fr'])
        self.add_line(f"{prefix}_right", corners['fr'], corners['br'])
        self.add_line(f"{prefix}_back", corners['br'], corners['bl'])
        self.add_line(f"{prefix}_left", corners['bl'], corners['fl'])
        
        return corners
    
    def add_box(self, prefix: str,
                origin: Point3D,
                width: float,
                depth: float, 
                height: float,
                fixed: bool = False) -> dict:
        """
        Add a 3D box wireframe.
        Returns dict with all 8 corner point IDs.
        """
        # Bottom rectangle
        bottom = self.add_rectangle(f"{prefix}_base", origin, width, depth, fixed)
        
        # Top rectangle  
        top_origin = origin.offset(dz=height)
        top = self.add_rectangle(f"{prefix}_top", top_origin, width, depth, fixed)
        
        # Vertical edges
        self.add_line(f"{prefix}_edge_fl", bottom['fl'], top['fl'])
        self.add_line(f"{prefix}_edge_fr", bottom['fr'], top['fr'])
        self.add_line(f"{prefix}_edge_br", bottom['br'], top['br'])
        self.add_line(f"{prefix}_edge_bl", bottom['bl'], top['bl'])
        
        return {'bottom': bottom, 'top': top}
    
    def add_hollow_box(self, prefix: str,
                       origin: Point3D,
                       width: float,
                       depth: float,
                       height: float,
                       wall_thickness: float,
                       fixed_outer: bool = False) -> dict:
        """
        Add a hollow box with wall thickness.
        Creates outer and inner shells.
        """
        # Outer shell
        outer = self.add_box(f"{prefix}_outer", origin, width, depth, height, fixed_outer)
        
        # Inner shell (offset inward by wall_thickness)
        inner_origin = origin.offset(dx=wall_thickness, dy=wall_thickness)
        inner_width = width - 2 * wall_thickness
        inner_depth = depth - 2 * wall_thickness
        inner = self.add_box(f"{prefix}_inner", inner_origin, inner_width, inner_depth, height, False)
        
        # Add distance constraints to maintain wall thickness
        # (diagonal distance = wall_thickness * sqrt(2))
        diag = wall_thickness * math.sqrt(2)
        for corner in ['fl', 'fr', 'br', 'bl']:
            self.constraints.append(DistanceConstraint(
                outer['bottom'][corner], 
                inner['bottom'][corner], 
                diag
            ))
            self.constraints.append(DistanceConstraint(
                outer['top'][corner], 
                inner['top'][corner], 
                diag
            ))
        
        return {'outer': outer, 'inner': inner}
    
    def add_gable_roof(self, prefix: str,
                       base_corners: dict,
                       ridge_height: float,
                       fixed: bool = False) -> dict:
        """
        Add a gable roof on top of a rectangle.
        base_corners should have 'fl', 'fr', 'bl', 'br' keys.
        """
        # Get base positions to calculate ridge position
        fl_entity = next(e for e in self.entities if e.id == base_corners['fl'])
        fr_entity = next(e for e in self.entities if e.id == base_corners['fr'])
        bl_entity = next(e for e in self.entities if e.id == base_corners['bl'])
        
        # Ridge runs front to back, centered on width
        mid_x = (fl_entity.at.x + fr_entity.at.x) / 2
        ridge_z = fl_entity.at.z + ridge_height
        
        ridge_front = self.add_point(
            f"{prefix}_ridge_front",
            Point3D(mid_x, fl_entity.at.y, ridge_z),
            fixed
        )
        ridge_back = self.add_point(
            f"{prefix}_ridge_back", 
            Point3D(mid_x, bl_entity.at.y, ridge_z),
            fixed
        )
        
        # Gable edges
        self.add_line(f"{prefix}_gable_fl", base_corners['fl'], ridge_front)
        self.add_line(f"{prefix}_gable_fr", base_corners['fr'], ridge_front)
        self.add_line(f"{prefix}_gable_bl", base_corners['bl'], ridge_back)
        self.add_line(f"{prefix}_gable_br", base_corners['br'], ridge_back)
        self.add_line(f"{prefix}_ridge", ridge_front, ridge_back)
        
        return {'front': ridge_front, 'back': ridge_back}
    
    def add_hole(self, prefix: str,
                 outer_pos: Point3D,
                 inner_pos: Point3D,
                 diameter: float) -> dict:
        """
        Add a hole that goes through a wall.
        Creates circles on both outer and inner surfaces.
        """
        outer_id = self.add_circle(f"{prefix}_outer", outer_pos, diameter)
        inner_id = self.add_circle(f"{prefix}_inner", inner_pos, diameter)
        return {'outer': outer_id, 'inner': inner_id}
    
    def add_dowel(self, prefix: str,
                  start: Point3D,
                  end: Point3D,
                  diameter: float,
                  fixed: bool = False) -> dict:
        """Add a cylindrical dowel (line with circle at end)"""
        start_id = self.add_point(f"{prefix}_start", start, fixed)
        end_id = self.add_point(f"{prefix}_end", end, fixed)
        line_id = self.add_line(f"{prefix}_shaft", start_id, end_id)
        cap_id = self.add_circle(f"{prefix}_cap", end, diameter)
        return {'start': start_id, 'end': end_id, 'line': line_id, 'cap': cap_id}
    
    # ========================================================================
    # Output
    # ========================================================================
    
    def to_dict(self, description: str = "") -> dict:
        """Convert to SLVSX JSON format"""
        return {
            "schema": "slvs-json/1",
            "units": "mm",
            "description": description,
            "parameters": self.parameters,
            "entities": [e.to_dict() for e in self.entities],
            "constraints": [c.to_dict() for c in self.constraints],
        }
    
    def to_json(self, description: str = "", indent: int = 2) -> str:
        """Convert to JSON string"""
        return json.dumps(self.to_dict(description), indent=indent)


# ============================================================================
# Birdhouse Builder
# ============================================================================

def build_birdhouse(
    width: float = 150,
    depth: float = 150,
    wall_height: float = 180,
    roof_height: float = 80,
    wall_thickness: float = 10,
    entrance_diameter: float = 40,
    entrance_height: float = 120,
    perch_length: float = 25,
    perch_diameter: float = 8,
) -> GeometryBuilder:
    """
    Build a birdhouse with parametric dimensions.
    
    All dimensions are in mm.
    """
    g = GeometryBuilder()
    
    # Store parameters
    g.set_parameter("width", width)
    g.set_parameter("depth", depth)
    g.set_parameter("wall_height", wall_height)
    g.set_parameter("roof_height", roof_height)
    g.set_parameter("wall_thickness", wall_thickness)
    g.set_parameter("entrance_diameter", entrance_diameter)
    
    # Create hollow box for walls
    box = g.add_hollow_box(
        "walls",
        origin=Point3D(0, 0, 0),
        width=width,
        depth=depth,
        height=wall_height,
        wall_thickness=wall_thickness,
        fixed_outer=True
    )
    
    # Add gable roofs (outer and inner)
    outer_roof = g.add_gable_roof(
        "outer_roof",
        box['outer']['top'],
        roof_height,
        fixed=True
    )
    
    inner_roof = g.add_gable_roof(
        "inner_roof", 
        box['inner']['top'],
        roof_height - wall_thickness,  # Slightly lower inner roof
        fixed=True
    )
    
    # Add entrance hole (front wall)
    entrance_x = width / 2
    g.add_hole(
        "entrance",
        outer_pos=Point3D(entrance_x, 0, entrance_height),
        inner_pos=Point3D(entrance_x, wall_thickness, entrance_height),
        diameter=entrance_diameter
    )
    
    # Add perch
    g.add_dowel(
        "perch",
        start=Point3D(entrance_x, 0, entrance_height - 30),
        end=Point3D(entrance_x, -perch_length, entrance_height - 30),
        diameter=perch_diameter,
        fixed=True
    )
    
    return g


# ============================================================================
# Main
# ============================================================================

if __name__ == "__main__":
    birdhouse = build_birdhouse(
        width=150,
        depth=150,
        wall_height=180,
        roof_height=80,
        wall_thickness=10,
        entrance_diameter=40,
    )
    
    print(birdhouse.to_json(
        description="Birdhouse with wall thickness. Generated by birdhouse.py. "
                    "Inner and outer shells define walls. Import to CAD and "
                    "loft between shells to create solid geometry."
    ))

