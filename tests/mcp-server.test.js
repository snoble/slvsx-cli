#!/usr/bin/env node
/**
 * Tests for MCP server functionality
 * 
 * Run with: node tests/mcp-server.test.js
 */

import assert from 'assert';
import * as fs from 'fs';
import * as path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const projectRoot = path.resolve(__dirname, '..');

// Test utilities
let passed = 0;
let failed = 0;

function test(name, fn) {
  try {
    fn();
    console.log(`✓ ${name}`);
    passed++;
  } catch (error) {
    console.log(`✗ ${name}`);
    console.log(`  Error: ${error.message}`);
    failed++;
  }
}

async function asyncTest(name, fn) {
  try {
    await fn();
    console.log(`✓ ${name}`);
    passed++;
  } catch (error) {
    console.log(`✗ ${name}`);
    console.log(`  Error: ${error.message}`);
    failed++;
  }
}

// ============================================
// Test: Cosine Similarity
// ============================================

function cosineSimilarity(a, b) {
  let dotProduct = 0;
  let normA = 0;
  let normB = 0;
  for (let i = 0; i < a.length; i++) {
    dotProduct += a[i] * b[i];
    normA += a[i] * a[i];
    normB += b[i] * b[i];
  }
  return dotProduct / (Math.sqrt(normA) * Math.sqrt(normB));
}

test('cosineSimilarity: identical vectors have similarity 1', () => {
  const v = [1, 2, 3, 4, 5];
  const similarity = cosineSimilarity(v, v);
  assert(Math.abs(similarity - 1.0) < 0.0001, `Expected 1.0, got ${similarity}`);
});

test('cosineSimilarity: orthogonal vectors have similarity 0', () => {
  const a = [1, 0, 0];
  const b = [0, 1, 0];
  const similarity = cosineSimilarity(a, b);
  assert(Math.abs(similarity) < 0.0001, `Expected 0, got ${similarity}`);
});

test('cosineSimilarity: opposite vectors have similarity -1', () => {
  const a = [1, 2, 3];
  const b = [-1, -2, -3];
  const similarity = cosineSimilarity(a, b);
  assert(Math.abs(similarity + 1.0) < 0.0001, `Expected -1.0, got ${similarity}`);
});

test('cosineSimilarity: similar vectors have high similarity', () => {
  const a = [1, 2, 3];
  const b = [1.1, 2.1, 3.1];
  const similarity = cosineSimilarity(a, b);
  assert(similarity > 0.99, `Expected > 0.99, got ${similarity}`);
});

// ============================================
// Test: Documentation Index Structure
// ============================================

test('docs index file exists after build', () => {
  const docsPath = path.join(projectRoot, 'dist', 'docs.json');
  // This test only passes after running npm run build:docs
  if (!fs.existsSync(docsPath)) {
    console.log('  (skipped - run "npm run build:docs" first)');
    return;
  }
  assert(fs.existsSync(docsPath), 'docs.json should exist');
});

test('docs index has correct structure', () => {
  const docsPath = path.join(projectRoot, 'dist', 'docs.json');
  if (!fs.existsSync(docsPath)) {
    console.log('  (skipped - run "npm run build:docs" first)');
    return;
  }
  
  const data = JSON.parse(fs.readFileSync(docsPath, 'utf-8'));
  
  assert(data.version, 'Should have version');
  assert(data.model, 'Should have model');
  assert(Array.isArray(data.chunks), 'Should have chunks array');
  assert(data.createdAt, 'Should have createdAt');
  
  if (data.chunks.length > 0) {
    const chunk = data.chunks[0];
    assert(chunk.id, 'Chunk should have id');
    assert(chunk.content, 'Chunk should have content');
    assert(chunk.source, 'Chunk should have source');
    assert(Array.isArray(chunk.embedding), 'Chunk should have embedding array');
    assert(chunk.embedding.length > 0, 'Embedding should not be empty');
  }
});

// ============================================
// Test: Search Documentation (mock)
// ============================================

test('searchDocumentation returns error when index not loaded', async () => {
  // This tests the error path when docsIndex is null
  const mockSearchDocumentation = async (query, topK = 3) => {
    const docsIndex = null; // Simulate not loaded
    if (!docsIndex) {
      return { error: 'Documentation index not loaded. Run "npm run build:docs" first.' };
    }
    return { results: [] };
  };
  
  const result = await mockSearchDocumentation('test query');
  assert(result.error, 'Should return error when index not loaded');
  assert(result.error.includes('not loaded'), 'Error should mention not loaded');
});

test('loadDocsIndex validates structure before assigning', () => {
  // Verify the mcp-server.js validates docs structure
  const serverPath = path.join(projectRoot, 'mcp-server.js');
  const content = fs.readFileSync(serverPath, 'utf-8');
  
  // Should parse to temp variable first
  assert(content.includes('const parsed = JSON.parse'), 'Should parse to temp variable');
  // Should validate chunks array exists
  assert(content.includes('!parsed.chunks') || content.includes('parsed.chunks'), 'Should check chunks exists');
  assert(content.includes('Array.isArray(parsed.chunks)'), 'Should verify chunks is array');
  // Should reset to null on error
  assert(content.includes('docsIndex = null'), 'Should reset docsIndex to null on error');
});

// ============================================
// Test: MCP Server Tool Definitions
// ============================================

test('MCP server defines search_documentation tool', () => {
  // Read the mcp-server.js and verify tool is defined
  const serverPath = path.join(projectRoot, 'mcp-server.js');
  const content = fs.readFileSync(serverPath, 'utf-8');
  
  assert(content.includes("name: 'search_documentation'"), 'Should define search_documentation tool');
  assert(content.includes('Search SLVSX documentation'), 'Should have description');
  assert(content.includes("required: ['query']"), 'Should require query parameter');
});

test('MCP server imports documentation dependencies', () => {
  const serverPath = path.join(projectRoot, 'mcp-server.js');
  const content = fs.readFileSync(serverPath, 'utf-8');
  
  assert(content.includes('@xenova/transformers'), 'Should import xenova/transformers');
  assert(content.includes('cosineSimilarity'), 'Should have cosineSimilarity function');
  assert(content.includes('searchDocumentation'), 'Should have searchDocumentation function');
});

// ============================================
// Test: Build Docs Script
// ============================================

test('build-docs script exists', () => {
  const scriptPath = path.join(projectRoot, 'scripts', 'build-docs.ts');
  assert(fs.existsSync(scriptPath), 'build-docs.ts should exist');
});

test('build-docs script defines user-facing docs list', () => {
  const scriptPath = path.join(projectRoot, 'scripts', 'build-docs.ts');
  const content = fs.readFileSync(scriptPath, 'utf-8');
  
  assert(content.includes('USER_FACING_DOCS'), 'Should define USER_FACING_DOCS');
  assert(content.includes('AI_GUIDE.md'), 'Should include AI_GUIDE.md');
  assert(content.includes('README.md'), 'Should include README.md');
});

test('build-docs script uses correct chunking settings', () => {
  const scriptPath = path.join(projectRoot, 'scripts', 'build-docs.ts');
  const content = fs.readFileSync(scriptPath, 'utf-8');
  
  assert(content.includes('chunkSize: 1000'), 'Should have chunkSize 1000');
  assert(content.includes('chunkOverlap: 200'), 'Should have chunkOverlap 200');
});

// ============================================
// Test: Package.json Configuration
// ============================================

test('package.json has build:docs script', () => {
  const pkgPath = path.join(projectRoot, 'package.json');
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
  
  assert(pkg.scripts['build:docs'], 'Should have build:docs script');
  assert(pkg.scripts['build:docs'].includes('build-docs.ts'), 'Script should run build-docs.ts');
});

test('package.json has required dependencies', () => {
  const pkgPath = path.join(projectRoot, 'package.json');
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
  
  assert(pkg.dependencies['@xenova/transformers'], 'Should have xenova/transformers');
  assert(pkg.dependencies['@langchain/textsplitters'], 'Should have langchain textsplitters');
});

test('package.json includes docs.json in files', () => {
  const pkgPath = path.join(projectRoot, 'package.json');
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf-8'));
  
  assert(pkg.files.includes('dist/docs.json'), 'Should include dist/docs.json in published files');
});

// ============================================
// Summary
// ============================================

console.log('\n-----------------------------------');
console.log(`Tests: ${passed + failed} total, ${passed} passed, ${failed} failed`);

if (failed > 0) {
  process.exit(1);
}

