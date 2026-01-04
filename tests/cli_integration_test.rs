use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use std::fs;
use tempfile::NamedTempFile;

/// Test basic solve command with stdin
#[test]
fn test_solve_from_stdin() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("solve").arg("-")
        .write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"ok\""))
        .stdout(predicate::str::contains("\"p1\""));
}

/// Test validate command
#[test]
fn test_validate_valid_document() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("validate").arg("-")
        .write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("✓ Document is valid"));
}

/// Test validate with invalid JSON
#[test]
fn test_validate_invalid_json() {
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("validate").arg("-")
        .write_stdin("{invalid json}");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("JSON parsing error"));
}

/// Test validate with missing required fields
#[test]
fn test_validate_missing_fields() {
    let invalid = json!({
        "entities": []
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("validate").arg("-")
        .write_stdin(serde_json::to_string(&invalid).unwrap());

    cmd.assert()
        .failure();
}

/// Test capabilities command
#[test]
fn test_capabilities_command() {
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("capabilities");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"version\""))
        .stdout(predicate::str::contains("\"entities\""))
        .stdout(predicate::str::contains("\"constraints\""))
        .stdout(predicate::str::contains("\"export_formats\""));
}

/// Test export command
#[test]
fn test_export_to_svg() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]},
            {"type": "point", "id": "p2", "at": [100, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"},
            {"type": "distance", "between": ["p1", "p2"], "value": 100}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("export")
        .arg("-f").arg("svg")
        .arg("-");

    cmd.write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<svg"))
        .stdout(predicate::str::contains("</svg>"));
}

/// Test export with file output
#[test]
fn test_export_to_file() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"}
        ]
    });

    let tmp_file = NamedTempFile::new().unwrap();
    let output_path = tmp_file.path();

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("export")
        .arg("-f").arg("svg")
        .arg("--output").arg(output_path)
        .arg("-");

    cmd.write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert().success();

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("<svg"));
}

/// Test distance constraint solving
#[test]
fn test_distance_constraint() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "A", "at": [0, 0, 0]},
            {"type": "point", "id": "B", "at": [100, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "A"},
            {"type": "distance", "between": ["A", "B"], "value": 75.0}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("solve").arg("-")
        .write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"ok\""))
        .stdout(predicate::str::contains("\"A\""))
        .stdout(predicate::str::contains("\"B\""));
}

/// Test with parameters
#[test]
fn test_parameter_usage() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "parameters": {
            "length": 100.0
        },
        "entities": [
            {"type": "point", "id": "start", "at": [0, 0, 0]},
            {"type": "point", "id": "end", "at": [50, 30, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "start"},
            {"type": "distance", "between": ["start", "end"], "value": "$length"}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("solve").arg("-")
        .write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"ok\""));
}

/// Test error handling for non-existent file
#[test]
fn test_nonexistent_file() {
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("solve").arg("nonexistent.json");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

/// Test help command
#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("solve"))
        .stdout(predicate::str::contains("validate"))
        .stdout(predicate::str::contains("export"));
}

/// Test export to DXF format
#[test]
fn test_export_to_dxf() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]},
            {"type": "point", "id": "p2", "at": [100, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"},
            {"type": "distance", "between": ["p1", "p2"], "value": 100}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("export")
        .arg("-f").arg("dxf")
        .arg("-");

    cmd.write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success();
}

/// Test export to SLVS format
#[test]
fn test_export_to_slvs() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("export")
        .arg("-f").arg("slvs")
        .arg("-");

    cmd.write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success();
}

/// Test export to STL format
#[test]
fn test_export_to_stl() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]},
            {"type": "point", "id": "p2", "at": [100, 0, 0]},
            {"type": "point", "id": "p3", "at": [50, 50, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"},
            {"type": "distance", "between": ["p1", "p2"], "value": 100},
            {"type": "distance", "between": ["p2", "p3"], "value": 100}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("export")
        .arg("-f").arg("stl")
        .arg("-");

    cmd.write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success();
}

/// Test export with XZ view plane
#[test]
fn test_export_xz_view() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]},
            {"type": "point", "id": "p2", "at": [100, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"},
            {"type": "distance", "between": ["p1", "p2"], "value": 100}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("export")
        .arg("-f").arg("svg")
        .arg("-v").arg("xz")
        .arg("-");

    cmd.write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<svg"));
}

/// Test export with YZ view plane
#[test]
fn test_export_yz_view() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]},
            {"type": "point", "id": "p2", "at": [100, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"},
            {"type": "distance", "between": ["p1", "p2"], "value": 100}
        ]
    });

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("export")
        .arg("-f").arg("svg")
        .arg("-v").arg("yz")
        .arg("-");

    cmd.write_stdin(serde_json::to_string(&problem).unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<svg"));
}

/// Test solve with file path (not stdin)
#[test]
fn test_solve_from_file() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"}
        ]
    });

    let tmp_file = NamedTempFile::new().unwrap();
    fs::write(tmp_file.path(), serde_json::to_string(&problem).unwrap()).unwrap();

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("solve").arg(tmp_file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"ok\""));
}

/// Test validate with file path
#[test]
fn test_validate_from_file() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"}
        ]
    });

    let tmp_file = NamedTempFile::new().unwrap();
    fs::write(tmp_file.path(), serde_json::to_string(&problem).unwrap()).unwrap();

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("validate").arg(tmp_file.path());

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("✓ Document is valid"));
}

/// Test JSON error with line 0 (edge case)
#[test]
fn test_json_error_line_zero() {
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("validate").arg("-")
        .write_stdin("");

    cmd.assert()
        .failure();
}

/// Test JSON error with trailing comma
#[test]
fn test_json_error_trailing_comma() {
    let invalid = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [],
        "constraints": [],
    }); // Note: trailing comma in JSON string

    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("validate").arg("-")
        .write_stdin(serde_json::to_string(&invalid).unwrap().replace("}", ",}"));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("JSON parsing error"));
}

/// Test export with all format options
#[test]
fn test_export_all_formats() {
    let problem = json!({
        "schema": "slvs-json/1",
        "units": "mm",
        "entities": [
            {"type": "point", "id": "p1", "at": [0, 0, 0]},
            {"type": "point", "id": "p2", "at": [100, 0, 0]}
        ],
        "constraints": [
            {"type": "fixed", "entity": "p1"},
            {"type": "distance", "between": ["p1", "p2"], "value": 100}
        ]
    });

    for format in ["svg", "dxf", "slvs"] {
        let mut cmd = Command::cargo_bin("slvsx").unwrap();
        cmd.arg("export")
            .arg("-f").arg(format)
            .arg("-");

        cmd.write_stdin(serde_json::to_string(&problem).unwrap());

        cmd.assert().success();
    }
}

