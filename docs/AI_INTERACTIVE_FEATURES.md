# Interactive Editing Features for AI Agents

SolveSpace has powerful interactive editing features that can be leveraged by AI agents to create more intelligent, iterative design workflows. These features go beyond static JSON syntax - they enable **solving behavior** that preserves user intent and enables incremental refinement.

## Core Concepts

### 1. **Dragged Parameters**

**What it does**: Marks parameters that should change **minimally** during solving. The solver will favor changing other parameters even if it means they change more, to preserve the dragged parameters' values.

**Why it's useful for AI**:
- **Incremental editing**: When modifying a model, preserve certain aspects that shouldn't change
- **Iterative refinement**: Build up a design step-by-step, keeping established features fixed
- **Constraint exploration**: Try different constraint values while preserving key design elements
- **What-if scenarios**: Explore design variations while keeping certain features constant

**Example use case**: An AI is refining a birdhouse design. It wants to adjust the roof angle but preserve the base dimensions. By marking the base points as "dragged", the solver will try to keep them fixed while adjusting the roof.

### 2. **WHERE_DRAGGED Constraint**

**What it does**: Locks a point to its current position. More aggressive than dragged parameters - actually constrains the point to stay exactly where it is.

**Why it's useful for AI**:
- **Anchor points**: Mark certain points as absolutely fixed during exploration
- **Reference geometry**: Keep construction geometry fixed while modifying the main design
- **Staged solving**: Solve in stages, locking earlier stages before moving to next

## Proposed API

### Option 1: Entity-Level Metadata

Add a `preserve` field to entities that marks them for minimal change:

```json
{
  "entities": [
    {
      "type": "point",
      "id": "base_corner",
      "at": [0, 0, 0],
      "preserve": true  // Mark as dragged - minimize changes
    },
    {
      "type": "point",
      "id": "roof_peak",
      "at": [50, 50, 100],
      "preserve": false  // Can change freely
    }
  ]
}
```

### Option 2: Explicit Dragged List

Add a top-level `dragged` array that lists entity IDs to preserve:

```json
{
  "entities": [...],
  "constraints": [...],
  "dragged": ["base_corner", "base_width"],  // These entities' parameters are dragged
  "locked": ["reference_point"]  // These are WHERE_DRAGGED (absolutely fixed)
}
```

### Option 3: Solving Mode Flag

Add a solving mode that specifies behavior:

```json
{
  "entities": [...],
  "constraints": [...],
  "solve_options": {
    "mode": "incremental",  // or "full", "preserve_dragged"
    "preserve": ["base_corner", "base_width"],
    "lock": ["reference_point"]
  }
}
```

### Option 4: Separate Command/Workflow

Instead of JSON syntax, use CLI commands for iterative workflows:

```bash
# Initial solve
slvsx solve model.json

# Mark entities to preserve
slvsx preserve model.json --entities base_corner base_width

# Modify constraints
slvsx solve model.json --preserve  # Uses preserved entities

# Or use a session file
slvsx solve model.json --session session.json
```

Where `session.json` tracks what's preserved:
```json
{
  "preserved_entities": ["base_corner", "base_width"],
  "locked_entities": ["reference_point"]
}
```

## Examples

See the following examples for practical usage:

- **`examples/23_preserve_flag.json`** - Basic preserve flag usage
- **`examples/24_where_dragged.json`** - WHERE_DRAGGED constraint for absolute locking
- **`examples/25_iterative_refinement.json`** - Iterative design workflow

## AI Agent Workflows

### Workflow 1: Iterative Refinement

**Scenario**: AI is refining a design step-by-step.

1. **Initial solve**: Create base structure, solve
2. **Mark as preserved**: Mark base structure entities as `preserve: true`
3. **Add refinement**: Add new constraints/entities
4. **Solve**: Solver preserves base while adjusting new parts
5. **Repeat**: Continue refining while preserving established features

**Example**:
```json
// Step 1: Base structure
{
  "entities": [
    {"type": "point", "id": "p1", "at": [0, 0, 0], "preserve": false},
    {"type": "point", "id": "p2", "at": [100, 0, 0], "preserve": false}
  ],
  "constraints": [
    {"type": "fixed", "entity": "p1"},
    {"type": "distance", "between": ["p1", "p2"], "value": 100}
  ]
}

// Step 2: After solving, mark base as preserved, add roof
{
  "entities": [
    {"type": "point", "id": "p1", "at": [0, 0, 0], "preserve": true},  // Preserve!
    {"type": "point", "id": "p2", "at": [100, 0, 0], "preserve": true},  // Preserve!
    {"type": "point", "id": "roof", "at": [50, 50, 50], "preserve": false}
  ],
  "constraints": [
    {"type": "fixed", "entity": "p1"},
    {"type": "distance", "between": ["p1", "p2"], "value": 100},
    {"type": "distance", "between": ["p1", "roof"], "value": 70.7}
  ]
}
```

### Workflow 2: Constraint Exploration

**Scenario**: AI wants to explore different constraint values while keeping design stable.

1. **Establish base**: Solve initial design
2. **Mark key features**: Mark important entities as preserved
3. **Vary constraints**: Try different constraint values
4. **Observe changes**: See how design adapts while preserving key features

**Example**:
```json
{
  "entities": [
    {"type": "point", "id": "corner1", "at": [0, 0, 0], "preserve": true},
    {"type": "point", "id": "corner2", "at": [100, 0, 0], "preserve": true},
    {"type": "point", "id": "corner3", "at": [100, 100, 0], "preserve": true},
    {"type": "point", "id": "corner4", "at": [0, 100, 0], "preserve": false}  // Can adjust
  ],
  "constraints": [
    {"type": "fixed", "entity": "corner1"},
    {"type": "distance", "between": ["corner1", "corner2"], "value": 100},
    {"type": "distance", "between": ["corner2", "corner3"], "value": 100},
    {"type": "distance", "between": ["corner3", "corner4"], "value": "$width"},  // Vary this
    {"type": "distance", "between": ["corner4", "corner1"], "value": 100}
  ],
  "parameters": {
    "width": 100  // Try different values: 80, 120, 150...
  }
}
```

### Workflow 3: Staged Solving

**Scenario**: Complex design built in stages, each stage locked before next.

1. **Stage 1**: Solve base structure, mark as `locked`
2. **Stage 2**: Add walls, solve, mark as `locked`
3. **Stage 3**: Add roof, solve

**Example**:
```json
// Stage 1: Base
{
  "entities": [
    {"type": "point", "id": "base1", "at": [0, 0, 0]},
    {"type": "point", "id": "base2", "at": [100, 0, 0]}
  ],
  "constraints": [...],
  "locked": ["base1", "base2"]  // Absolutely fixed
}

// Stage 2: Add walls (base1, base2 are locked)
{
  "entities": [
    {"type": "point", "id": "base1", "at": [0, 0, 0]},  // Locked from stage 1
    {"type": "point", "id": "base2", "at": [100, 0, 0]},  // Locked from stage 1
    {"type": "point", "id": "wall_top1", "at": [0, 0, 50]},
    {"type": "point", "id": "wall_top2", "at": [100, 0, 50]}
  ],
  "constraints": [...],
  "locked": ["base1", "base2", "wall_top1", "wall_top2"]  // Lock stage 2
}
```

## Implementation Approach

### Phase 1: Basic Dragged Parameters

1. Add `preserve` boolean field to entities (default: `false`)
2. Map `preserve: true` to dragged parameters in FFI
3. Update solver to mark dragged parameters before solving
4. Add tests

### Phase 2: WHERE_DRAGGED Constraint

1. Add `Dragged` constraint type to IR
2. Add FFI binding for `SLVS_C_WHERE_DRAGGED`
3. Update constraint registry
4. Add tests

### Phase 3: Advanced Workflows

1. Add `locked` array to document (for WHERE_DRAGGED)
2. Add `solve_options` for more control
3. Consider session files for iterative workflows
4. Add CLI commands for interactive workflows

## Benefits for AI Agents

1. **Smarter Iteration**: AI can refine designs without losing established features
2. **Exploration**: Try different constraint values while preserving key aspects
3. **Stability**: More predictable solving behavior
4. **Efficiency**: Don't need to rebuild entire model for small changes
5. **User Intent**: Better preservation of design intent during modifications

## Example: AI Refining Birdhouse

```json
{
  "entities": [
    // Base - preserve these
    {"type": "point", "id": "base_fl", "at": [0, 0, 0], "preserve": true},
    {"type": "point", "id": "base_fr", "at": [152, 0, 0], "preserve": true},
    {"type": "point", "id": "base_bl", "at": [0, 152, 0], "preserve": true},
    {"type": "point", "id": "base_br", "at": [152, 152, 0], "preserve": true},
    
    // Roof - can adjust
    {"type": "point", "id": "roof_peak", "at": [76, 76, 100], "preserve": false}
  ],
  "constraints": [
    {"type": "fixed", "entity": "base_fl"},
    {"type": "distance", "between": ["base_fl", "base_fr"], "value": 152},
    {"type": "distance", "between": ["base_fr", "base_br"], "value": 152},
    {"type": "distance", "between": ["base_bl", "base_br"], "value": 152},
    {"type": "distance", "between": ["base_fl", "base_bl"], "value": 152},
    
    // Adjust roof height - base stays fixed!
    {"type": "distance", "between": ["base_fl", "roof_peak"], "value": "$roof_height"}
  ],
  "parameters": {
    "roof_height": 120  // Try different values: 100, 140, 160...
  }
}
```

When `roof_height` changes, the base corners stay fixed (because `preserve: true`), and only the roof adjusts.

## Next Steps

1. **Choose API approach**: Entity-level `preserve` field seems simplest to start
2. **Implement dragged parameters**: Map to SolveSpace's `dragged` parameter set
3. **Add WHERE_DRAGGED constraint**: For absolute locking
4. **Document workflows**: Show AI agents how to use these features
5. **Consider CLI commands**: For more advanced iterative workflows

This is a **workflow feature**, not just syntax - it enables AI agents to work more intelligently with geometric models.

