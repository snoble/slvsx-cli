# 🚀 Quick Start - 30 Seconds to Geometry Solving

## Install in ONE Line

```bash
curl -fsSL https://raw.githubusercontent.com/snoble/slvsx-cli/main/install.sh | bash
```

That's it! You now have `slvsx` installed.

## Test It Works

```bash
# Simple point
echo '{"entities":[{"type":"point","id":"p1","at":[0,0,0]}],"constraints":[],"units":"mm"}' | slvsx solve -
```

## For AI Agents (Claude, GPT, etc.)

### Option 1: Direct CLI (Simplest)
```bash
# Your AI agent can just run:
slvsx solve geometry.json
slvsx export -f svg geometry.json > output.svg
```

### Option 2: Python One-Liner
```python
import subprocess, json
result = subprocess.run(['slvsx', 'solve', '-'], input=json.dumps(problem), capture_output=True, text=True)
solution = json.loads(result.stdout)
```

### Option 3: MCP Server Mode
Add to your Claude Desktop config:
```json
{
  "mcpServers": {
    "slvsx": {
      "command": "~/.local/bin/slvsx",
      "args": ["mcp-server"]
    }
  }
}
```

## Examples

### Triangle with Constraints
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

## No Cloud, No Cost

- Runs 100% locally on YOUR computer
- No API keys needed
- No cloud costs
- Your data stays private

## Troubleshooting

If `slvsx` isn't found after install:
```bash
export PATH="$HOME/.local/bin:$PATH"  # Add to your .bashrc/.zshrc
```

## Build from Source (Optional)

Only if you want to modify it:
```bash
git clone https://github.com/snoble/slvsx-cli.git
cd slvsx-cli
./build.sh  # We should create this
```

---
**That's it!** You're solving geometry in under a minute. 🎉