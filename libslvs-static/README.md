# libslvs-static - Static Build Fork of SolveSpace libslvs

## Overview

This directory contains a fork of the SolveSpace constraint solver library (libslvs), configured specifically for static linking with slvsx-cli. This fork maintains 100% of the original functionality - no features have been removed or truncated.

## GPL Compliance

**IMPORTANT**: This is a fork of SolveSpace, which is licensed under GPL-3.0. By using this code, the combined work (slvsx-cli) must also be distributed under GPL-3.0 or a compatible license.

### Original Project
- **Project**: SolveSpace
- **Website**: https://solvespace.com
- **Repository**: https://github.com/solvespace/solvespace
- **License**: GNU General Public License v3.0
- **Copyright**: SolveSpace contributors

### Our Compliance
We are committed to GPL compliance in both letter and spirit:

1. **Complete Source Code**: All source code is included, nothing is hidden or obfuscated
2. **Preserved Functionality**: All libslvs functionality is preserved - nothing truncated
3. **Clear Attribution**: Original copyright notices and author credits are maintained
4. **Modifications Documented**: All changes are clearly documented below
5. **License Propagation**: The combined work (slvsx-cli) is GPL-3.0 licensed

## Modifications from Original

The only modifications made are to the build system to enable static linking:

1. **CMakeLists.txt**: Simplified to build only libslvs as a static library
2. **No code changes**: The actual solver code is unchanged
3. **No functionality removed**: All solver capabilities remain intact

## Building

To build the static library:

```bash
mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF
make
```

The static library will be created as `build/lib/libslvs.a`

## Why This Fork?

This fork exists to:
1. Enable static linking for portable binaries
2. Simplify the build process by removing GUI dependencies
3. Ensure consistent builds across different platforms
4. Maintain a stable version for slvsx-cli

## Updates from Upstream

To update from the upstream SolveSpace repository:

```bash
# Add upstream remote (one time)
git remote add upstream https://github.com/solvespace/solvespace.git

# Fetch and merge updates
git fetch upstream
git merge upstream/master --allow-unrelated-histories
```

## License

This fork, like the original SolveSpace, is licensed under GPL-3.0. See LICENSE file for details.

## Acknowledgments

We gratefully acknowledge the SolveSpace team and contributors for creating this excellent constraint solver. This fork would not exist without their hard work and dedication to open source software.

---
*Last synced with upstream: 2024-09-07 (commit 74a13ef61c490a400eaa49c5335622d91ada12fc)*