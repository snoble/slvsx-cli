# Change Inventory and Impact Analysis

## Summary of Key Findings

**CRITICAL DISCOVERY:** The CLI **WAS WORKING** on September 6, 2025!
- Commit d075470 "Fix libslvs integration and get examples working" had the CLI fully functional
- All example SVGs were successfully generated on Sept 6-7 using the actual CLI
- The working version was lost during the Sept 7-9 refactoring to remove mock solver

The CLI regression timeline:
- **Sept 6:** ✅ WORKING - Generated all example outputs successfully (commit d075470)
- **Sept 7-9:** ❌ BROKEN - Major refactor removed mock solver, broke FFI wrapper
- **Sept 10:** ❌ STILL BROKEN - Attempts to fix made things worse

## Timeline of Changes and Their Impacts

### Phase 0: WORKING VERSION (Sept 6)
**Commit:** d075470
- **Changes:** Fixed FFI wrapper memory allocation (dragged/failed arrays)
- **Impact:** All examples solved correctly with real libslvs
- **CI Status:** Not yet in CI, but CLI worked locally
- **Evidence:** Generated all SVG outputs that are still in the repo

### Phase 1: Initial FFI Integration (Sept 7-9)
**Commits:** 3a93972 to 9124d6a
- **Changes:** Switched from mock solver to real libslvs-static fork
- **Impact:** Tests started crashing immediately
- **CI Status:** ❌ Failing - build issues and test crashes

### Phase 2: Attempting to Fix Test Crashes (Sept 9)
**Commits:** afacaea to 088b8f1
- **Changes:** 
  - Added workplane initialization
  - Fixed group field assignment issues
  - Uploaded artifacts even on failure
- **Impact:** Some CI runs succeeded but tests still crashed
- **CI Status:** ✅/❌ Mixed - some builds passed, tests failed

### Phase 3: FFI Wrapper Fixes (Sept 9)
**Commits:** 230e2ad to d7caf8c
- **Changes:** Rewrote real_slvs_wrapper.c multiple times
- **Impact:** Tests "passed functionally" but still had SIGABRT
- **CI Status:** ✅ Success (but using workarounds)

### Phase 4: SIGABRT Workaround (Sept 10)
**Commits:** 00acdd4 to f888122
- **Changes:** Added `|| true` to ignore SIGABRT in CI
- **Impact:** CI showed green but problem wasn't fixed
- **CI Status:** ✅ Success (false positive - hiding crashes)

### Phase 5: Memory Allocator Fix Attempt (Sept 10)
**Commit:** 7291cf8
- **Changes:** Removed mimalloc from libslvs-static
- **Impact:** 
  - ✅ Fixed SIGABRT crashes
  - ❌ Broke constraint solving ("Cannot find handle")
- **CI Status:** ✅ Success (but examples broken)

### Phase 6: Re-enabling Tests (Sept 10)
**Commit:** 15686c1
- **Changes:** Removed CI workarounds to expose real issues
- **Impact:** CI now properly fails showing constraint solver is broken
- **CI Status:** ❌ Failing - constraint solver broken

### Phase 7: Local Fixes (Uncommitted)
**Commits:** ae5e103 to cec59d0
- **Changes:** 
  - Fixed ID remapping bugs in FFI wrapper
  - Fixed parameter/constraint ID collisions
- **Impact:** 
  - ✅ Constraint solving produces correct output
  - ❌ SIGABRT crashes returned
- **CI Status:** Not pushed yet

## Root Cause Analysis

### The Memory Allocator Conflict
1. **Original Setup:** libslvs-static uses mimalloc, system uses standard malloc
2. **Problem:** Memory allocated by one allocator is freed by another → SIGABRT
3. **Failed Fix:** Removing mimalloc broke constraint solving

### The ID Management Issues
1. **FFI Wrapper Bugs:** Incorrect ID remapping (adding 1000 to entity IDs)
2. **ID Collisions:** Parameters and constraints both started at ID 100
3. **Handle Lookup Failures:** Broken ID mapping caused "Cannot find handle"

## Key Insights

1. **The CLI WAS working on Sept 6** - commit d075470 successfully generated all examples
2. **The Sept 7-9 refactor broke everything** - removing mock solver introduced new bugs
3. **CI "successes" were false positives** - using `|| true` to ignore crashes
4. **Two separate issues conflated:**
   - Memory allocator incompatibility (causes SIGABRT)
   - ID management bugs (causes constraint solver failures)
5. **Fixing one breaks the other** - removing mimalloc fixed crashes but broke solver

## Recommendations

### Option 1: Fix mimalloc Integration Properly
- Keep mimalloc but ensure all memory operations use consistent allocator
- May require wrapping all C++ allocations/deallocations
- Most complex but preserves original design

### Option 2: Replace mimalloc with Compatible Allocator
- Find allocator that doesn't conflict with system malloc
- Or implement proper arena allocator that doesn't cross boundaries
- Medium complexity

### Option 3: Fix Memory Lifecycle Management
- Ensure memory doesn't cross allocator boundaries
- Copy data instead of passing pointers between layers
- Simplest but may have performance impact

## Recommended Action Plan

### Option A: Revert to Working Version (RECOMMENDED)
1. **Check out commit d075470** - the last known working version
2. **Compare FFI wrapper** - See what was different in the working version
3. **Cherry-pick good changes** - Keep improvements from later commits
4. **Fix forward carefully** - Address issues without breaking what works

### Option B: Fix Current Version
1. **Restore mimalloc** - Since removing it broke constraint solving
2. **Fix memory lifecycle** - Ensure allocations don't cross boundaries
3. **Keep ID fixes** - The ID remapping fixes are correct

## Next Steps

1. ✅ **Found truly working version:** commit d075470 (Sept 6)
2. **Compare working vs broken:** Diff the FFI wrapper between d075470 and current
3. **Test with mimalloc restored:** See if constraint solving works again
4. **Fix memory boundaries:** Ensure allocations stay within their allocator

## Critical Questions ANSWERED

1. ✅ **Were examples generated with CLI?** YES - commit d075470 used real CLI
2. **Is mimalloc necessary?** Appears to be - removing it broke constraint solving
3. **Can we isolate memory allocation?** This is the key to fixing SIGABRT properly