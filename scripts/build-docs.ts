#!/usr/bin/env npx tsx
/**
 * Build documentation embeddings for MCP documentation search
 * 
 * This script:
 * 1. Reads user-facing markdown files (for using the CLI/MCP)
 * 2. Chunks them using RecursiveCharacterTextSplitter
 * 3. Generates embeddings using @xenova/transformers
 * 4. Saves the output to dist/docs.json
 * 
 * Run with: npm run build:docs
 */

import * as fs from 'fs';
import * as path from 'path';
import { RecursiveCharacterTextSplitter } from '@langchain/textsplitters';
import { pipeline } from '@xenova/transformers';

interface DocChunk {
  id: string;
  content: string;
  source: string;
  embedding: number[];
}

interface DocsIndex {
  version: string;
  model: string;
  chunks: DocChunk[];
  createdAt: string;
}

// User-facing docs for using the CLI/MCP (not developer docs)
const USER_FACING_DOCS = [
  // Main docs directory - user guides
  'docs/AI_GUIDE.md',
  'docs/AI_INTERACTIVE_FEATURES.md',
  'docs/AI_MODELING_GUIDE.md',
  'docs/ADVANCED_TECHNIQUES.md',  // Walls with thickness, patterns, mechanisms
  'docs/ITERATIVE_DESIGN.md',
  'docs/JSON_SCHEMA.md',
  'docs/MCP_INTEGRATION.md',
  'docs/USAGE_EXAMPLES.md',
  'docs/VISUAL_GALLERY.md',
  // Generators documentation
  'examples/generators/README.md',
  // Top-level user docs
  'README.md',
  'QUICKSTART.md',
  'MCP_SETUP.md',
  'MCP_SERVER.md',
  'README-EASY.md',
  'SHOWCASE.md',
];

async function main() {
  const projectRoot = path.resolve(process.cwd());
  const outputDir = path.join(projectRoot, 'dist');
  const outputFile = path.join(outputDir, 'docs.json');
  
  console.log('📚 Building documentation embeddings...');
  console.log(`   Project root: ${projectRoot}`);
  console.log('   Including only user-facing documentation');
  
  // Get list of files that exist
  const allFiles = USER_FACING_DOCS
    .map(f => path.join(projectRoot, f))
    .filter(f => {
      if (fs.existsSync(f)) {
        return true;
      }
      console.log(`   Skipping (not found): ${f}`);
      return false;
    });
  
  console.log(`   Found ${allFiles.length} markdown files`);
  
  if (allFiles.length === 0) {
    console.error('❌ No markdown files found!');
    process.exit(1);
  }
  
  // Initialize text splitter
  // Larger chunks (2000 chars) provide more context for AI agents
  const splitter = new RecursiveCharacterTextSplitter({
    chunkSize: 2000,
    chunkOverlap: 400,
    separators: ['\n## ', '\n### ', '\n#### ', '\n\n', '\n', ' ', ''],
  });
  
  // Collect all chunks
  const allChunks: { content: string; source: string }[] = [];
  
  for (const filePath of allFiles) {
    const relativePath = path.relative(projectRoot, filePath);
    console.log(`   Processing: ${relativePath}`);
    
    const content = fs.readFileSync(filePath, 'utf-8');
    const chunks = await splitter.splitText(content);
    
    for (const chunk of chunks) {
      allChunks.push({
        content: chunk,
        source: relativePath,
      });
    }
  }
  
  console.log(`   Total chunks: ${allChunks.length}`);
  
  // Initialize the embedding model
  console.log('🤖 Loading embedding model...');
  const embedder = await pipeline('feature-extraction', 'Xenova/all-MiniLM-L6-v2');
  
  // Generate embeddings
  console.log('🔢 Generating embeddings...');
  const indexedChunks: DocChunk[] = [];
  
  for (let i = 0; i < allChunks.length; i++) {
    const chunk = allChunks[i];
    
    // Generate embedding
    const output = await embedder(chunk.content, { pooling: 'mean', normalize: true });
    const embedding = Array.from(output.data as Float32Array);
    
    indexedChunks.push({
      id: `chunk_${i}`,
      content: chunk.content,
      source: chunk.source,
      embedding,
    });
    
    // Progress indicator
    if ((i + 1) % 10 === 0 || i === allChunks.length - 1) {
      console.log(`   Progress: ${i + 1}/${allChunks.length}`);
    }
  }
  
  // Create output directory if needed
  if (!fs.existsSync(outputDir)) {
    fs.mkdirSync(outputDir, { recursive: true });
  }
  
  // Save the index
  const docsIndex: DocsIndex = {
    version: '1.0.0',
    model: 'Xenova/all-MiniLM-L6-v2',
    chunks: indexedChunks,
    createdAt: new Date().toISOString(),
  };
  
  fs.writeFileSync(outputFile, JSON.stringify(docsIndex, null, 2));
  
  console.log(`\n✅ Documentation index saved to: ${outputFile}`);
  console.log(`   Total chunks: ${indexedChunks.length}`);
  console.log(`   File size: ${(fs.statSync(outputFile).size / 1024 / 1024).toFixed(2)} MB`);
}

main().catch((error) => {
  console.error('❌ Error building docs:', error);
  process.exit(1);
});

