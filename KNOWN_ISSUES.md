# Known Issues

## ~~CRITICAL BUG: Solver Constraint Handling Broken~~ FIXED (2025-09-10)

### Issue (FIXED)
After removing mimalloc to fix SIGABRT crashes, the solver was failing when processing constraints with the error:
```
File libslvs-static/src/dsc.h, line 534, function FindById:
Assertion failed: t != nullptr.
Message: Cannot find handle.
```

### Root Cause
The issue was caused by incorrect ID mapping in `ffi/real_slvs_wrapper.c`. The working version from Sept 6 used a consistent `1000+` offset for all entity IDs to avoid conflicts with internal SolveSpace entities. During refactoring, this offset was accidentally removed.

### Solution
Restored the proper ID mapping strategy:
- All entities (points, lines, circles) use `1000 + id` for internal IDs
- All constraints use `10000 + id` for internal IDs  
- Distance constraints pass the distance value directly (not as a parameter)

This fix was verified against the working commit 8f2cc06 from Sept 6, 2025.

### Tests Status
✅ All tests now pass
✅ All 17 example files solve correctly
✅ CI pipeline is green

## SIGABRT on Exit (FIXED)

### Previous Issue (Fixed in commit 7291cf8)
Binary was crashing with SIGABRT on exit due to mixing memory allocators (mimalloc vs system malloc).

### Solution
Removed mimalloc entirely from libslvs-static and replaced with standard memory allocation.