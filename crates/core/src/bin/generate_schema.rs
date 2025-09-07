use schemars::schema_for;
use slvsx_core::ir::InputDocument;
use std::fs;

fn main() {
    let schema = schema_for!(InputDocument);
    let json = serde_json::to_string_pretty(&schema).unwrap();

    // Write to schema file
    fs::create_dir_all("../../schemas").expect("Failed to create schemas directory");
    fs::write("../../schemas/slvs-json-v1-generated.schema.json", &json)
        .expect("Failed to write schema file");

    println!("Schema generated successfully!");
    println!("{}", json);
}
