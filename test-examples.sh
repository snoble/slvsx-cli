#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "================================================"
echo "         Testing All Examples"
echo "================================================"

# Create temp directory for extracted JSON
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Track results
TOTAL=0
PASSED=0
FAILED=0

# Set library path for binary
export DYLD_LIBRARY_PATH="./libslvs/SolveSpaceLib/build/bin:$DYLD_LIBRARY_PATH"

# Extract and test each example
for example in examples/*.md; do
    if [ ! -f "$example" ]; then
        continue
    fi
    
    # Check if file contains JSON
    if ! grep -q '```json' "$example"; then
        continue
    fi
    
    basename=$(basename "$example" .md)
    echo -e "\n${YELLOW}Testing: $basename${NC}"
    
    # Extract JSON content between ```json and ```
    awk '/```json/{flag=1; next} /```/{flag=0} flag' "$example" > "$TEMP_DIR/$basename.json"
    
    # Check if we extracted any JSON
    if [ ! -s "$TEMP_DIR/$basename.json" ]; then
        echo -e "${YELLOW}  No JSON content found, skipping${NC}"
        continue
    fi
    
    TOTAL=$((TOTAL + 1))
    
    # Test with slvsx solver
    if ./target/release/slvsx solve "$TEMP_DIR/$basename.json" > "$TEMP_DIR/$basename.output" 2>&1; then
        echo -e "${GREEN}  ✅ Solved successfully${NC}"
        PASSED=$((PASSED + 1))
        
        # Generate SVG if it doesn't exist
        SVG_FILE="examples/$basename.svg"
        if [ ! -f "$SVG_FILE" ]; then
            echo -e "${YELLOW}  Generating SVG...${NC}"
            if ./target/release/slvsx export --format svg --output "$SVG_FILE" "$TEMP_DIR/$basename.json" 2>&1; then
                echo -e "${GREEN}  ✅ SVG generated: $SVG_FILE${NC}"
            else
                echo -e "${RED}  ❌ Failed to generate SVG${NC}"
            fi
        fi
    else
        echo -e "${RED}  ❌ Failed to solve${NC}"
        echo -e "${RED}  Error output:${NC}"
        cat "$TEMP_DIR/$basename.output" | head -10
        FAILED=$((FAILED + 1))
    fi
done

echo -e "\n================================================"
echo -e "Test Results:"
echo -e "  Total:  $TOTAL"
echo -e "  ${GREEN}Passed: $PASSED${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "  ${RED}Failed: $FAILED${NC}"
else
    echo -e "  Failed: 0"
fi
echo -e "================================================"

if [ $FAILED -gt 0 ]; then
    exit 1
fi