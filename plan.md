# SLVSX-CLI Improvement Plan

This plan breaks down the work from [spec.md](./spec.md) into sessions that can be done incrementally.

---

## Session 1: Branch Protection & Codecov Setup

**Goal**: Get CI rigor in place before making other changes.

### Tasks

1. **Configure branch protection** (GitHub settings)
   - Enable "Require a pull request before merging"
   - Enable "Require status checks to pass before merging"
   - Add required checks: `build (ubuntu-latest)`, `build (macos-latest)`
   - Optional: Require 1 approving review

2. **Create codecov.yml** in repo root
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
- [ ] Direct push to main is blocked
- [ ] PR requires CI to pass
- [ ] Coverage report appears on Codecov
- [ ] Coverage badge shows in README

---

## Session 2: Fix Download URLs & Release Documentation

**Goal**: Make download instructions actually work.

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
- [ ] Run download commands from each doc file
- [ ] Verify binary works after download

---

## Session 3: Fix Broken Internal Links

**Goal**: All internal doc links resolve to existing files.

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
- [ ] Click every internal link in README.md
- [ ] Click every internal link in QUICKSTART.md
- [ ] Click every internal link in docs/*.md

---

## Session 4: MCP Documentation Cleanup

**Goal**: MCP docs focus on local CLI usage, remove references to unimplemented features.

### Context
User requirement: MCP should be "easy to download and run locally" - it's just a convenient way to run the CLI locally, not a hosted server.

### Tasks

1. **Consolidate MCP docs**
   - Currently have: MCP_SERVER.md, MCP_SETUP.md, docs/MCP_INTEGRATION.md
   - Consider merging into single clear document

2. **Remove references to unimplemented features**
   - `slvsx mcp-server` command (not implemented)
   - `crates/cli/src/mcp.rs` (doesn't exist)
   - `mcp_server/slvsx_mcp.py` (doesn't exist)
   - NPM package (doesn't exist)

3. **Focus on what works today**
   - Subprocess-based integration
   - Python/Node.js examples calling CLI
   - JSON in, JSON out

4. **Update MCP_SERVER.md**
   - Change title to something like "Using SLVSX with AI Agents"
   - Remove MCP server mode claims
   - Keep subprocess examples
   - Mark future MCP server as "Planned"

5. **Update MCP_SETUP.md**
   - Remove Python wrapper reference
   - Focus on CLI usage

6. **Update docs/MCP_INTEGRATION.md**
   - Remove mcp.rs reference
   - Update status section to be accurate

7. **Decide fate of mcp-server.js**
   - It exists as a prototype
   - Either document it properly or remove it

### Verification
- [ ] No docs claim features that don't exist
- [ ] All code examples in MCP docs work
- [ ] Clear distinction between "works today" and "planned"

---

## Session 5: Cleanup Obsolete Documentation

**Goal**: Remove or archive docs that are no longer needed.

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

1. **Review each file** - determine if still relevant
2. **Archive historical docs** - move to `docs/archive/` or delete
3. **Update KNOWN_ISSUES.md** - remove FIXED items or archive
4. **Update CLAUDE.md** - ensure it references current state

### Verification
- [ ] Root directory is cleaner
- [ ] No outdated status docs confuse readers

---

## Session 6: Final Verification & Testing

**Goal**: Verify everything works end-to-end.

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
- [ ] New user can download and use slvsx
- [ ] Developer can build from source
- [ ] PR workflow enforces quality gates
- [ ] All documentation links work

---

## Quick Reference: File Changes by Session

| Session | Files Created | Files Modified |
|---------|--------------|----------------|
| 1 | codecov.yml, COVERAGE.md | build.yml, README.md |
| 2 | - | README.md, QUICKSTART.md, README-EASY.md, MCP_*.md |
| 3 | docs/DEVELOPMENT.md, build.sh | README.md |
| 4 | - | MCP_SERVER.md, MCP_SETUP.md, docs/MCP_INTEGRATION.md |
| 5 | - | Delete/archive 7 files |
| 6 | - | Any remaining fixes |

---

## Dependencies Between Sessions

```
Session 1 (Branch Protection + Codecov)
    ↓
Session 2 (Download URLs) ←→ Session 3 (Internal Links)
    ↓                            ↓
Session 4 (MCP Cleanup) ←→ Session 5 (Obsolete Docs)
    ↓
Session 6 (Final Verification)
```

Sessions 2-5 can be done in parallel or any order. Session 1 should be first, Session 6 should be last.

---

## Estimated Effort

| Session | Complexity | Notes |
|---------|------------|-------|
| 1 | Medium | Requires GitHub admin access |
| 2 | Low | Simple find/replace |
| 3 | Low-Medium | May need to create docs/DEVELOPMENT.md |
| 4 | Medium | Requires careful rewriting |
| 5 | Low | Mostly deletion |
| 6 | Low | Testing and verification |

Total: ~6 focused sessions, can be spread across multiple days.
