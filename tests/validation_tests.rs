use std::collections::HashMap;
use std::process::Command;

#[test]
fn test_no_gear_overlaps() {
    // Generate SVG for test configuration
    let output = Command::new("./target/debug/slvsx")
        .args(&["export", "--format", "svg", "--output", "/tmp/test_overlap.svg", "testdata/planetary_equal_spacing.json"])
        .output()
        .expect("Failed to run slvsx");
    
    assert!(output.status.success(), "slvsx export failed");
    
    // Run overlap detector
    let overlap_check = Command::new("python3")
        .args(&["tools/detect_overlaps.py", "/tmp/test_overlap.svg"])
        .output()
        .expect("Failed to run overlap detector");
    
    let output_str = String::from_utf8_lossy(&overlap_check.stdout);
    
    // Check for any overlaps
    let lines: Vec<&str> = output_str.lines().collect();
    let mut overlaps = Vec::new();
    
    for line in lines {
        if line.contains("❌") && line.contains("overlap") {
            overlaps.push(line.to_string());
        }
    }
    
    assert!(
        overlaps.is_empty(),
        "Found {} gear overlaps (MUST BE ZERO for 3D printing):\n{}",
        overlaps.len(),
        overlaps.join("\n")
    );
}

#[test]
fn test_all_gears_same_module() {
    use slvsx_core::ir::InputDocument;
    use std::fs;
    
    let json_content = fs::read_to_string("testdata/planetary_equal_spacing.json")
        .expect("Failed to read test file");
    
    let doc: InputDocument = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");
    
    // Extract all module values
    let mut modules = HashMap::new();
    
    for entity in &doc.entities {
        if let slvsx_core::ir::Entity::Gear { id, module, .. } = entity {
            // Evaluate module parameter
            let module_value = match module {
                slvsx_core::ir::ExprOrNumber::Number(n) => *n,
                slvsx_core::ir::ExprOrNumber::Expression(expr) => {
                    // For now, assume all use same parameter
                    if expr == "$module" {
                        doc.parameters.get("module")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0)
                    } else {
                        panic!("Complex expression in module: {}", expr);
                    }
                }
            };
            modules.insert(id.clone(), module_value);
        }
    }
    
    // Check all modules are the same
    let first_module = modules.values().next().expect("No gears found");
    for (gear_id, module) in &modules {
        assert_eq!(
            module, first_module,
            "Gear '{}' has module {} but expected {} (all gears must have same module)",
            gear_id, module, first_module
        );
    }
}

#[test]
fn test_assembly_constraint() {
    use slvsx_core::ir::InputDocument;
    use std::fs;
    
    let json_content = fs::read_to_string("testdata/planetary_equal_spacing.json")
        .expect("Failed to read test file");
    
    let doc: InputDocument = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");
    
    // Get parameters
    let params = &doc.parameters;
    let sun_teeth = params.get("sun_teeth")
        .and_then(|v| v.as_u64())
        .expect("Missing sun_teeth") as u32;
    let ring_teeth = params.get("ring_teeth")
        .and_then(|v| v.as_u64())
        .expect("Missing ring_teeth") as u32;
    let num_planets = params.get("num_planets")
        .and_then(|v| v.as_u64())
        .expect("Missing num_planets") as u32;
    
    // Check assembly constraint: (S + R) / n must be integer
    let assembly_value = (sun_teeth + ring_teeth) as f64 / num_planets as f64;
    assert_eq!(
        assembly_value,
        assembly_value.floor(),
        "Assembly constraint failed: ({} + {}) / {} = {} (must be integer)",
        sun_teeth, ring_teeth, num_planets, assembly_value
    );
}

#[test]
fn test_mesh_distances() {
    use slvsx_core::ir::InputDocument;
    use std::fs;
    
    let json_content = fs::read_to_string("testdata/planetary_equal_spacing.json")
        .expect("Failed to read test file");
    
    let doc: InputDocument = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");
    
    let params = &doc.parameters;
    let module = params.get("module")
        .and_then(|v| v.as_f64())
        .expect("Missing module");
    
    // Check critical mesh distances
    let sun_teeth = params.get("sun_teeth")
        .and_then(|v| v.as_u64())
        .expect("Missing sun_teeth") as f64;
    let inner_teeth = params.get("inner_teeth")
        .and_then(|v| v.as_u64())
        .expect("Missing inner_teeth") as f64;
    let outer_teeth = params.get("outer_teeth")
        .and_then(|v| v.as_u64())
        .expect("Missing outer_teeth") as f64;
    let ring_teeth = params.get("ring_teeth")
        .and_then(|v| v.as_u64())
        .expect("Missing ring_teeth") as f64;
    
    let sun_r = (sun_teeth * module) / 2.0;
    let inner_r = (inner_teeth * module) / 2.0;
    let outer_r = (outer_teeth * module) / 2.0;
    let ring_r = (ring_teeth * module) / 2.0;
    
    // Verify orbit radii
    let inner_orbit = params.get("inner_orbit")
        .and_then(|v| v.as_f64())
        .expect("Missing inner_orbit");
    let outer_orbit = params.get("outer_orbit")
        .and_then(|v| v.as_f64())
        .expect("Missing outer_orbit");
    
    let expected_inner_orbit = sun_r + inner_r;
    let expected_outer_orbit = ring_r - outer_r;
    
    assert!(
        (inner_orbit - expected_inner_orbit).abs() < 0.01,
        "Inner orbit incorrect: {} (expected {})",
        inner_orbit, expected_inner_orbit
    );
    
    assert!(
        (outer_orbit - expected_outer_orbit).abs() < 0.01,
        "Outer orbit incorrect: {} (expected {})",
        outer_orbit, expected_outer_orbit
    );
}

#[test]
fn test_minimum_clearances() {
    // This test ensures we have minimum clearances for 3D printing
    // CRITICAL: Even 0.2mm is too close and will fuse during printing!
    const MIN_CLEARANCE_MM: f64 = 0.5; // Absolute minimum for FDM printing
    const SAFE_CLEARANCE_MM: f64 = 0.8; // Safe clearance for reliable printing
    
    // Run overlap detector and check clearances
    let output = Command::new("./target/debug/slvsx")
        .args(&["export", "--format", "svg", "--output", "/tmp/test_clearance.svg", "testdata/planetary_equal_spacing.json"])
        .output()
        .expect("Failed to run slvsx");
    
    assert!(output.status.success(), "slvsx export failed");
    
    let overlap_check = Command::new("python3")
        .args(&["tools/detect_overlaps.py", "/tmp/test_clearance.svg"])
        .output()
        .expect("Failed to run overlap detector");
    
    let output_str = String::from_utf8_lossy(&overlap_check.stdout);
    
    // Parse clearances
    let mut insufficient_clearances = Vec::new();
    
    for line in output_str.lines() {
        if line.contains("✓") && line.contains("clearance") {
            // Extract clearance value
            if let Some(pos) = line.find(':') {
                let rest = &line[pos+1..];
                if let Some(mm_pos) = rest.find("mm") {
                    let clearance_str = &rest[..mm_pos].trim();
                    if let Ok(clearance) = clearance_str.parse::<f64>() {
                        if clearance < MIN_CLEARANCE_MM {
                            insufficient_clearances.push(format!("{}: {}mm (need >= {}mm)", 
                                line.split(':').next().unwrap_or("").trim(),
                                clearance, MIN_CLEARANCE_MM));
                        }
                    }
                }
            }
        }
    }
    
    assert!(
        insufficient_clearances.is_empty(),
        "Found {} gear pairs with insufficient clearance for 3D printing:\n{}",
        insufficient_clearances.len(),
        insufficient_clearances.join("\n")
    );
}

#[test]
fn test_meshing_gear_phase_alignment() {
    // Test that gears which should be meshing have proper phase alignment
    // This catches phase calculation errors that cause tooth overlap
    
    // First export to get gear data with phases
    let output = Command::new("./target/debug/slvsx")
        .args(&["export", "--format", "svg", "--output", "/tmp/test_phase.svg", "testdata/planetary_equal_spacing.json"])
        .output()
        .expect("Failed to run slvsx");
    
    assert!(output.status.success(), "slvsx export failed");
    
    // Solve to get JSON output
    let solve_output = Command::new("./target/debug/slvsx")
        .args(&["solve", "testdata/planetary_equal_spacing.json"])
        .output()
        .expect("Failed to run slvsx solve");
    
    // Extract just the JSON from output (skip debug messages)
    let output_str = String::from_utf8_lossy(&solve_output.stdout);
    let json_start = output_str.find('{').expect("No JSON in output");
    let json_data = &output_str[json_start..];
    
    // Save clean JSON for meshing detector
    std::fs::write("/tmp/test_meshing.json", json_data)
        .expect("Failed to write test JSON");
    
    // Run meshing overlap detector
    let meshing_check = Command::new("python3")
        .args(&["tools/detect_meshing_overlaps.py", "/tmp/test_meshing.json"])
        .output()
        .expect("Failed to run meshing overlap detector");
    
    let meshing_output = String::from_utf8_lossy(&meshing_check.stdout);
    let meshing_stderr = String::from_utf8_lossy(&meshing_check.stderr);
    
    assert!(
        meshing_check.status.success(),
        "Meshing gear phase alignment test failed:\n{}\n{}",
        meshing_output,
        meshing_stderr
    );
    
    // Check for phase errors
    assert!(
        !meshing_output.contains("PHASE ERROR"),
        "Phase calculation errors detected in meshing gears:\n{}",
        meshing_output
    );
}

#[test]
fn test_ring_overlap_detection() {
    // This test EXPECTS the validator to detect ring overlaps
    // The test file has a planet that overlaps with the ring
    
    let output = Command::new("./target/debug/slvsx")
        .args(&["solve", "testdata/ring_overlap_test.json"])
        .output()
        .expect("Failed to run slvsx");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}\n{}", stderr, stdout);
    
    // Check that validation FAILED
    assert!(
        combined.contains("❌ PHASE 2 VALIDATION FAILED") || 
        combined.contains("overlap"),
        "Validator MUST detect ring overlap! Planet is at wrong position for ring.\nOutput: {}",
        combined
    );
    
    // Check that it detected the specific ring overlap
    assert!(
        combined.contains("ring") && (
            combined.contains("overlap") || 
            combined.contains("Overlap") ||
            combined.contains("collision")
        ),
        "Validator MUST specifically detect ring overlap!\nOutput: {}",
        combined
    );
}