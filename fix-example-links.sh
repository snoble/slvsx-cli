#!/bin/bash

# Base URL for GitHub
BASE_URL="https://github.com/snoble/slvsx-cli/blob/main/examples"
RAW_URL="https://raw.githubusercontent.com/snoble/slvsx-cli/main/examples"

# Fix all numbered examples
for file in examples/[0-9]*.md; do
    base=$(basename "$file" .md)
    echo "Fixing $base..."
    
    # Create temp file
    cp "$file" "$file.tmp"
    
    # Fix image references - change relative to absolute raw URLs
    sed -i '' "s|](${base}.svg)|](${RAW_URL}/${base}.svg)|g" "$file.tmp"
    
    # Fix navigation links - change relative to absolute GitHub URLs
    sed -i '' 's|](\([0-9][0-9]_[^)]*\.md\))|]('"${BASE_URL}"'/\1)|g' "$file.tmp"
    
    # Move temp file back
    mv "$file.tmp" "$file"
done

echo "Fixed all example links!"