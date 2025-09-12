#!/usr/bin/env python3
"""
MCP Server wrapper for slvsx geometry solver
Provides a simple interface for AI agents to solve geometric constraints
"""

import json
import subprocess
import sys
import os
from typing import Dict, Any, Optional

class SlvsxMCPServer:
    def __init__(self, binary_path: str = None):
        """Initialize the MCP server with the slvsx binary path"""
        if binary_path and os.path.exists(binary_path):
            self.binary_path = binary_path
        else:
            # Try to find the binary
            possible_paths = [
                "./target/release/slvsx",
                "./target/x86_64-unknown-linux-gnu/release/slvsx",
                "./target/debug/slvsx",
                "/usr/local/bin/slvsx",
                "./slvsx"
            ]
            for path in possible_paths:
                if os.path.exists(path):
                    self.binary_path = path
                    break
            else:
                raise FileNotFoundError("slvsx binary not found. Please build the project first.")
        
        print(f"Using slvsx binary at: {self.binary_path}", file=sys.stderr)
    
    def solve_geometry(self, problem: Dict[str, Any]) -> Dict[str, Any]:
        """
        Solve a geometry problem using slvsx
        
        Args:
            problem: A dictionary containing the geometry problem in slvs-json format
                    Must include: entities, constraints, units
        
        Returns:
            Solution dictionary with solved positions
        """
        # Ensure required fields
        if "schema" not in problem:
            problem["schema"] = "slvs-json/1"
        if "units" not in problem:
            problem["units"] = "mm"
        
        # Write problem to temp file or use stdin
        problem_json = json.dumps(problem)
        
        try:
            # Run slvsx solver
            result = subprocess.run(
                [self.binary_path, "solve", "-"],
                input=problem_json,
                capture_output=True,
                text=True,
                check=True
            )
            
            # Parse the output
            solution = json.loads(result.stdout)
            return {
                "status": "success",
                "solution": solution
            }
        except subprocess.CalledProcessError as e:
            return {
                "status": "error",
                "error": e.stderr or str(e)
            }
        except json.JSONDecodeError as e:
            return {
                "status": "error",
                "error": f"Failed to parse solution: {e}"
            }
    
    def export_svg(self, problem: Dict[str, Any]) -> str:
        """Export a geometry problem to SVG"""
        problem_json = json.dumps(problem)
        
        try:
            result = subprocess.run(
                [self.binary_path, "export", "-f", "svg", "-"],
                input=problem_json,
                capture_output=True,
                text=True,
                check=True
            )
            return result.stdout
        except subprocess.CalledProcessError as e:
            raise RuntimeError(f"Failed to export SVG: {e.stderr}")
    
    def validate(self, problem: Dict[str, Any]) -> Dict[str, Any]:
        """Validate a geometry problem"""
        problem_json = json.dumps(problem)
        
        try:
            result = subprocess.run(
                [self.binary_path, "validate", "-"],
                input=problem_json,
                capture_output=True,
                text=True,
                check=True
            )
            return {"status": "valid", "message": result.stdout}
        except subprocess.CalledProcessError as e:
            return {"status": "invalid", "error": e.stderr}

def handle_request(request: Dict[str, Any], server: SlvsxMCPServer) -> Dict[str, Any]:
    """Handle an MCP request"""
    method = request.get("method", "")
    params = request.get("params", {})
    
    if method == "solve":
        return server.solve_geometry(params)
    elif method == "export_svg":
        svg = server.export_svg(params)
        return {"svg": svg}
    elif method == "validate":
        return server.validate(params)
    elif method == "capabilities":
        return {
            "methods": ["solve", "export_svg", "validate"],
            "description": "Geometric constraint solver using SolveSpace",
            "version": "1.0.0"
        }
    else:
        return {"error": f"Unknown method: {method}"}

def main():
    """Main entry point for MCP server mode"""
    server = SlvsxMCPServer()
    
    # Simple JSON-RPC style interface over stdin/stdout
    print("SlvsX MCP Server ready", file=sys.stderr)
    
    for line in sys.stdin:
        try:
            request = json.loads(line)
            response = handle_request(request, server)
            print(json.dumps(response))
            sys.stdout.flush()
        except Exception as e:
            error_response = {"error": str(e)}
            print(json.dumps(error_response))
            sys.stdout.flush()

if __name__ == "__main__":
    main()