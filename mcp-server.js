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

// Find slvsx binary - check env var, bundled, PATH, then local build
function findSlvsxBinary() {
  const ext = process.platform === 'win32' ? '.exe' : '';
  
  // 1. Check explicit env var
  if (process.env.SLVSX_BINARY) {
    return process.env.SLVSX_BINARY;
  }
  
  // 2. Check bundled binary (installed via postinstall)
  const bundled = path.join(__dirname, 'bin', `slvsx${ext}`);
  if (fs.existsSync(bundled)) {
    return bundled;
  }
  
  // 3. Check if slvsx is in PATH
  try {
    const which = execSync('which slvsx 2>/dev/null || where slvsx 2>nul', { encoding: 'utf-8' }).trim();
    // Handle Windows CRLF line endings by trimming each line
    if (which) return which.split('\n')[0].trim();
  } catch (e) {
    // Not in PATH
  }
  
  // 4. Check local build
  const localBuild = `./target/release/slvsx${ext}`;
  if (fs.existsSync(localBuild)) {
    return localBuild;
  }
  
  // 5. Check relative to this script (for development)
  const devBuild = path.join(__dirname, 'target/release', `slvsx${ext}`);
  if (fs.existsSync(devBuild)) {
    return devBuild;
  }
  
  return null;
}

const SLVSX_BINARY = findSlvsxBinary();

// Load documentation embeddings if available
let docsIndex = null;
let embedder = null;

const DOCS_INDEX_PATH = path.join(__dirname, 'dist', 'docs.json');
const EMBEDDING_MODEL = 'Xenova/all-MiniLM-L6-v2';

async function loadDocsIndex() {
  if (fs.existsSync(DOCS_INDEX_PATH)) {
    try {
      const data = fs.readFileSync(DOCS_INDEX_PATH, 'utf-8');
      const parsed = JSON.parse(data);
      // Validate structure before assigning
      if (!parsed.chunks || !Array.isArray(parsed.chunks)) {
        throw new Error('Invalid docs.json structure: missing or invalid chunks array');
      }
      // Validate model matches runtime embedder
      if (parsed.model && parsed.model !== EMBEDDING_MODEL) {
        throw new Error(`Model mismatch: docs.json uses '${parsed.model}' but runtime uses '${EMBEDDING_MODEL}'. Rebuild with 'npm run build:docs'.`);
      }
      docsIndex = parsed;
      console.error(`Loaded ${docsIndex.chunks.length} documentation chunks (model: ${EMBEDDING_MODEL})`);
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
    embedder = await pipeline('feature-extraction', EMBEDDING_MODEL);
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
        },
        {
          name: 'list_constraints',
          description: 'Get a complete reference of all constraint types with their field names and descriptions. Use this to see all available constraints.',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'list_entities',
          description: 'Get a complete reference of all entity types (point, line, circle, arc, etc.) with their field names and descriptions.',
          inputSchema: {
            type: 'object',
            properties: {},
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
          
          case 'list_constraints':
            return this.listConstraints();
          
          case 'list_entities':
            return this.listEntities();
          
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
          { type: 'point', id: 'p1', at: [0, 0, 0] },
          { type: 'point', id: 'p2', at: [100, 0, 0] },
          { type: 'point', id: 'p3', at: [50, 86.6, 0] }
        ],
        constraints: [
          { type: 'fixed', entity: 'p1' },
          { type: 'distance', between: ['p1', 'p2'], value: 100 },
          { type: 'distance', between: ['p2', 'p3'], value: 100 },
          { type: 'distance', between: ['p3', 'p1'], value: 100 }
        ]
      },
      square: {
        schema: 'slvs-json/1',
        units: 'mm',
        entities: [
          { type: 'point', id: 'p1', at: [0, 0, 0] },
          { type: 'point', id: 'p2', at: [100, 0, 0] },
          { type: 'point', id: 'p3', at: [100, 100, 0] },
          { type: 'point', id: 'p4', at: [0, 100, 0] },
          { type: 'line', id: 'l1', p1: 'p1', p2: 'p2' },
          { type: 'line', id: 'l2', p1: 'p2', p2: 'p3' },
          { type: 'line', id: 'l3', p1: 'p3', p2: 'p4' },
          { type: 'line', id: 'l4', p1: 'p4', p2: 'p1' }
        ],
        constraints: [
          { type: 'fixed', entity: 'p1' },
          { type: 'fixed', entity: 'p2' },
          { type: 'perpendicular', a: 'l1', b: 'l2' },
          { type: 'perpendicular', a: 'l2', b: 'l3' },
          { type: 'perpendicular', a: 'l3', b: 'l4' },
          { type: 'equal_length', entities: ['l1', 'l2'] }
        ]
      },
      circle: {
        schema: 'slvs-json/1',
        units: 'mm',
        entities: [
          { type: 'point', id: 'center', at: [50, 50, 0] },
          { type: 'circle', id: 'c1', center: [50, 50, 0], diameter: 60 }
        ],
        constraints: [
          { type: 'fixed', entity: 'center' },
          { type: 'diameter', circle: 'c1', value: 60 }
        ]
      },
      linkage: {
        schema: 'slvs-json/1',
        units: 'mm',
        parameters: {
          input_angle: 45
        },
        entities: [
          { type: 'point', id: 'ground1', at: [0, 0, 0] },
          { type: 'point', id: 'ground2', at: [100, 0, 0] },
          { type: 'point', id: 'joint1', at: [30, 30, 0] },
          { type: 'point', id: 'joint2', at: [70, 40, 0] },
          { type: 'line', id: 'link1', p1: 'ground1', p2: 'joint1' },
          { type: 'line', id: 'link2', p1: 'joint1', p2: 'joint2' },
          { type: 'line', id: 'link3', p1: 'joint2', p2: 'ground2' }
        ],
        constraints: [
          { type: 'fixed', entity: 'ground1' },
          { type: 'fixed', entity: 'ground2' },
          { type: 'distance', between: ['ground1', 'joint1'], value: 40 },
          { type: 'distance', between: ['joint1', 'joint2'], value: 50 },
          { type: 'distance', between: ['joint2', 'ground2'], value: 35 }
        ]
      },
      parametric: {
        schema: 'slvs-json/1',
        units: 'mm',
        parameters: {
          width: 150,
          height: 100,
          hole_diameter: 20
        },
        entities: [
          { type: 'point', id: 'p1', at: [0, 0, 0] },
          { type: 'point', id: 'p2', at: ['$width', 0, 0] },
          { type: 'point', id: 'p3', at: ['$width', '$height', 0] },
          { type: 'point', id: 'p4', at: [0, '$height', 0] },
          { type: 'line', id: 'bottom', p1: 'p1', p2: 'p2' },
          { type: 'line', id: 'right', p1: 'p2', p2: 'p3' },
          { type: 'line', id: 'top', p1: 'p3', p2: 'p4' },
          { type: 'line', id: 'left', p1: 'p4', p2: 'p1' },
          { type: 'circle', id: 'hole', center: [75, 50, 0], diameter: '$hole_diameter' }
        ],
        constraints: [
          { type: 'fixed', entity: 'p1' },
          { type: 'distance', between: ['p1', 'p2'], value: '$width' },
          { type: 'distance', between: ['p1', 'p4'], value: '$height' },
          { type: 'perpendicular', a: 'bottom', b: 'right' },
          { type: 'perpendicular', a: 'right', b: 'top' },
          { type: 'diameter', circle: 'hole', value: '$hole_diameter' }
        ]
      },
      '3d': {
        schema: 'slvs-json/1',
        units: 'mm',
        entities: [
          { type: 'point', id: 'p1', at: [0, 0, 0] },
          { type: 'point', id: 'p2', at: [100, 0, 0] },
          { type: 'point', id: 'p3', at: [100, 100, 0] },
          { type: 'point', id: 'p4', at: [0, 100, 0] },
          { type: 'point', id: 'p5', at: [0, 0, 50] },
          { type: 'point', id: 'p6', at: [100, 0, 50] },
          { type: 'point', id: 'p7', at: [100, 100, 50] },
          { type: 'point', id: 'p8', at: [0, 100, 50] },
          { type: 'line', id: 'base1', p1: 'p1', p2: 'p2' },
          { type: 'line', id: 'base2', p1: 'p2', p2: 'p3' },
          { type: 'line', id: 'base3', p1: 'p3', p2: 'p4' },
          { type: 'line', id: 'base4', p1: 'p4', p2: 'p1' },
          { type: 'line', id: 'top1', p1: 'p5', p2: 'p6' },
          { type: 'line', id: 'top2', p1: 'p6', p2: 'p7' },
          { type: 'line', id: 'top3', p1: 'p7', p2: 'p8' },
          { type: 'line', id: 'top4', p1: 'p8', p2: 'p5' },
          { type: 'line', id: 'vert1', p1: 'p1', p2: 'p5' },
          { type: 'line', id: 'vert2', p1: 'p2', p2: 'p6' },
          { type: 'line', id: 'vert3', p1: 'p3', p2: 'p7' },
          { type: 'line', id: 'vert4', p1: 'p4', p2: 'p8' }
        ],
        constraints: [
          { type: 'fixed', entity: 'p1' },
          { type: 'fixed', entity: 'p2' },
          { type: 'fixed', entity: 'p4' },
          { type: 'distance', between: ['p1', 'p5'], value: 50 },
          { type: 'distance', between: ['p2', 'p6'], value: 50 },
          { type: 'distance', between: ['p3', 'p7'], value: 50 },
          { type: 'distance', between: ['p4', 'p8'], value: 50 }
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

  listConstraints() {
    const constraintRef = `# SLVSX Constraint Reference

All constraints with their field names:

| Constraint | Fields | Description |
|------------|--------|-------------|
| \`fixed\` | \`entity\`, \`workplane?\` | Lock a point in place. For 2D points, include workplane. |
| \`distance\` | \`between: [p1, p2]\`, \`value\` | Set distance between two points |
| \`angle\` | \`between: [l1, l2]\`, \`value\` | Set angle between two lines (degrees) |
| \`perpendicular\` | \`a\`, \`b\` | Make two lines perpendicular |
| \`parallel\` | \`entities: [l1, l2, ...]\` | Make lines parallel |
| \`horizontal\` | \`a\`, \`workplane\` | Make line horizontal (2D only!) |
| \`vertical\` | \`a\`, \`workplane\` | Make line vertical (2D only!) |
| \`equal_length\` | \`entities: [l1, l2, ...]\` | Make lines equal length |
| \`equal_radius\` | \`a\`, \`b\` | Make circles/arcs equal radius |
| \`midpoint\` | \`point\`, \`of\` | Place point at midpoint of line |
| \`point_on_line\` | \`point\`, \`line\` | Constrain point to lie on line |
| \`point_on_circle\` | \`point\`, \`circle\` | Constrain point to lie on circle |
| \`coincident\` | \`entities: [p1, p2]\` | Make points coincident |
| \`tangent\` | \`a\`, \`b\` | Make arc/line tangent (NOT for circles!) |
| \`diameter\` | \`circle\`, \`value\` | Set circle diameter |
| \`symmetric\` | \`a\`, \`b\`, \`about\` | Mirror symmetry (2D only!) |
| \`symmetric_horizontal\` | \`a\`, \`b\`, \`workplane\` | Same Y, opposite X (mirror across Y-axis) |
| \`symmetric_vertical\` | \`a\`, \`b\`, \`workplane\` | Same X, opposite Y (mirror across X-axis) |
| \`dragged\` | \`point\`, \`workplane?\` | Lock point position absolutely |

## Important Notes
- **2D constraints** (horizontal, vertical, symmetric) require a workplane and 2D entities (point2_d, line2_d)
- **tangent** does NOT work with circle entities - use arc entities instead
- **symmetric** about a line requires 2D mode; use symmetric_horizontal/vertical for 3D`;

    return {
      content: [{ type: 'text', text: constraintRef }],
    };
  }

  listEntities() {
    const entityRef = `# SLVSX Entity Reference

All entity types with their field names:

| Entity | Fields | Description |
|--------|--------|-------------|
| \`point\` | \`id\`, \`at: [x,y,z]\`, \`preserve?\` | 3D point |
| \`point2_d\` | \`id\`, \`at: [u,v]\`, \`workplane\` | 2D point in workplane |
| \`line\` | \`id\`, \`p1\`, \`p2\` | Line between two 3D points |
| \`line2_d\` | \`id\`, \`p1\`, \`p2\`, \`workplane\` | Line between two 2D points |
| \`circle\` | \`id\`, \`center\`, \`diameter\`, \`normal?\` | Circle (center can be coords or point ref) |
| \`arc\` | \`id\`, \`center\`, \`start\`, \`end\`, \`normal\` | Arc defined by center and endpoints |
| \`cubic\` | \`id\`, \`control_points: [p0,p1,p2,p3]\` | Cubic Bezier curve |
| \`plane\` | \`id\`, \`origin: [x,y,z]\`, \`normal: [x,y,z]\` | Workplane definition |

## Circle Center Options
Circles can use either fixed coordinates or a point reference:

\`\`\`json
// Fixed coordinates (circle won't move)
{"type": "circle", "id": "c1", "center": [50, 50, 0], "diameter": 20}

// Point reference (circle tracks the point!)
{"type": "circle", "id": "c1", "center": "my_point", "diameter": 20}
\`\`\`

## 2D Geometry Setup
For horizontal/vertical constraints, use 2D entities:

\`\`\`json
{"type": "plane", "id": "xy", "origin": [0,0,0], "normal": [0,0,1]},
{"type": "point2_d", "id": "p1", "at": [0,0], "workplane": "xy"},
{"type": "line2_d", "id": "l1", "p1": "p1", "p2": "p2", "workplane": "xy"}
\`\`\``;

    return {
      content: [{ type: 'text', text: entityRef }],
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
if (!SLVSX_BINARY || !fs.existsSync(SLVSX_BINARY)) {
  console.error('Error: SLVSX binary not found.');
  console.error('');
  console.error('The MCP server requires the slvsx CLI binary. Install it via one of:');
  console.error('');
  console.error('  Option 1: Install via Homebrew (macOS/Linux)');
  console.error('    brew install sknoble/tap/slvsx');
  console.error('');
  console.error('  Option 2: Build from source');
  console.error('    git clone https://github.com/snoble/slvsx-cli');
  console.error('    cd slvsx-cli && cargo build --release');
  console.error('    export SLVSX_BINARY=$PWD/target/release/slvsx');
  console.error('');
  console.error('  Option 3: Download binary from GitHub releases');
  console.error('    https://github.com/snoble/slvsx-cli/releases');
  console.error('');
  console.error('Then restart the MCP server.');
  process.exit(1);
}

// Run the server
const server = new SlvsxServer();
server.run().catch(console.error);