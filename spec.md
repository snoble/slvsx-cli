# SLVSX-CLI Improvements Specification

## Overview

This document specifies the work needed to:
1. Add branch protection for the `main` branch
2. Integrate Codecov for test coverage reporting
3. Audit and fix all broken documentation links

---

## 1. Branch Protection Requirements

### Current State
- No branch protection rules configured on `main`
- Direct pushes to `main` are allowed
- No required checks before merging

### Target State
- Require pull requests before merging to `main`
- Require CI status checks to pass before merge
- Require at least 1 approval (optional, discuss with team)
- No force pushes to `main`

### Implementation
Configure via GitHub API or web UI:
```bash
# Example using gh CLI (may require admin permissions)
gh api repos/snoble/slvsx-cli/branches/main/protection \
  -X PUT \
  -H "Accept: application/vnd.github+json" \
  -f required_status_checks='{"strict":true,"contexts":["build (ubuntu-latest)","build (macos-latest)"]}' \
  -f enforce_admins=false \
  -f required_pull_request_reviews='{"required_approving_review_count":1}' \
  -f restrictions=null
```

---

## 2. Codecov Integration Requirements

### Current State
- `CODECOV_TOKEN` secret exists in GitHub
- No codecov configuration file
- No coverage reporting in CI workflows
- Tests run but coverage not collected

### Target State
- Coverage collected during test runs
- Coverage reports uploaded to Codecov
- Coverage badge in README
- PR comments with coverage diff

### Coverage Policy

**Goal**: 100% test coverage with documented exceptions.

**Exceptions**:
1. **Provably unreachable code** - Lines that cannot be executed (e.g., exhaustive match arms required by compiler but logically impossible)
2. **Generated code** - Auto-generated files (e.g., schema bindings, FFI wrappers)

**Enforcement**:
- PRs should not decrease coverage
- New code must have tests
- Exceptions must be documented with `// coverage: ignore` comments

### Implementation

#### 2.1 Add codecov.yml configuration
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
        # Fail if coverage drops below 100% (accounting for ignore comments)
    patch:
      default:
        target: 100%
        # All new code must be covered

comment:
  layout: "reach,diff,flags,files"
  behavior: default

ignore:
  - "ffi/**/*"                    # Generated FFI code
  - "**/generated/**"             # Any generated directories
  - "schema/**/*"                 # Schema files
```

#### 2.2 Create COVERAGE.md policy document
Document the coverage policy for contributors:
- 100% coverage goal
- How to mark exceptions
- How to run coverage locally

#### 2.3 Modify build.yml workflow
Add to test step:
```yaml
- name: Install cargo-llvm-cov
  uses: taiki-e/install-action@cargo-llvm-cov

- name: Run tests with coverage
  run: cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
  env:
    RUST_TEST_THREADS: 1

- name: Upload coverage to Codecov
  uses: codecov/codecov-action@v4
  with:
    token: ${{ secrets.CODECOV_TOKEN }}
    files: lcov.info
    fail_ci_if_error: false
```

---

## 3. Documentation Audit Results

### 3.1 Broken Internal Links

| File | Broken Link | Issue | Fix |
|------|-------------|-------|-----|
| README.md | `docs/DEVELOPMENT.md` | File doesn't exist | Create file or remove link |
| README.md | `examples/01_first_point.json` | Correct, exists | OK |
| README.md | `examples/four_bar_linkage.json` | Wrong path | Fix: `examples/ai-examples/four_bar_linkage.json` |
| README.md | `examples/planetary_gears_simple.json` | File doesn't exist | Create file or remove link |
| QUICKSTART.md | `./build.sh` | File doesn't exist | Create file (references install.sh which exists) |
| MCP_INTEGRATION.md | `mcp-server.js` prototype | Exists but path not specified | Clarify path |
| MCP_INTEGRATION.md | `crates/cli/src/mcp.rs` | File doesn't exist | Remove reference (not implemented) |
| MCP_SETUP.md | `mcp_server/slvsx_mcp.py` | Directory doesn't exist | Remove reference (not implemented) |

### 3.2 Broken External Links / Download URLs

| File | Issue | Current | Should Be |
|------|-------|---------|-----------|
| README.md, QUICKSTART.md, MCP_*.md | Download URL format wrong | `slvsx-$(uname -s)-$(uname -m)` | Actual releases are `slvsx-linux.tar.gz`, `slvsx-macos-arm64.tar.gz`, `slvsx-macos-x86_64.tar.gz` |
| MCP_SETUP.md | Wrong binary names | `slvsx-linux`, `slvsx-macos` | Should reference .tar.gz archives |

### 3.3 Documentation for Non-Existent Features

| File | Feature Documented | Status |
|------|-------------------|--------|
| README-EASY.md | `npx slvsx` | NPM package doesn't exist |
| README-EASY.md | `brew tap snoble/slvsx` | Homebrew tap doesn't exist |
| README-EASY.md | `docker run ghcr.io/snoble/slvsx` | Docker image doesn't exist |
| QUICKSTART.md | `slvsx mcp-server` command | Not implemented in CLI |
| MCP_SERVER.md | MCP server mode | Not implemented |
| MCP_SETUP.md | Python MCP wrapper | Not implemented |

### 3.4 MCP Documentation Strategy

**User Requirement**: MCP should focus on local usage, not hosted server.

**Current Problem**: Docs describe a full MCP server implementation that doesn't exist and suggest hosted/cloud patterns.

**Proposed Fix**:
1. Keep MCP docs minimal and focused on local CLI usage
2. Remove references to unimplemented MCP server features
3. Focus on subprocess-based integration (which works today)
4. Mark future MCP server as "planned" clearly

---

## 4. Files to Create

### 4.1 docs/DEVELOPMENT.md
Development guide covering:
- Setting up development environment
- Running tests locally
- Code style guidelines
- PR process

### 4.2 build.sh (optional)
Simple build script wrapping the existing process:
```bash
#!/bin/bash
set -e
cd libslvs-static && mkdir -p build && cd build && cmake .. -DCMAKE_BUILD_TYPE=Release && make && cd ../..
export SLVS_LIB_DIR=$PWD/libslvs-static/build
cargo build --release
```

### 4.3 examples/planetary_gears_simple.json
Either create this file or update README.md to reference an existing gear example.

### 4.4 codecov.yml
Codecov configuration file.

### 4.5 COVERAGE.md
Coverage policy document explaining:
- 100% coverage goal
- Exception policy (unreachable code, generated code)
- How to use `// coverage: ignore` comments
- How to run coverage locally with `cargo llvm-cov`

---

## 5. Files to Update

### 5.1 README.md
- Fix link to four_bar_linkage.json
- Fix or remove link to planetary_gears_simple.json
- Fix or remove link to docs/DEVELOPMENT.md
- Update download URLs to match actual release assets

### 5.2 QUICKSTART.md
- Fix install.sh URL (looks correct)
- Remove or update mcp-server reference
- Remove build.sh reference or create the file

### 5.3 README-EASY.md
- Mark npx/brew/docker as "Coming Soon" or remove
- Fix download instructions

### 5.4 MCP_SERVER.md
- Rewrite to focus on local CLI usage
- Remove unimplemented MCP server references
- Keep subprocess examples (they work)
- Mark MCP server as "planned"

### 5.5 MCP_SETUP.md
- Remove Python MCP wrapper reference
- Focus on direct CLI usage
- Update download URLs

### 5.6 MCP_INTEGRATION.md
- Remove crates/cli/src/mcp.rs reference
- Focus on current working subprocess approach
- Update download URLs

### 5.7 .github/workflows/build.yml
- Add coverage collection
- Add Codecov upload step

---

## 6. External Configuration (GitHub)

### 6.1 Branch Protection (via UI or API)
- Enable required status checks
- Require PR reviews
- Disable force push to main

### 6.2 Codecov (already done)
- CODECOV_TOKEN secret already exists

---

## Summary Metrics

| Category | Count |
|----------|-------|
| Broken internal links | 7 |
| Broken download URLs | 4+ files affected |
| Non-existent features documented | 6 |
| Files to create | 5 |
| Files to update | 8+ |
| Workflow changes | 1 (build.yml) |
| GitHub settings changes | 1 (branch protection) |
