#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "================================================"
echo "         SLVSX Local CI Runner"
echo "================================================"

# Check if we're in a nix-shell or if nix is available
if [ -z "$IN_NIX_SHELL" ] && command -v nix-shell >/dev/null 2>&1; then
    echo -e "\n${YELLOW}Using nix-shell environment...${NC}"
    # Run the rest of the script inside nix-shell
    exec nix-shell build.nix --run "$0 --inside-nix $*"
fi

# If we get here, we're either inside nix or nix is not available
if [ "$1" = "--inside-nix" ]; then
    shift # Remove the --inside-nix flag
fi

# Track overall status
OVERALL_STATUS="passing"
EXIT_CODE=0

# Function to run a test
run_test() {
    local name="$1"
    local cmd="$2"
    
    echo -e "\n${YELLOW}Running: $name${NC}"
    
    if eval "$cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ $name passed${NC}"
    else
        echo -e "${RED}❌ $name failed${NC}"
        OVERALL_STATUS="failing"
        EXIT_CODE=1
    fi
}

# Run the test suite
run_test "Cargo Format Check" "cargo fmt -- --check"
run_test "Cargo Build" "cargo build --release"

# Quick test of the binary
if [ -f "./target/release/slvsx" ]; then
    run_test "Binary Test" "DYLD_LIBRARY_PATH=./libslvs/SolveSpaceLib/build/bin:$DYLD_LIBRARY_PATH ./target/release/slvsx --help"
    
    # Test all examples (allow 03_constraints to fail as it's intentionally over-constrained)
    if [ -f "./test-examples.sh" ]; then
        echo -e "\n${YELLOW}Running example tests...${NC}"
        ./test-examples.sh 2>&1 | tail -5
        # Check if only 03_constraints failed (which is expected)
        FAILED_COUNT=$(./test-examples.sh 2>&1 | grep "Failed:" | awk '{print $2}')
        if [ "$FAILED_COUNT" = "1" ]; then
            echo -e "${GREEN}✅ Example tests passed (03_constraints intentionally fails)${NC}"
        else
            echo -e "${RED}❌ Example tests failed unexpectedly${NC}"
            OVERALL_STATUS="failing"
            EXIT_CODE=1
        fi
    fi
fi

# Update CI status badge
if [ "$OVERALL_STATUS" = "passing" ]; then
    BADGE_COLOR="success"
    BADGE_TEXT="passing"
    STATUS_EMOJI="✅"
else
    BADGE_COLOR="critical"
    BADGE_TEXT="failing"
    STATUS_EMOJI="❌"
fi

# Update README badge
sed -i.bak "s|https://img.shields.io/badge/CI-[^)]*|https://img.shields.io/badge/CI-$BADGE_TEXT-$BADGE_COLOR|" README.md && rm README.md.bak

# Create status file
echo "# CI Status" > ci-status.md
echo "" >> ci-status.md
echo "![CI Status](https://img.shields.io/badge/CI-$BADGE_TEXT-$BADGE_COLOR)" >> ci-status.md
echo "" >> ci-status.md
echo "Last run: $(date)" >> ci-status.md
echo "Status: $STATUS_EMOJI $OVERALL_STATUS" >> ci-status.md

echo -e "\n================================================"
echo -e "Overall Status: $STATUS_EMOJI $OVERALL_STATUS"
echo -e "================================================"

# Option to commit status
if [ "$1" = "--commit" ]; then
    echo -e "\n${YELLOW}Committing CI status...${NC}"
    git add README.md ci-status.md
    git commit -m "CI Status: $STATUS_EMOJI $OVERALL_STATUS - $(date '+%Y-%m-%d %H:%M')" || echo "No changes to commit"
    echo -e "${GREEN}Status committed${NC}"
fi

exit $EXIT_CODE