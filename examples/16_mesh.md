# Example 16: Mesh Constraint (Gears)

**[← Equal Radius](15_equal_radius.md)** | **[Next: Complex Mechanisms →](17_complex_mechanisms.md)**

## The Story

Gears are the heart of mechanical power transmission. The mesh constraint ensures gears engage properly, with teeth interlocking at the correct phase angles. This is critical for 3D printing and manufacturing - overlapping teeth would cause jamming or printing failures!

Let's create a simple gear pair with proper meshing.

## The Entities

We'll create:
1. A sun gear (driver)
2. A planet gear (driven)
3. Mesh constraint to ensure proper engagement

## The JSON

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  "entities": [
    {
      "type": "point",
      "id": "sun_center",
      "at": [100, 100, 0]
    },
    {
      "type": "gear",
      "id": "sun",
      "center": [100, 100, 0],
      "teeth": 30,
      "module": 2.0,
      "pressure_angle": 20.0,
      "rotation": 0
    },
    {
      "type": "point",
      "id": "planet_center",
      "at": [145, 100, 0]
    },
    {
      "type": "gear",
      "id": "planet",
      "center": [145, 100, 0],
      "teeth": 15,
      "module": 2.0,
      "pressure_angle": 20.0,
      "rotation": 0
    }
  ],
  "constraints": [
    {
      "type": "fixed",
      "entity": "sun_center"
    },
    {
      "type": "distance",
      "between": ["sun_center", "planet_center"],
      "value": 45,
      "_comment": "Center distance = module * (teeth1 + teeth2) / 2 = 2 * (30 + 15) / 2 = 45"
    },
    {
      "type": "mesh",
      "gear1": "sun",
      "gear2": "planet"
    }
  ]
}
```

## Understanding the Code

- **Gear entity**: Needs teeth count, module, pressure angle
- **Module**: Gear size parameter (pitch diameter = teeth × module)
- **Pressure angle**: Standard is 20° for most gears
- **Center distance formula**: `module × (teeth1 + teeth2) / 2`
- **Mesh constraint**: Automatically calculates phase angles

## The Solution

```json
{
  "status": "ok",
  "entities": {
    "sun": {
      "center": [100.0, 100.0, 0.0],
      "teeth": 30,
      "module": 2.0,
      "pressure_angle": 20.0,
      "phase": 0.0
    },
    "planet": {
      "center": [145.0, 100.0, 0.0],
      "teeth": 15,
      "module": 2.0,
      "pressure_angle": 20.0,
      "phase": 12.0
    }
  }
}
```

Notice the phase angles! The planet gear has phase = 12°, ensuring teeth don't collide.

## Phase Calculation

The solver automatically calculates phase angles to prevent tooth collision:
- Reference gear (sun): phase = 0°
- Meshing gear (planet): phase calculated for proper engagement
- This ensures teeth interlock correctly for 3D printing

## Gear Train Applications

- **Speed reduction**: Large driving small
- **Speed increase**: Small driving large
- **Direction reversal**: Add idler gear
- **Planetary systems**: Multiple planets around sun

## Key Takeaway

The mesh constraint is essential for functional gear systems. It automatically handles the complex phase calculations needed for proper tooth engagement, making it safe to 3D print or manufacture your gear designs.

**[Next: Complex Mechanisms →](17_complex_mechanisms.md)**