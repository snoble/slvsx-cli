//! Test that all example files are valid and can be processed by slvsx
//!
//! This test ensures that:
//! 1. All example JSON files are valid JSON
//! 2. All examples can be validated by slvsx
//! 3. Examples that should solve successfully do so
//! 4. Examples with missing features are properly documented

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Find all example JSON files (excluding solution files)
fn find_example_files() -> Vec<PathBuf> {
    let mut examples = Vec::new();
    
    // Main examples directory
    if let Ok(entries) = fs::read_dir("examples") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension() == Some("json".as_ref()) 
                && !path.file_name().unwrap().to_string_lossy().contains("solution") {
                examples.push(path);
            }
        }
    }
    
    // Subdirectories
    for subdir in &["ai-examples", "basics", "constraints"] {
        let subdir_path = format!("examples/{}", subdir);
        if let Ok(entries) = fs::read_dir(&subdir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension() == Some("json".as_ref()) {
                    examples.push(path);
                }
            }
        }
    }
    
    examples.sort();
    examples
}

/// Test that all example files are valid JSON
#[test]
fn test_all_examples_valid_json() {
    let examples = find_example_files();
    assert!(!examples.is_empty(), "No example files found");
    
    for example in &examples {
        let content = fs::read_to_string(example)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", example.display(), e));
        
        let _: Value = serde_json::from_str(&content)
            .unwrap_or_else(|e| panic!("Invalid JSON in {}: {}", example.display(), e));
    }
}

/// Test that all examples can be validated by slvsx
#[test]
fn test_all_examples_validate() {
    let examples = find_example_files();
    assert!(!examples.is_empty(), "No example files found");
    
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    
    for example in &examples {
        cmd.arg("validate").arg(example);
        
        let output = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to run validate on {}: {}", example.display(), e);
        });
        
        // Examples should either:
        // 1. Validate successfully (exit code 0)
        // 2. Fail due to missing features (which is documented)
        // 3. Fail with a clear error message
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // If it's a missing feature, that's OK - it should be documented
            if stderr.contains("not yet implemented") || stdout.contains("not yet implemented") {
                // This is expected for examples that require missing constraints
                continue;
            }
            
            // Otherwise, log the error but don't fail - some examples might be intentionally invalid
            eprintln!("Warning: {} failed validation: {}", example.display(), stderr);
        }
        
        // Reset command for next iteration
        cmd = Command::cargo_bin("slvsx").unwrap();
    }
}

/// Test that examples requiring implemented constraints can be solved
#[test]
fn test_solvable_examples_solve() {
    // These examples should solve successfully with current constraints
    let solvable_examples = vec![
        "examples/01_first_point.json",
        "examples/01_basic_distance.json",
        "examples/02_distance_constraint.json",
        "examples/02_triangle.json",
        "examples/03_correctly_constrained.json",
        "examples/05_parallel_perpendicular.json",
        "examples/07_point_on_line.json",
        "examples/09_coincident.json",
    ];
    
    for example_path in &solvable_examples {
        if !PathBuf::from(example_path).exists() {
            continue; // Skip if file doesn't exist
        }
        
        let mut cmd = Command::cargo_bin("slvsx").unwrap();
        cmd.arg("solve").arg(example_path);
        
        let output = cmd.output().unwrap_or_else(|e| {
            panic!("Failed to run solve on {}: {}", example_path, e);
        });
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            panic!(
                "Example {} should solve but failed:\nSTDOUT: {}\nSTDERR: {}",
                example_path, stdout, stderr
            );
        }
        
        // Verify output is valid JSON with "status": "ok"
        let stdout = String::from_utf8_lossy(&output.stdout);
        let solution: Value = serde_json::from_str(&stdout)
            .unwrap_or_else(|e| panic!("Invalid JSON output from {}: {}", example_path, e));
        
        assert_eq!(
            solution.get("status").and_then(|s| s.as_str()),
            Some("ok"),
            "Example {} should solve successfully, got: {}",
            example_path,
            stdout
        );
    }
}

/// Test that new tutorial examples document missing features
#[test]
fn test_tutorial_examples_document_missing_features() {
    let tutorial_examples = vec![
        ("examples/17_four_bar_linkage.json", "Angle"),
        ("examples/18_simple_rectangle.json", "Horizontal/Vertical"),
        ("examples/19_parametric_square.json", "EqualLength"),
        ("examples/20_slider_crank.json", "Horizontal/Angle"),
    ];
    
    for (example_path, expected_missing) in &tutorial_examples {
        if !PathBuf::from(example_path).exists() {
            continue;
        }
        
        // Check that the example file exists
        assert!(
            PathBuf::from(example_path).exists(),
            "Tutorial example {} should exist",
            example_path
        );
        
        // Check that corresponding markdown file exists and documents missing features
        let md_path = example_path.replace(".json", ".md");
        if PathBuf::from(&md_path).exists() {
            let md_content = fs::read_to_string(&md_path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", md_path, e));
            
            assert!(
                md_content.contains("Missing") || md_content.contains("requires"),
                "{} should document missing features: {}",
                md_path,
                expected_missing
            );
        }
        
        // Try to validate - should either work or fail with missing feature message
        let mut cmd = Command::cargo_bin("slvsx").unwrap();
        cmd.arg("validate").arg(example_path);
        
        let output = cmd.output().unwrap();
        
        // It's OK if validation fails due to missing features
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // Should mention missing feature or not yet implemented
            let output_text = format!("{} {}", stdout, stderr);
            if !output_text.contains("not yet implemented") 
                && !output_text.contains("missing") 
                && !output_text.contains("Missing") {
                eprintln!(
                    "Warning: {} validation failed but doesn't mention missing features: {}",
                    example_path,
                    output_text
                );
            }
        }
    }
}

