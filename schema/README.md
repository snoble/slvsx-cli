# JSON Schema for slvsx

This directory contains the JSON Schema for slvsx input documents.

## Files

- `slvs-json.schema.json` - The JSON Schema for input documents

## Generating the Schema

The schema is automatically generated from the Rust type definitions in `crates/core/src/ir.rs`.

To regenerate the schema:

```bash
# Using nix-shell (recommended)
nix-shell --run "cd crates/core && cargo run --bin generate-schema"

# Or if you have Rust installed
cd crates/core
cargo run --bin generate-schema
```

## Validating Documents

To validate JSON documents against the schema:

```bash
# Using the provided validation script
node scripts/validate-schema.js examples/*.json

# Or using any JSON Schema validator
ajv validate -s schema/slvs-json.schema.json -d your-file.json
```

## Schema Version

The current schema version is `slvs-json/1`. All input documents must include:

```json
{
  "schema": "slvs-json/1",
  "units": "mm",
  ...
}
```

## Type Definitions

The schema is generated from these Rust types:
- `InputDocument` - Root document structure
- `Entity` - Geometric entities (point, line, circle, arc, plane)
- `Constraint` - Constraints between entities
- `ExprOrNumber` - Values that can be numbers or expressions

To add new entity or constraint types, update the enums in `crates/core/src/ir.rs` and regenerate the schema.