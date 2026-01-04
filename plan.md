# SLVSX-CLI Improvement Plan

This plan breaks down the work from [spec.md](./spec.md) into sessions that can be done incrementally.

---

## Session 1: Branch Protection & Codecov Setup ✅ COMPLETE

**Goal**: Get CI rigor in place before making other changes.

**Status**: ✅ Complete - Merged in PR #5

### Tasks

1. **Configure branch protection** (GitHub settings) ✅
   - Enable "Require a pull request before merging"
   - Enable "Require status checks to pass before merging"
   - Add required checks: `build (ubuntu-latest)`, `build (macos-latest)`
   - Optional: Require 1 approving review

2. **Create codecov.yml** in repo root ✅
   ```yaml
   codecov:
     require_ci_to_pass: yes

   coverage:
     precision: 2
     round: down
     status:
       project:
         default:
           target: 100%
           threshold: 0%
       patch:
         default:
           target: 100%

   comment:
     layout: "reach,diff,flags,files"
     behavior: default

   ignore:
     - "ffi/**/*"
     - "**/generated/**"
     - "schema/**/*"
   ```

3. **Create COVERAGE.md** policy document
   - Explain 100% coverage goal
   - Document exception policy (provably unreachable code, generated code)
   - Require proof that code is unreachable, not just claims
   - Show how to run coverage locally

4. **Update .github/workflows/build.yml**
   - Add `cargo-llvm-cov` installation
   - Run tests with coverage collection
   - Upload to Codecov

5. **Add coverage badge to README.md**
   ```markdown
   [![codecov](https://codecov.io/gh/snoble/slvsx-cli/graph/badge.svg)](https://codecov.io/gh/snoble/slvsx-cli)
   ```

### Verification
- [x] Direct push to main is blocked
- [x] PR requires CI to pass
- [x] Coverage report appears on Codecov
- [x] Coverage badge shows in README

---

## Session 2: Fix Download URLs & Release Documentation ✅ COMPLETE

**Goal**: Make download instructions actually work.

**Status**: ✅ Complete - Merged in PR #6

### Current State
Docs reference: `slvsx-$(uname -s)-$(uname -m)` and `slvsx-linux`, `slvsx-macos`

Actual releases are:
- `slvsx-linux.tar.gz`
- `slvsx-macos-arm64.tar.gz`
- `slvsx-macos-x86_64.tar.gz`

### Tasks

1. **Update README.md** download section
   ```bash
   # Linux
   curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-linux.tar.gz | tar xz
   sudo mv slvsx /usr/local/bin/

   # macOS (Apple Silicon)
   curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-macos-arm64.tar.gz | tar xz
   sudo mv slvsx /usr/local/bin/

   # macOS (Intel)
   curl -L https://github.com/snoble/slvsx-cli/releases/latest/download/slvsx-macos-x86_64.tar.gz | tar xz
   sudo mv slvsx /usr/local/bin/
   ```

2. **Update QUICKSTART.md** download section

3. **Update README-EASY.md**
   - Fix download URLs
   - Mark npx/brew/docker as "Coming Soon" or remove entirely

4. **Update MCP_SERVER.md** download section

5. **Update MCP_SETUP.md** download section

6. **Update docs/MCP_INTEGRATION.md** download section

7. **Verify install.sh** still works with new release format

### Verification
- [x] Run download commands from each doc file
- [x] Verify binary works after download

---

## Session 3: Fix Broken Internal Links ✅ COMPLETE

**Goal**: All internal doc links resolve to existing files.

**Status**: ✅ Complete - Merged in PR #6 (combined with Session 2)

### Tasks

1. **README.md fixes**
   - Change `examples/four_bar_linkage.json` → `examples/ai-examples/four_bar_linkage.json`
   - Either create `examples/planetary_gears_simple.json` OR change to existing gear example
   - Either create `docs/DEVELOPMENT.md` OR remove the link

2. **Create docs/DEVELOPMENT.md** (if keeping the link)
   - Development environment setup
   - How to run tests
   - PR guidelines
   - Code style

3. **Create build.sh** (referenced in QUICKSTART.md)
   ```bash
   #!/bin/bash
   set -e
   echo "Building libslvs-static..."
   cd libslvs-static
   mkdir -p build && cd build
   cmake .. -DCMAKE_BUILD_TYPE=Release
   make
   cd ../..

   echo "Building slvsx..."
   export SLVS_LIB_DIR=$PWD/libslvs-static/build
   cargo build --release

   echo "Done! Binary at: target/release/slvsx"
   ```

4. **Create examples/planetary_gears_simple.json** OR update README reference
   - Check if a similar example exists in `examples/ai-examples/gear_meshing.json`
   - Either create the missing file or update README to point to existing example

### Verification
- [x] Click every internal link in README.md
- [x] Click every internal link in QUICKSTART.md
- [x] Click every internal link in docs/*.md

---

## Session 4: Implement MCP Server Mode 🚧 IN PROGRESS

**Goal**: Make `slvsx mcp-server` command work as documented.

**Status**: 🚧 In Progress - Implemented in PR #8 (open), needs testing

### Context
The docs describe MCP server functionality that doesn't exist yet. Rather than remove the docs, we implement the feature.

### Tasks

1. **Implement `slvsx mcp-server` command** ✅
   - Add MCP server subcommand to CLI
   - Use stdio transport (standard for local MCP servers)
   - Implement MCP protocol handshake

2. **Implement MCP tools** ✅
   - `solve_constraints` - Solve a constraint system (returns JSON)
   - `validate_constraints` - Check validity without solving
   - `render_solution` - Return SVG/PNG image inline (agent can see it!)
   - `export_solution` - Export to SVG/DXF/STL formats (returns file content)
   - `get_capabilities` - List supported constraint types

3. **Implement MCP resources (searchable docs)** ⏳
   - Expose documentation as MCP resources
   - Include: constraint types, JSON schema, examples
   - Enable AI to search/read docs through MCP protocol

4. **Create crates/cli/src/mcp.rs** ✅
   - MCP protocol handler
   - JSON-RPC message handling
   - Tool dispatch
   - Resource serving

5. **Test with Claude Desktop** ⏳
   - Add to Claude Desktop config
   - Verify tools appear
   - Test constraint solving through MCP
   - Verify docs are searchable

6. **Update mcp-server.js prototype** ⏳
   - Either remove (replaced by Rust implementation)
   - Or keep as reference/alternative

### Verification
- [ ] `slvsx mcp-server` starts and responds to MCP protocol
- [ ] Tools work in Claude Desktop
- [ ] Docs are searchable through MCP resources
- [ ] All MCP docs are accurate

---

## Session 5: Distribution Packages 🚧 IN PROGRESS

**Goal**: Implement the distribution methods documented in README-EASY.md.

**Status**: 🚧 In Progress - Implemented in separate branches (feature/distribution-packages), needs PR

### Tasks

1. **NPM Package**
   - Create package.json with bin entry
   - Bundle static binaries for each platform
   - Publish to npm as `slvsx`
   - Test `npx slvsx solve`

2. **Homebrew Tap**
   - Create `homebrew-slvsx` repo
   - Add formula pointing to GitHub releases
   - Test `brew install snoble/slvsx/slvsx`

3. **Docker Image**
   - Create optimized Dockerfile
   - Publish to ghcr.io/snoble/slvsx
   - Test `docker run ghcr.io/snoble/slvsx`

4. **Update install.sh**
   - Auto-detect platform
   - Download correct binary
   - Install to ~/.local/bin

### Verification
- [ ] `npx slvsx --version` works
- [ ] `brew install snoble/slvsx/slvsx` works
- [ ] `docker run ghcr.io/snoble/slvsx --version` works
- [ ] `curl ... | bash` installer works

---

## Session 6: Preserve and Consolidate Historical Documentation ✅ COMPLETE

**Goal**: Preserve valuable debugging insights while cleaning up obsolete docs.

**Status**: ✅ Complete - Done in docs/preserve-history branch (preserved docs in archive/, created HISTORY.md and TROUBLESHOOTING.md)

### Review These Files

| File | Purpose | Action |
|------|---------|--------|
| CHANGE_INVENTORY.md | Historical debugging | Archive or delete |
| CI_INVESTIGATION.md | Historical debugging | Archive or delete |
| CURRENT_STATUS.md | Historical status | Archive or delete |
| KNOWN_ISSUES.md | Issues (all marked FIXED) | Update or archive |
| SIGABRT_FIX.md | Historical fix | Archive or delete |
| SOLUTION_FOUND.md | Historical debugging | Archive or delete |
| docs/SIGABRT_ISSUE.md | Historical issue | Archive or delete |

### Tasks

1. **Review each file** - determine if still relevant ✅
2. **Archive historical docs** - move to `docs/archive/` or delete ✅
3. **Update KNOWN_ISSUES.md** - remove FIXED items or archive ✅
4. **Extract lessons** - Created HISTORY.md and TROUBLESHOOTING.md ✅

### Verification
- [x] Root directory is cleaner
- [x] Historical insights preserved in docs/archive/
- [x] Key lessons extracted to consolidated docs

---

## Session 7: Final Verification & Testing 🚧 IN PROGRESS

**Goal**: Verify everything works end-to-end.

**Status**: 🚧 In Progress - Comprehensive tests added in PR #8, validation improvements in PR #9

### Tasks

1. **Test download flow**
   - Follow README instructions from scratch
   - Verify binary works

2. **Test build flow**
   - Follow docs/BUILDING.md
   - Verify build.sh works

3. **Test coverage**
   - Run `cargo llvm-cov` locally
   - Verify Codecov receives data

4. **Test PR workflow**
   - Create test branch
   - Open PR
   - Verify CI runs
   - Verify Codecov comments
   - Verify branch protection works

5. **Link audit**
   - Automated: Run link checker on all .md files
   - Manual: Click-test critical paths

### Verification
- [x] New user can download and use slvsx
- [x] Developer can build from source
- [x] PR workflow enforces quality gates
- [x] All documentation links work
- [x] Comprehensive test coverage added
- [x] Validation improvements implemented

---

## Quick Reference: File Changes by Session

| Session | Focus | Files Created/Modified |
|---------|-------|----------------------|
| 1 | Branch Protection + Codecov | codecov.yml, COVERAGE.md, build.yml, README.md |
| 2 | Fix Download URLs | README.md, QUICKSTART.md, MCP_*.md |
| 3 | Fix Internal Links | docs/DEVELOPMENT.md, build.sh, README.md |
| 4 | Implement MCP Server | crates/cli/src/mcp.rs, CLI changes |
| 5 | Distribution Packages | package.json, Dockerfile, homebrew formula |
| 6 | Cleanup Old Docs | Archive/delete 7 obsolete files |
| 7 | Final Verification | Testing all flows |

---

## Dependencies Between Sessions

```
Session 1 (Branch Protection + Codecov) ✅ DONE (PR #5)
    ↓
Session 2 (Download URLs) ✅ DONE (PR #6)
    ↓
Session 3 (Internal Links) ✅ DONE (PR #6)
    ↓
Session 4 (MCP Server) 🚧 IN PROGRESS (PR #8 - open)
    ↓
Session 5 (Distribution) 🚧 IN PROGRESS (branches ready)
    ↓
Session 6 (Preserve Docs) ✅ DONE (docs/preserve-history)
    ↓
Session 7 (Final Verification) 🚧 IN PROGRESS (PR #8, #9)
```

Sessions 2-3 can be done together. Sessions 4-5 are independent features.

---

## Estimated Effort

| Session | Complexity | Status |
|---------|------------|--------|
| 1 | Medium | ✅ DONE - Branch protection + Codecov (PR #5) |
| 2 | Low | ✅ DONE - Simple URL fixes (PR #6) |
| 3 | Low-Medium | ✅ DONE - Created docs/DEVELOPMENT.md, build.sh (PR #6) |
| 4 | High | 🚧 IN PROGRESS - MCP server implemented (PR #8 open) |
| 5 | Medium | 🚧 IN PROGRESS - NPM/Homebrew/Docker ready (branches exist) |
| 6 | Low | ✅ DONE - Preserved docs in archive/, created HISTORY.md |
| 7 | Low | 🚧 IN PROGRESS - Tests added (PR #8), validation (PR #9) |

Total: ~7 sessions. Sessions 4-5 are substantial feature work.

---

## Session 8: Implement Missing Constraints for Tutorial Examples

**Goal**: Implement constraints needed for classic SolveSpace tutorial examples.

### Missing Constraints Identified

While implementing SolveSpace tutorial examples, the following constraints were found to be missing:

1. **Angle** - Constrain angle between two lines/entities
2. **Horizontal** - Constrain line to be horizontal
3. **Vertical** - Constrain line to be vertical
4. **EqualLength** - Constrain multiple lines to have equal length
5. **EqualRadius** - Constrain circles to have equal radius
6. **Tangent** - Constrain line/circle to be tangent to circle/arc
7. **PointOnCircle** - Constrain point to lie on circle
8. **Symmetric** - Constrain entities to be symmetric about a line/point
9. **Midpoint** - Constrain point to be midpoint of line

### Tasks

1. **Implement Angle constraint**
   - Add FFI mapping in `constraint_registry.rs`
   - Support angle between two lines
   - Support angle value in degrees or radians
   - Add tests

2. **Implement Horizontal/Vertical constraints**
   - Add FFI mapping for horizontal line constraint
   - Add FFI mapping for vertical line constraint
   - Add tests

3. **Implement EqualLength constraint**
   - Add FFI mapping for multiple lines
   - Support 2+ lines in constraint
   - Add tests

4. **Implement remaining constraints** (as needed for examples)
   - EqualRadius
   - Tangent
   - PointOnCircle
   - Symmetric
   - Midpoint

### Tutorial Examples Created

- `17_four_bar_linkage.json` - Requires Angle constraint
- `18_simple_rectangle.json` - Requires Horizontal/Vertical constraints
- `19_parametric_square.json` - Requires Horizontal/Vertical/EqualLength constraints
- `20_slider_crank.json` - Requires Horizontal/Angle constraints

### Verification

- [ ] All tutorial examples solve successfully
- [ ] Constraints are properly validated
- [ ] Tests cover all constraint types
- [ ] Documentation updated with new constraints
