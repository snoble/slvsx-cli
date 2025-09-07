use std::collections::HashMap;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_solve_and_export() {
    // Build the project first
    let output = Command::new("cargo")
        .args(&["build", "--features", "mock-solver"])
        .output()
        .expect("Failed to build project");
    
    if !output.status.success() {
        panic!("Build failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Test solve command
    let output = Command::new("./target/debug/slvsx")
        .args(&["solve", "testdata/planetary_gear_constraints.json"])
        .output()
        .expect("Failed to run solve command");
    
    assert!(output.status.success(), "Solve command failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"status\": \"ok\""));
    
    // Test export to SVG
    let tmp_dir = tempdir().unwrap();
    let svg_path = tmp_dir.path().join("test.svg");
    
    let output = Command::new("./target/debug/slvsx")
        .args(&[
            "export",
            "--format", "svg",
            "--output", svg_path.to_str().unwrap(),
            "testdata/planetary_gear_constraints.json"
        ])
        .output()
        .expect("Failed to run export command");
    
    assert!(output.status.success(), "Export command failed");
    assert!(svg_path.exists(), "SVG file was not created");
    
    // Read and validate SVG content
    let svg_content = std::fs::read_to_string(&svg_path).unwrap();
    assert!(svg_content.contains("<svg"));
    assert!(svg_content.contains("</svg>"));
    assert!(svg_content.contains("<circle"));
    
    // Verify circles have non-zero radius
    let lines: Vec<&str> = svg_content.lines().collect();
    for line in lines {
        if line.contains("<circle") && line.contains("r=") {
            // Extract radius value
            if let Some(r_start) = line.find("r=\"") {
                let r_str = &line[r_start + 3..];
                if let Some(r_end) = r_str.find("\"") {
                    let radius: f64 = r_str[..r_end].parse().unwrap_or(0.0);
                    assert!(radius > 0.0, "Circle has zero radius in SVG: {}", line);
                }
            }
        }
    }
}

#[test]
fn test_solver_returns_entities() {
    use slvsx_core::{
        solver::{Solver, SolverConfig},
        ir::{InputDocument, Entity, ExprOrNumber},
    };
    
    let solver = Solver::new(SolverConfig::default());
    let mut params = HashMap::new();
    params.insert("gear_diameter".to_string(), 60.0);
    
    let doc = InputDocument {
        schema: "slvs-json/1".to_string(),
        units: "mm".to_string(),
        parameters: params,
        entities: vec![
            Entity::Circle {
                id: "test_circle".to_string(),
                center: vec![
                    ExprOrNumber::Number(0.0),
                    ExprOrNumber::Number(0.0),
                ],
                diameter: ExprOrNumber::Expression("$gear_diameter".to_string()),
            },
        ],
        constraints: vec![],
    };
    
    let result = solver.solve(&doc).expect("Solver failed");
    assert_eq!(result.status, "ok");
    
    let entities = result.entities.expect("No entities returned");
    assert!(!entities.is_empty(), "Entities map is empty");
    
    assert!(entities.contains_key("test_circle"), "test_circle not found in results");
    
    if let Some(slvsx_core::ir::ResolvedEntity::Circle { diameter, .. }) = entities.get("test_circle") {
        assert_eq!(*diameter, 60.0, "Diameter not correctly resolved");
    } else {
        panic!("test_circle is not a Circle");
    }
}