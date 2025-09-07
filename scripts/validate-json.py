#!/usr/bin/env python3
"""
Simple JSON schema validation script for SLVS constraint documents
"""
import json
import sys
from pathlib import Path
import jsonschema

def validate_document(json_file, schema_file):
    """Validate a JSON document against the schema"""
    try:
        # Load schema
        with open(schema_file, 'r') as f:
            schema = json.load(f)
        
        # Load document
        with open(json_file, 'r') as f:
            document = json.load(f)
        
        # Validate
        jsonschema.validate(document, schema)
        print(f"✓ {json_file} is valid")
        return True
        
    except jsonschema.ValidationError as e:
        print(f"✗ {json_file} validation failed:")
        print(f"  {e.message}")
        if e.path:
            print(f"  at path: {' -> '.join(str(p) for p in e.path)}")
        return False
    except Exception as e:
        print(f"✗ Error validating {json_file}: {e}")
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: validate-json.py <json-file> [schema-file]")
        sys.exit(1)
    
    json_file = sys.argv[1]
    schema_file = sys.argv[2] if len(sys.argv) > 2 else "schemas/slvs-json-v1-generated.schema.json"
    
    success = validate_document(json_file, schema_file)
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()