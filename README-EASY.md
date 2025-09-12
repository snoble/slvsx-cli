# 🎯 slvsx - The EASIEST Geometry Solver

**Zero cloud. Zero cost. Zero hassle.**

## 🚀 Install (Pick ONE)

### Option 1: One-Line Install (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/snoble/slvsx-cli/main/install.sh | bash
```

### Option 2: NPX (No Install!)
```bash
npx slvsx solve geometry.json
```

### Option 3: Homebrew (Mac)
```bash
brew tap snoble/slvsx
brew install slvsx
```

### Option 4: Docker (Any OS)
```bash
docker run -i ghcr.io/snoble/slvsx solve - < geometry.json
```

### Option 5: Download Binary
Go to [Releases](https://github.com/snoble/slvsx-cli/releases) and download:
- Linux: `slvsx-linux`
- Mac: `slvsx-macos`

## 🎮 Use It (30 seconds)

### Simplest Example
```bash
echo '{"entities":[{"type":"point","id":"p1","at":[0,0,0]}],"constraints":[],"units":"mm"}' | slvsx solve -
```

### Real Example - Equilateral Triangle
```bash
cat << 'EOF' | slvsx solve -
{
  "entities": [
    {"type": "point", "id": "p1", "at": [0, 0, 0]},
    {"type": "point", "id": "p2", "at": [100, 0, 0]},
    {"type": "point", "id": "p3", "at": [50, 50, 0]}
  ],
  "constraints": [
    {"type": "distance", "between": ["p1", "p2"], "value": 100},
    {"type": "distance", "between": ["p2", "p3"], "value": 100},
    {"type": "distance", "between": ["p3", "p1"], "value": 100}
  ],
  "units": "mm"
}
EOF
```

## 🤖 For AI Agents

### Claude Desktop
Add to your config:
```json
{
  "mcpServers": {
    "slvsx": {
      "command": "npx",
      "args": ["slvsx", "mcp-server"]
    }
  }
}
```

### Python
```python
import subprocess, json

def solve_geometry(problem):
    result = subprocess.run(['npx', 'slvsx', 'solve', '-'], 
                          input=json.dumps(problem), 
                          capture_output=True, text=True)
    return json.loads(result.stdout)
```

### Node.js
```javascript
const { execSync } = require('child_process');

function solveGeometry(problem) {
  const result = execSync('npx slvsx solve -', {
    input: JSON.stringify(problem)
  });
  return JSON.parse(result.toString());
}
```

## 💡 Why This is Great

- **No Cloud Costs**: Runs 100% on YOUR computer
- **No API Keys**: No signup, no limits
- **Privacy**: Your CAD data never leaves your machine
- **Fast**: Native binary, instant results
- **Simple**: JSON in, JSON out

## 📚 What Can It Do?

- ✅ Points, lines, circles, arcs
- ✅ Distance, angle, parallel, perpendicular constraints
- ✅ Export to SVG, DXF, STL
- ✅ 2D and 3D geometry
- ✅ Under/over-constrained detection

## 🛠 Troubleshooting

**"Command not found"**: Add to PATH:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**"Permission denied"**: Make executable:
```bash
chmod +x slvsx
```

## 📦 Package Managers

Coming soon:
- `pip install slvsx`
- `cargo install slvsx`
- `apt install slvsx`
- `yum install slvsx`

## 🎯 That's It!

You now have industrial-strength geometry solving in 30 seconds. No cloud bills. No complexity.

---
**Run locally. Solve instantly. Pay nothing.** 🚀