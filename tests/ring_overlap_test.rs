use std::process::Command;

#[test]
fn test_ring_overlap_detection() {
    // This test EXPECTS the validator to detect ring overlaps
    // Planet1 is at 40mm radius, but ring is at 50mm pitch radius
    // So planet1 at 40mm + its 20mm pitch radius = 60mm > 50mm ring
    // This MUST cause an overlap
    
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
    
    // The status should be error, not ok
    assert!(
        combined.contains(r#""status": "error"#) || 
        !combined.contains(r#""status": "ok"#),
        "Solution with overlaps MUST return error status!\nOutput: {}",
        combined
    );
}

#[test]
fn test_meshing_overlap_types() {
    // Test that we detect different types of overlaps correctly
    let output = Command::new("./target/debug/slvsx")
        .args(&["solve", "testdata/ring_overlap_test.json"])
        .output()
        .expect("Failed to run slvsx");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should detect either ToothCollision or MeshingTeethOverlap
    assert!(
        stderr.contains("ToothCollision") || 
        stderr.contains("MeshingTeethOverlap") ||
        stderr.contains("InsufficientClearance"),
        "Should detect specific overlap type for ring collision"
    );
}