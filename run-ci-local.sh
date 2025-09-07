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

# Create a results directory with timestamp
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="ci-results/$TIMESTAMP"
mkdir -p "$RESULTS_DIR"

# Initialize results file
RESULTS_FILE="$RESULTS_DIR/results.md"
echo "# CI Results - $(date)" > "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Track overall status
OVERALL_STATUS="✅ PASSED"

# Function to run a test and capture results
run_test() {
    local name="$1"
    local cmd="$2"
    
    echo -e "\n${YELLOW}Running: $name${NC}"
    echo "## $name" >> "$RESULTS_FILE"
    echo '```' >> "$RESULTS_FILE"
    
    if eval "$cmd" >> "$RESULTS_FILE" 2>&1; then
        echo -e "${GREEN}✅ $name passed${NC}"
        echo '```' >> "$RESULTS_FILE"
        echo "**Result: ✅ PASSED**" >> "$RESULTS_FILE"
    else
        echo -e "${RED}❌ $name failed${NC}"
        echo '```' >> "$RESULTS_FILE"
        echo "**Result: ❌ FAILED**" >> "$RESULTS_FILE"
        OVERALL_STATUS="❌ FAILED"
    fi
    echo "" >> "$RESULTS_FILE"
}

# Build libslvs if needed
echo -e "\n${YELLOW}Building libslvs...${NC}"
if [ ! -f libslvs/SolveSpaceLib/build/libslvs.a ]; then
    mkdir -p libslvs/SolveSpaceLib/build
    cd libslvs/SolveSpaceLib/build
    cmake .. -DCMAKE_BUILD_TYPE=Release
    make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu)
    cd ../../..
fi

export LIBSLVS_DIR="$PWD/libslvs/SolveSpaceLib/build"
export LD_LIBRARY_PATH="$LIBSLVS_DIR:$LD_LIBRARY_PATH"
export DYLD_LIBRARY_PATH="$LIBSLVS_DIR:$DYLD_LIBRARY_PATH"

# Run the test suite
run_test "Cargo Format Check" "cargo fmt -- --check"
run_test "Cargo Clippy" "cargo clippy --all-targets --all-features -- -D warnings"
run_test "Cargo Test" "cargo test --verbose"
run_test "Cargo Build" "cargo build --release"

# Test examples
echo -e "\n${YELLOW}Testing examples...${NC}"
echo "## Example Validation" >> "$RESULTS_FILE"
EXAMPLE_FAILED=false
for example in examples/*.json; do
    if [ -f "$example" ]; then
        echo "Testing $(basename $example)..." >> "$RESULTS_FILE"
        if ./target/release/slvsx validate "$example" >> "$RESULTS_FILE" 2>&1; then
            echo "✅ $(basename $example)" >> "$RESULTS_FILE"
        else
            echo "❌ $(basename $example)" >> "$RESULTS_FILE"
            EXAMPLE_FAILED=true
        fi
    fi
done

if [ "$EXAMPLE_FAILED" = true ]; then
    OVERALL_STATUS="❌ FAILED"
fi

# Summary
echo "" >> "$RESULTS_FILE"
echo "## Overall Status: $OVERALL_STATUS" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "Run completed at $(date)" >> "$RESULTS_FILE"

echo -e "\n================================================"
echo -e "Overall Status: $OVERALL_STATUS"
echo -e "Results saved to: $RESULTS_FILE"
echo -e "================================================"

# Option to push results to GitHub
if [ "$1" = "--push" ]; then
    echo -e "\n${YELLOW}Pushing results to GitHub...${NC}"
    
    # Create a badge
    if [ "$OVERALL_STATUS" = "✅ PASSED" ]; then
        BADGE_COLOR="success"
        BADGE_TEXT="passing"
    else
        BADGE_COLOR="critical"
        BADGE_TEXT="failing"
    fi
    
    # Update README with badge (if wanted)
    # echo "![CI Status](https://img.shields.io/badge/CI-$BADGE_TEXT-$BADGE_COLOR)" > ci-status.md
    
    # Copy latest results to root
    cp "$RESULTS_FILE" "ci-latest-results.md"
    
    # Commit and push
    git add ci-latest-results.md ci-results/
    git commit -m "CI Results: $OVERALL_STATUS - $TIMESTAMP" || echo "No changes to commit"
    git push origin main || echo "Push failed - may need to pull first"
    
    echo -e "${GREEN}Results pushed to GitHub${NC}"
fi

# Exit with appropriate code
if [ "$OVERALL_STATUS" = "✅ PASSED" ]; then
    exit 0
else
    exit 1
fi