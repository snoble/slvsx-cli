# JSON Schema Generation

## Overview

The slvsx-cli project uses JSON for input/output format definition. The JSON schema is automatically generated from the Rust type definitions to ensure consistency between the code and the schema.

## Type Definitions

The main types are defined in `crates/core/src/ir.rs`:
- `InputDocument` - The root document structure
- `Entity` - Geometric entities (points, lines, circles, etc.)
- `Constraint` - Constraints between entities
- `SolveResult` - The solver output format

All these types use the `schemars::JsonSchema` derive macro to enable automatic schema generation.

## Schema Generation

### Prerequisites
- Rust toolchain with cargo installed
- Working directory: project root

### Generate Schema

Run the following command from the project root:

```bash
cd crates/core
cargo run --bin generate-schema
```

This will:
1. Generate the JSON schema from the Rust types
2. Write it to `schema/slvs-json.schema.json`
3. Print the generated schema to stdout

### Schema Generator Source

The schema generator is located at:
`crates/core/src/bin/generate_schema.rs`

It uses the `schemars` crate to generate a JSON Schema from the `InputDocument` type and all its nested types.

## Manual Updates

If you need to manually update the schema:

1. **DO NOT** edit `schema/slvs-json.schema.json` directly
2. Instead, update the type definitions in `crates/core/src/ir.rs`
3. Run the schema generator to update the schema file
4. Commit both the type changes and the generated schema

## Current Schema vs Generated Schema

The current manually-maintained schema at `schema/slvs-json.schema.json` includes some entity types that are not yet in the Rust implementation:
- Gear entities
- Mesh constraints

These should be added to the Rust types in `ir.rs` when implementing gear support.

## Adding New Types

To add new entity or constraint types:

1. Add the type variant to the appropriate enum in `ir.rs`
2. Ensure all fields have appropriate `serde` attributes
3. Add `schemars` attributes if needed for validation (e.g., regex patterns, min/max values)
4. Run the schema generator
5. Update examples and tests

## Schema Validation

The generated schema can be validated against example files using any JSON Schema validator:

```bash
# Example using ajv-cli (npm install -g ajv-cli)
ajv validate -s schema/slvs-json.schema.json -d examples/*.json
```

## Notes

- The schema version is currently hardcoded as "slvs-json/1"
- Default units are "mm" if not specified
- The `schemars` crate handles complex types like enums with `#[serde(tag)]` correctly