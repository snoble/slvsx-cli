#!/bin/bash

# Revert all image links back to relative paths
for file in examples/[0-9]*.md; do
    base=$(basename "$file" .md)
    echo "Fixing $base..."
    
    # Create temp file
    cp "$file" "$file.tmp"
    
    # Change absolute raw URLs back to relative paths for images
    sed -i '' "s|](https://raw.githubusercontent.com/snoble/slvsx-cli/main/examples/${base}.svg)|](${base}.svg)|g" "$file.tmp"
    
    # Keep navigation links as relative too
    sed -i '' 's|](https://github.com/snoble/slvsx-cli/blob/main/examples/\([^)]*\))|](\1)|g' "$file.tmp"
    
    # Move temp file back
    mv "$file.tmp" "$file"
done

echo "Reverted all links to relative paths!"