# Iterative Refinement Example

This example demonstrates an iterative design workflow using the `preserve` flag to build up a design step-by-step.

## What This Example Shows

- **Base structure**: Four corner points form a rectangular base, all marked with `preserve: true`
- **Roof peak**: A single point for the roof peak, not preserved, so it can adjust
- **Iterative workflow**: When you change `roof_height`, the base tries to stay fixed while the roof adjusts

## Iterative Design Workflow

1. **Start simple**: Create a basic structure (the base rectangle)
2. **Mark as preserved**: Set `preserve: true` on established features
3. **Add complexity**: Add new features (roof peak) without preserve flag
4. **Refine**: Adjust parameters - preserved features stay stable, new features adapt

## Try It Out

1. Solve the current model:
   ```bash
   slvsx solve examples/25_iterative_refinement.json
   ```

2. Change `roof_height` to `150` and solve again - notice how the base stays stable

3. Remove `preserve: true` from the base points and solve - see how everything adjusts together

## Best Practices

- **Preserve early**: Mark features as preserved once they're established
- **Preserve selectively**: Don't preserve everything - leave some flexibility
- **Use parameters**: Combine with parameters for easy iteration
- **Build incrementally**: Add features one at a time, preserving as you go

## Related Examples

- `examples/23_preserve_flag.json` - Basic preserve flag usage
- `examples/24_where_dragged.json` - Absolute locking with WHERE_DRAGGED
- `examples/21_birdhouse.json` - Complex iterative design

## Documentation

See `docs/AI_INTERACTIVE_FEATURES.md` for detailed information about interactive editing features.

