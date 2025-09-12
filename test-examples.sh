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

# Find the slvsx binary (could be in different locations depending on build target)
if [ -f "./target/release/slvsx" ]; then
    SLVSX_BIN="./target/release/slvsx"
elif [ -f "./target/x86_64-unknown-linux-gnu/release/slvsx" ]; then
    SLVSX_BIN="./target/x86_64-unknown-linux-gnu/release/slvsx"
elif [ -f "./target/debug/slvsx" ]; then
    SLVSX_BIN="./target/debug/slvsx"
else
    echo -e "${RED}Error: slvsx binary not found!${NC}"
    echo "Looked in:"
    echo "  ./target/release/slvsx"
    echo "  ./target/x86_64-unknown-linux-gnu/release/slvsx"
    echo "  ./target/debug/slvsx"
    exit 1
fi

echo "Using binary: $SLVSX_BIN"

# Create temp directory for extracted JSON
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Track results
TOTAL=0
PASSED=0
FAILED=0

# No need to set library path - we're using static linking

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
    
    # Check if this test is expected to fail
    EXPECT_FAILURE=false
    if grep -q "❌ FAILS" "$example" || grep -q "Over-Constrained" "$example"; then
        EXPECT_FAILURE=true
        echo -e "${YELLOW}  (Expected to fail - over-constrained)${NC}"
    fi
    
    # Extract only the FIRST JSON content block between ```json and ```
    awk '/```json/{flag=1; next} /```/{if(flag) exit} flag' "$example" > "$TEMP_DIR/$basename.json"
    
    # Check if we extracted any JSON
    if [ ! -s "$TEMP_DIR/$basename.json" ]; then
        echo -e "${YELLOW}  No JSON content found, skipping${NC}"
        continue
    fi
    
    TOTAL=$((TOTAL + 1))
    
    # Test with slvsx solver
    if "$SLVSX_BIN" solve "$TEMP_DIR/$basename.json" > "$TEMP_DIR/$basename.output" 2>&1; then
        if [ "$EXPECT_FAILURE" = true ]; then
            echo -e "${RED}  ❌ Unexpectedly succeeded (should have failed)${NC}"
            FAILED=$((FAILED + 1))
        else
            echo -e "${GREEN}  ✅ Solved successfully${NC}"
            PASSED=$((PASSED + 1))
            
            # Generate SVG if it doesn't exist
            SVG_FILE="examples/$basename.svg"
            if [ ! -f "$SVG_FILE" ]; then
                echo -e "${YELLOW}  Generating SVG...${NC}"
                if "$SLVSX_BIN" export --format svg --output "$SVG_FILE" "$TEMP_DIR/$basename.json" 2>&1; then
                    echo -e "${GREEN}  ✅ SVG generated: $SVG_FILE${NC}"
                else
                    echo -e "${RED}  ❌ Failed to generate SVG${NC}"
                fi
            fi
        fi
    else
        if [ "$EXPECT_FAILURE" = true ]; then
            echo -e "${GREEN}  ✅ Failed as expected (over-constrained)${NC}"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}  ❌ Failed to solve${NC}"
            echo -e "${RED}  Error output:${NC}"
            cat "$TEMP_DIR/$basename.output" | head -10
            FAILED=$((FAILED + 1))
        fi
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