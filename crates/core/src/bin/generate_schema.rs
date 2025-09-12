use schemars::schema_for;
use slvsx_core::ir::InputDocument;
use std::fs;

fn main() {
    let schema = schema_for!(InputDocument);
    let json = serde_json::to_string_pretty(&schema).unwrap();

    // Write to schema file
    fs::create_dir_all("schema").expect("Failed to create schema directory");
    fs::write("schema/slvs-json.schema.json", &json)
        .expect("Failed to write schema file");

    println!("Schema generated successfully at schema/slvs-json.schema.json");
    println!("{}", json);
}
