#!/usr/bin/env node

/**
 * Validates JSON files against the slvs-json schema
 * 
 * Usage:
 *   node scripts/validate-schema.js examples/*.json
 *   node scripts/validate-schema.js specific-file.json
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Simple JSON Schema validator (basic implementation)
function validateAgainstSchema(data, schemaPath) {
    // For a full implementation, you would use a library like ajv
    // This is a basic check for required fields
    
    const requiredFields = ['schema', 'units'];
    const errors = [];
    
    for (const field of requiredFields) {
        if (!(field in data)) {
            errors.push(`Missing required field: ${field}`);
        }
    }
    
    if (data.schema && data.schema !== 'slvs-json/1') {
        errors.push(`Invalid schema version: ${data.schema} (expected: slvs-json/1)`);
    }
    
    if (data.units && !['mm', 'in', 'm'].includes(data.units)) {
        errors.push(`Invalid units: ${data.units} (expected: mm, in, or m)`);
    }
    
    return errors;
}

function main() {
    const args = process.argv.slice(2);
    
    if (args.length === 0) {
        console.log('Usage: node validate-schema.js <json-files...>');
        console.log('Example: node validate-schema.js examples/*.json');
        process.exit(1);
    }
    
    const schemaPath = path.join(__dirname, '..', 'schema', 'slvs-json.schema.json');
    
    if (!fs.existsSync(schemaPath)) {
        console.error(`Schema file not found: ${schemaPath}`);
        console.error('Run "cargo run --bin generate-schema" from crates/core/ to generate it');
        process.exit(1);
    }
    
    let hasErrors = false;
    
    for (const filePath of args) {
        if (!fs.existsSync(filePath)) {
            console.error(`File not found: ${filePath}`);
            hasErrors = true;
            continue;
        }
        
        try {
            const content = fs.readFileSync(filePath, 'utf8');
            const data = JSON.parse(content);
            
            const errors = validateAgainstSchema(data, schemaPath);
            
            if (errors.length > 0) {
                console.error(`❌ ${filePath}:`);
                for (const error of errors) {
                    console.error(`   - ${error}`);
                }
                hasErrors = true;
            } else {
                console.log(`✅ ${filePath}: Valid`);
            }
        } catch (e) {
            console.error(`❌ ${filePath}: ${e.message}`);
            hasErrors = true;
        }
    }
    
    process.exit(hasErrors ? 1 : 0);
}

main();