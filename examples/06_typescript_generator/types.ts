// Generated TypeScript types for SLVSX constraint system

export type ExprOrNumber = number | string;

export interface InputDocument {
  schema: "slvs-json/1";
  units?: string;
  parameters?: { [key: string]: number };
  entities: Entity[];
  constraints: Constraint[];
}

export type Entity =
  | PointEntity
  | LineEntity
  | CircleEntity
  | ArcEntity
  | PlaneEntity
  | GearEntity;

export interface PointEntity {
  type: "point";
  id: string;
  at: ExprOrNumber[];
}

export interface LineEntity {
  type: "line";
  id: string;
  p1: string;
  p2: string;
}

export interface CircleEntity {
  type: "circle";
  id: string;
  center: ExprOrNumber[];
  diameter: ExprOrNumber;
}

export interface ArcEntity {
  type: "arc";
  id: string;
  center: ExprOrNumber[];
  start: string;
  end: string;
}

export interface PlaneEntity {
  type: "plane";
  id: string;
  origin: ExprOrNumber[];
  normal: ExprOrNumber[];
}

export interface GearEntity {
  type: "gear";
  id: string;
  center: ExprOrNumber[];
  teeth: ExprOrNumber;
  module: ExprOrNumber;
  pressure_angle: ExprOrNumber;
  phase?: ExprOrNumber;
  internal?: boolean;
}

export type Constraint =
  | CoincidentConstraint
  | DistanceConstraint
  | AngleConstraint
  | PerpendicularConstraint
  | ParallelConstraint
  | HorizontalConstraint
  | VerticalConstraint
  | EqualLengthConstraint
  | EqualRadiusConstraint
  | TangentConstraint
  | PointOnLineConstraint
  | PointOnCircleConstraint
  | FixedConstraint
  | MeshConstraint;

export interface CoincidentConstraint {
  type: "coincident";
  at: string;
  of: string[];
}

export interface DistanceConstraint {
  type: "distance";
  between: [string, string];
  value: ExprOrNumber;
}

export interface AngleConstraint {
  type: "angle";
  between: [string, string];
  value: ExprOrNumber;
}

export interface PerpendicularConstraint {
  type: "perpendicular";
  a: string;
  b: string;
}

export interface ParallelConstraint {
  type: "parallel";
  a: string;
  b: string;
}

export interface HorizontalConstraint {
  type: "horizontal";
  a: string;
}

export interface VerticalConstraint {
  type: "vertical";
  a: string;
}

export interface EqualLengthConstraint {
  type: "equal_length";
  a: string;
  b: string;
}

export interface EqualRadiusConstraint {
  type: "equal_radius";
  a: string;
  b: string;
}

export interface TangentConstraint {
  type: "tangent";
  a: string;
  b: string;
}

export interface PointOnLineConstraint {
  type: "point_on_line";
  point: string;
  line: string;
}

export interface PointOnCircleConstraint {
  type: "point_on_circle";
  point: string;
  circle: string;
}

export interface FixedConstraint {
  type: "fixed";
  entity: string;
}

export interface MeshConstraint {
  type: "mesh";
  gear1: string;
  gear2: string;
}