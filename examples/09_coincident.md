# Example 09: Coincident Points

**[← Working with Angles](https://github.com/snoble/slvsx-cli/blob/main/examples/08_angles.md)** | **[Next: Equal Length →](https://github.com/snoble/slvsx-cli/blob/main/examples/10_equal_length.md)**

## The Story

Sometimes different parts of your design need to connect at exactly the same point. Think of wires meeting at a junction, roads converging at an intersection, or structural members joining at a node. The coincident constraint forces two or more points to occupy the same position.

Let's create a truss node where multiple members meet.

## The Entities

We'll create:
1. Multiple structural members (lines)
2. Force their endpoints to meet at a common node

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "base_left",
      "at": [-50, 0, 0]
    },
    {
      "type": "point",
      "id": "base_right",
      "at": [50, 0, 0]
    },
    {
      "type": "point",
      "id": "top_left",
      "at": [-30, 60, 0]
    },
    {
      "type": "point",
      "id": "top_right",
      "at": [30, 60, 0]
    },
    {
      "type": "point",
      "id": "apex",
      "at": [0, 80, 0]
    },
    {
      "type": "line",
      "id": "left_vertical",
      "p1": "base_left",
      "p2": "top_left"
    },
    {
      "type": "line",
      "id": "right_vertical",
      "p1": "base_right",
      "p2": "top_right"
    },
    {
      "type": "line",
      "id": "left_diagonal",
      "p1": "top_left",
      "p2": "apex"
    },
    {
      "type": "line",
      "id": "right_diagonal",
      "p1": "top_right",
      "p2": "apex"
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "base_left"
    },
    {
      "type": "fixed",
      "entity": "base_right"
    },
    {
      "type": "coincident",
      "at": "top_left",
      "of": ["top_right"]
    },
    {
      "type": "coincident",
      "at": "top_left",
      "of": ["apex"]
    }
  ]
}
```

## Understanding the Code

- **`coincident` constraint**: Forces multiple points to the same location
- **Truss simplification**: The structure collapses from a rectangle to a triangle
- **Multiple coincidences**: Chain them to merge several points

## The Solution

The solver merges all specified points:

```json
{
  "status": "ok",
  "entities": {
    "base_left": { "at": [-50.0, 0.0, 0.0] },
    "base_right": { "at": [50.0, 0.0, 0.0] },
    "top_left": { "at": [0.0, 50.0, 0.0] },
    "top_right": { "at": [0.0, 50.0, 0.0] },
    "apex": { "at": [0.0, 50.0, 0.0] }
  }
}
```

All three top points converged to the same location, forming a simple triangle!

## Visual Output

![Coincident Points](https://raw.githubusercontent.com/snoble/slvsx-cli/main/examples/09_coincident.svg)

## Real-World Applications

- **Electrical circuits**: Wires joining at nodes
- **Trusses**: Members meeting at joints
- **Piping**: Pipes converging at manifolds
- **Road design**: Multiple roads at intersections

## Key Takeaway

Coincident constraints are about topology - defining which elements connect. They're essential for assemblies where parts must touch or join precisely.

**[Next: Equal Length →](https://github.com/snoble/slvsx-cli/blob/main/examples/10_equal_length.md)**