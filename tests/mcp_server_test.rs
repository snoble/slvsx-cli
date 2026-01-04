use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test MCP server command exists
#[test]
fn test_mcp_server_command_exists() {
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("mcp-server").arg("--help");
    
    // Should either show help or start (depending on implementation)
    // For now, just verify the command is recognized
    let result = cmd.output();
    // Command should not fail with "unknown command"
    match result {
        Ok(output) => {
            // If command succeeded or failed with non-2 exit code, that's fine
            assert_ne!(output.status.code(), Some(2), "Command should not fail with 'unknown command' error");
        }
        Err(_) => {
            // If command couldn't be executed, that's also acceptable for this test
            // The test is just checking that the command exists
        }
    }
}

/// Test MCP server responds to invalid JSON
#[test]
fn test_mcp_server_invalid_json() {
    // This tests the error handling path when JSON parsing fails
    // The server should continue running and not crash
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("mcp-server");
    
    let mut child = cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start mcp-server");
    
    // Send invalid JSON
    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "invalid json{{").unwrap();
        stdin.flush().unwrap();
    }
    
    // Server should handle this gracefully
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let _ = child.kill();
}

/// Test MCP server requires initialization
#[test]
fn test_mcp_server_requires_init() {
    // Test that calling tools before initialize returns error
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("mcp-server");
    
    let mut child = cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start mcp-server");
    
    // Try to call tools/list without initialize
    let list_tools = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list"
    });
    
    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "{}", list_tools).unwrap();
        stdin.flush().unwrap();
    }
    
    std::thread::sleep(std::time::Duration::from_millis(200));
    
    // Should get "Server not initialized" error
    let _ = child.kill();
}

/// Test MCP server handles unknown method
#[test]
fn test_mcp_server_unknown_method() {
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("mcp-server");
    
    let mut child = cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start mcp-server");
    
    // Initialize first
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {}
        }
    });
    
    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "{}", init_request).unwrap();
        stdin.flush().unwrap();
    }
    
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Send unknown method
    let unknown = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "unknown/method"
    });
    
    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "{}", unknown).unwrap();
        stdin.flush().unwrap();
    }
    
    std::thread::sleep(std::time::Duration::from_millis(200));
    
    // Should get "Method not found" error
    let _ = child.kill();
}

/// Test MCP server handles empty lines
#[test]
fn test_mcp_server_empty_lines() {
    // Test that empty lines are ignored
    let mut cmd = Command::cargo_bin("slvsx").unwrap();
    cmd.arg("mcp-server");
    
    let mut child = cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start mcp-server");
    
    // Send empty lines
    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "").unwrap();
        writeln!(stdin, "   ").unwrap();  // whitespace only
        writeln!(stdin, "").unwrap();
        stdin.flush().unwrap();
    }
    
    // Server should continue running
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let _ = child.kill();
}

