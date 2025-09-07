# Radius and Circle Constraints

Control the size and position of circles and arcs.

## Circle with Fixed Radius

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "id": "center",
      "type": "Point",
      "x": 50,
      "y": 50
    },
    {
      "id": "circle1",
      "type": "Circle",
      "center": "center",
      "radius": 30
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "center"
    },
    {
      "type": "Radius",
      "entity": "circle1",
      "radius": 30
    }
  ]
}
```

## Tangent Circles

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "id": "c1_center",
      "type": "Point",
      "x": 0,
      "y": 0
    },
    {
      "id": "c2_center",
      "type": "Point",
      "x": 50,
      "y": 0
    },
    {
      "id": "circle1",
      "type": "Circle",
      "center": "c1_center",
      "radius": 20
    },
    {
      "id": "circle2",
      "type": "Circle",
      "center": "c2_center",
      "radius": 30
    }
  ],
  "constraints": [
    {
      "type": "Fixed",
      "entity": "c1_center"
    },
    {
      "type": "Radius",
      "entity": "circle1",
      "radius": 20
    },
    {
      "type": "Radius",
      "entity": "circle2",
      "radius": 30
    },
    {
      "type": "Distance",
      "entities": ["c1_center", "c2_center"],
      "distance": 50
    }
  ]
}
```

## Run the Example

```bash
slvsx solve examples/constraints/05_radius_circle.json
```