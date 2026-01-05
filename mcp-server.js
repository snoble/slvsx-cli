#!/usr/bin/env node

/**
 * MCP Server for SLVSX Constraint Solver
 * 
 * Exposes the SLVSX constraint solver as an MCP server that can be used
 * by Claude and other AI assistants to solve geometric constraints.
 */

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { 
  CallToolRequestSchema, 
  ListToolsRequestSchema 
} from '@modelcontextprotocol/sdk/types.js';
import { execSync, spawn } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import { fileURLToPath } from 'url';

// Get directory of this script
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Check if slvsx binary exists
const SLVSX_BINARY = process.env.SLVSX_BINARY || './target/release/slvsx';

// Load documentation embeddings if available
let docsIndex = null;
let embedder = null;

const DOCS_INDEX_PATH = path.join(__dirname, 'dist', 'docs.json');

async function loadDocsIndex() {
  if (fs.existsSync(DOCS_INDEX_PATH)) {
    try {
      const data = fs.readFileSync(DOCS_INDEX_PATH, 'utf-8');
      const parsed = JSON.parse(data);
      // Validate structure before assigning
      if (!parsed.chunks || !Array.isArray(parsed.chunks)) {
        throw new Error('Invalid docs.json structure: missing or invalid chunks array');
      }
      docsIndex = parsed;
      console.error(`Loaded ${docsIndex.chunks.length} documentation chunks`);
    } catch (e) {
      docsIndex = null; // Reset to null on any error
      console.error('Warning: Failed to load docs index:', e.message);
    }
  } else {
    console.error('Note: docs.json not found. Run "npm run build:docs" to enable documentation search.');
  }
}

async function getEmbedder() {
  if (!embedder) {
    const { pipeline } = await import('@xenova/transformers');
    embedder = await pipeline('feature-extraction', 'Xenova/all-MiniLM-L6-v2');
  }
  return embedder;
}

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

async function searchDocumentation(query, topK = 3) {
  if (!docsIndex) {
    return { error: 'Documentation index not loaded. Run "npm run build:docs" first.' };
  }

  try {
    const embed = await getEmbedder();
    const output = await embed(query, { pooling: 'mean', normalize: true });
    const queryEmbedding = Array.from(output.data);

    // Calculate similarities
    const results = docsIndex.chunks.map(chunk => ({
      ...chunk,
      similarity: cosineSimilarity(queryEmbedding, chunk.embedding),
    }));

    // Sort by similarity and take top K
    results.sort((a, b) => b.similarity - a.similarity);
    const topResults = results.slice(0, topK);

    return {
      results: topResults.map(r => ({
        source: r.source,
        content: r.content,
        score: r.similarity.toFixed(4),
      })),
    };
  } catch (e) {
    return { error: `Search failed: ${e.message}` };
  }
}

class SlvsxServer {
  constructor() {
    this.server = new Server(
      {
        name: 'slvsx-mcp',
        version: '0.1.0',
      },
      {
        capabilities: {
          tools: {},
        },
      }
    );

    this.setupHandlers();
  }

  setupHandlers() {
    // List available tools
    this.server.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [
        {
          name: 'solve_constraints',
          description: 'Solve geometric constraints using SLVSX solver',
          inputSchema: {
            type: 'object',
            properties: {
              constraints: {
                type: 'object',
                description: 'JSON constraint document following SLVSX schema',
                properties: {
                  schema: { type: 'string', enum: ['slvs-json/1'] },
                  units: { type: 'string', enum: ['mm', 'cm', 'm', 'in', 'ft'] },
                  parameters: { type: 'object' },
                  entities: { type: 'array' },
                  constraints: { type: 'array' }
                },
                required: ['schema', 'entities', 'constraints']
              }
            },
            required: ['constraints'],
          },
        },
        {
          name: 'validate_constraints',
          description: 'Validate a constraint document without solving',
          inputSchema: {
            type: 'object',
            properties: {
              constraints: {
                type: 'object',
                description: 'JSON constraint document to validate',
              }
            },
            required: ['constraints'],
          },
        },
        {
          name: 'export_to_svg',
          description: 'Solve constraints and export result to SVG',
          inputSchema: {
            type: 'object',
            properties: {
              constraints: {
                type: 'object',
                description: 'JSON constraint document',
              },
              width: {
                type: 'number',
                description: 'SVG width in pixels',
                default: 800
              },
              height: {
                type: 'number',
                description: 'SVG height in pixels',
                default: 600
              }
            },
            required: ['constraints'],
          },
        },
        {
          name: 'get_schema',
          description: 'Get the JSON schema for constraint documents',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'create_example',
          description: 'Create an example constraint document',
          inputSchema: {
            type: 'object',
            properties: {
              type: {
                type: 'string',
                description: 'Type of example to create',
                enum: ['triangle', 'square', 'circle', 'linkage', 'parametric', '3d']
              }
            },
            required: ['type'],
          },
        },
        {
          name: 'search_documentation',
          description: 'Search SLVSX documentation for relevant information about using the constraint solver, JSON schema, examples, and best practices',
          inputSchema: {
            type: 'object',
            properties: {
              query: {
                type: 'string',
                description: 'Search query to find relevant documentation',
              }
            },
            required: ['query'],
          },
        }
      ],
    }));

    // Handle tool calls
    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;

      try {
        switch (name) {
          case 'solve_constraints':
            return await this.solveConstraints(args.constraints);
          
          case 'validate_constraints':
            return await this.validateConstraints(args.constraints);
          
          case 'export_to_svg':
            return await this.exportToSvg(args.constraints, args.width, args.height);
          
          case 'get_schema':
            return await this.getSchema();
          
          case 'create_example':
            return await this.createExample(args.type);
          
          case 'search_documentation':
            return await this.searchDocs(args.query);
          
          default:
            throw new Error(`Unknown tool: ${name}`);
        }
      } catch (error) {
        return {
          content: [
            {
              type: 'text',
              text: `Error: ${error.message}`,
            },
          ],
        };
      }
    });
  }

  async solveConstraints(constraints) {
    // Write constraints to temp file
    const tmpFile = path.join(os.tmpdir(), `slvsx-${Date.now()}.json`);
    fs.writeFileSync(tmpFile, JSON.stringify(constraints, null, 2));

    try {
      // Run slvsx solve
      const result = execSync(`${SLVSX_BINARY} solve ${tmpFile}`, {
        encoding: 'utf-8',
        maxBuffer: 10 * 1024 * 1024 // 10MB
      });

      // Parse the result
      const lines = result.split('\n');
      let jsonResult = '';
      let inJson = false;
      
      for (const line of lines) {
        if (line.startsWith('{')) inJson = true;
        if (inJson) jsonResult += line + '\n';
        if (line.startsWith('}')) inJson = false;
      }

      const solved = jsonResult ? JSON.parse(jsonResult) : { error: 'No solution found' };

      return {
        content: [
          {
            type: 'text',
            text: JSON.stringify(solved, null, 2),
          },
        ],
      };
    } finally {
      // Clean up temp file
      if (fs.existsSync(tmpFile)) {
        fs.unlinkSync(tmpFile);
      }
    }
  }

  async validateConstraints(constraints) {
    const tmpFile = path.join(os.tmpdir(), `slvsx-validate-${Date.now()}.json`);
    fs.writeFileSync(tmpFile, JSON.stringify(constraints, null, 2));

    try {
      const result = execSync(`${SLVSX_BINARY} validate ${tmpFile}`, {
        encoding: 'utf-8'
      });

      return {
        content: [
          {
            type: 'text',
            text: result.includes('✓') ? 'Valid constraint document' : result,
          },
        ],
      };
    } catch (error) {
      return {
        content: [
          {
            type: 'text',
            text: `Validation failed: ${error.message}`,
          },
        ],
      };
    } finally {
      if (fs.existsSync(tmpFile)) {
        fs.unlinkSync(tmpFile);
      }
    }
  }

  async exportToSvg(constraints, width = 800, height = 600) {
    const tmpInput = path.join(os.tmpdir(), `slvsx-input-${Date.now()}.json`);
    const tmpOutput = path.join(os.tmpdir(), `slvsx-output-${Date.now()}.svg`);
    
    fs.writeFileSync(tmpInput, JSON.stringify(constraints, null, 2));

    try {
      execSync(`${SLVSX_BINARY} export --format svg --output ${tmpOutput} ${tmpInput}`, {
        encoding: 'utf-8'
      });

      const svg = fs.readFileSync(tmpOutput, 'utf-8');
      
      return {
        content: [
          {
            type: 'text',
            text: svg,
          },
        ],
      };
    } finally {
      if (fs.existsSync(tmpInput)) fs.unlinkSync(tmpInput);
      if (fs.existsSync(tmpOutput)) fs.unlinkSync(tmpOutput);
    }
  }

  async getSchema() {
    try {
      const schema = execSync(`${SLVSX_BINARY} schema`, {
        encoding: 'utf-8'
      });

      return {
        content: [
          {
            type: 'text',
            text: schema,
          },
        ],
      };
    } catch (error) {
      return {
        content: [
          {
            type: 'text',
            text: `Failed to get schema: ${error.message}`,
          },
        ],
      };
    }
  }

  async createExample(type) {
    const examples = {
      triangle: {
        schema: 'slvs-json/1',
        units: 'mm',
        entities: [
          { id: 'p1', type: 'Point', x: 0, y: 0 },
          { id: 'p2', type: 'Point', x: 100, y: 0 },
          { id: 'p3', type: 'Point', x: 50, y: 86.6 }
        ],
        constraints: [
          { type: 'Fixed', entity: 'p1' },
          { type: 'Distance', entities: ['p1', 'p2'], distance: 100 },
          { type: 'Distance', entities: ['p2', 'p3'], distance: 100 },
          { type: 'Distance', entities: ['p3', 'p1'], distance: 100 }
        ]
      },
      square: {
        schema: 'slvs-json/1',
        units: 'mm',
        entities: [
          { id: 'p1', type: 'Point', x: 0, y: 0 },
          { id: 'p2', type: 'Point', x: 100, y: 0 },
          { id: 'p3', type: 'Point', x: 100, y: 100 },
          { id: 'p4', type: 'Point', x: 0, y: 100 },
          { id: 'l1', type: 'Line', points: ['p1', 'p2'] },
          { id: 'l2', type: 'Line', points: ['p2', 'p3'] },
          { id: 'l3', type: 'Line', points: ['p3', 'p4'] },
          { id: 'l4', type: 'Line', points: ['p4', 'p1'] }
        ],
        constraints: [
          { type: 'Fixed', entity: 'p1' },
          { type: 'Fixed', entity: 'p2' },
          { type: 'Perpendicular', entities: ['l1', 'l2'] },
          { type: 'Perpendicular', entities: ['l2', 'l3'] },
          { type: 'Perpendicular', entities: ['l3', 'l4'] },
          { type: 'Equal', entities: ['l1', 'l2'] }
        ]
      },
      circle: {
        schema: 'slvs-json/1',
        units: 'mm',
        entities: [
          { id: 'center', type: 'Point', x: 50, y: 50 },
          { id: 'c1', type: 'Circle', center: 'center', radius: 30 }
        ],
        constraints: [
          { type: 'Fixed', entity: 'center' },
          { type: 'Radius', entity: 'c1', radius: 30 }
        ]
      },
      linkage: {
        schema: 'slvs-json/1',
        units: 'mm',
        parameters: {
          input_angle: 45
        },
        entities: [
          { id: 'ground1', type: 'Point', x: 0, y: 0 },
          { id: 'ground2', type: 'Point', x: 100, y: 0 },
          { id: 'joint1', type: 'Point', x: 30, y: 30 },
          { id: 'joint2', type: 'Point', x: 70, y: 40 },
          { id: 'link1', type: 'Line', points: ['ground1', 'joint1'] },
          { id: 'link2', type: 'Line', points: ['joint1', 'joint2'] },
          { id: 'link3', type: 'Line', points: ['joint2', 'ground2'] }
        ],
        constraints: [
          { type: 'Fixed', entity: 'ground1' },
          { type: 'Fixed', entity: 'ground2' },
          { type: 'Distance', entities: ['ground1', 'joint1'], distance: 40 },
          { type: 'Distance', entities: ['joint1', 'joint2'], distance: 50 },
          { type: 'Distance', entities: ['joint2', 'ground2'], distance: 35 },
          { type: 'Angle', entities: ['link1'], angle: '$input_angle' }
        ]
      },
      parametric: {
        schema: 'slvs-json/1',
        units: 'mm',
        parameters: {
          width: 150,
          height: 100,
          hole_radius: 10
        },
        entities: [
          { id: 'p1', type: 'Point', x: 0, y: 0 },
          { id: 'p2', type: 'Point', x: '$width', y: 0 },
          { id: 'p3', type: 'Point', x: '$width', y: '$height' },
          { id: 'p4', type: 'Point', x: 0, y: '$height' },
          { id: 'hole_center', type: 'Point', x: 75, y: 50 },
          { id: 'hole', type: 'Circle', center: 'hole_center', radius: '$hole_radius' }
        ],
        constraints: [
          { type: 'Fixed', entity: 'p1' },
          { type: 'HorizontalDistance', entities: ['p1', 'p2'], distance: '$width' },
          { type: 'VerticalDistance', entities: ['p1', 'p4'], distance: '$height' },
          { type: 'Horizontal', entity: 'p2' },
          { type: 'Vertical', entity: 'p4' },
          { type: 'Radius', entity: 'hole', radius: '$hole_radius' }
        ]
      },
      '3d': {
        schema: 'slvs-json/1',
        units: 'mm',
        entities: [
          { id: 'p1', type: 'Point', x: 0, y: 0, z: 0 },
          { id: 'p2', type: 'Point', x: 100, y: 0, z: 0 },
          { id: 'p3', type: 'Point', x: 100, y: 100, z: 0 },
          { id: 'p4', type: 'Point', x: 0, y: 100, z: 0 },
          { id: 'p5', type: 'Point', x: 0, y: 0, z: 50 },
          { id: 'p6', type: 'Point', x: 100, y: 0, z: 50 },
          { id: 'p7', type: 'Point', x: 100, y: 100, z: 50 },
          { id: 'p8', type: 'Point', x: 0, y: 100, z: 50 }
        ],
        constraints: [
          { type: 'Fixed', entity: 'p1' },
          { type: 'Distance', entities: ['p1', 'p2'], distance: 100 },
          { type: 'Distance', entities: ['p2', 'p3'], distance: 100 },
          { type: 'Distance', entities: ['p3', 'p4'], distance: 100 },
          { type: 'Distance', entities: ['p4', 'p1'], distance: 100 },
          { type: 'Distance', entities: ['p1', 'p5'], distance: 50 },
          { type: 'Distance', entities: ['p2', 'p6'], distance: 50 },
          { type: 'Distance', entities: ['p3', 'p7'], distance: 50 },
          { type: 'Distance', entities: ['p4', 'p8'], distance: 50 }
        ]
      }
    };

    const example = examples[type];
    if (!example) {
      return {
        content: [
          {
            type: 'text',
            text: `Unknown example type: ${type}. Available: ${Object.keys(examples).join(', ')}`,
          },
        ],
      };
    }

    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(example, null, 2),
        },
      ],
    };
  }

  async searchDocs(query) {
    const result = await searchDocumentation(query, 3);
    
    if (result.error) {
      return {
        content: [
          {
            type: 'text',
            text: result.error,
          },
        ],
      };
    }

    // Format results as readable text
    const formatted = result.results.map((r, i) => 
      `--- Result ${i + 1} (${r.source}, score: ${r.score}) ---\n${r.content}`
    ).join('\n\n');

    return {
      content: [
        {
          type: 'text',
          text: formatted || 'No results found.',
        },
      ],
    };
  }

  async run() {
    // Load documentation index for search
    await loadDocsIndex();
    
    const transport = new StdioServerTransport();
    await this.server.connect(transport);
    console.error('SLVSX MCP Server running on stdio');
  }
}

// Check if slvsx binary exists
if (!fs.existsSync(SLVSX_BINARY)) {
  console.error(`Error: SLVSX binary not found at ${SLVSX_BINARY}`);
  console.error('Please build the project first with: cargo build --release');
  console.error('Or set SLVSX_BINARY environment variable to point to the binary');
  process.exit(1);
}

// Run the server
const server = new SlvsxServer();
server.run().catch(console.error);