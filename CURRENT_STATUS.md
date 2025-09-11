# Current Status - SLVSX CLI

## Problem Summary
The SLVSX CLI is experiencing critical crashes that make it unusable. While constraint solving produces correct results, the program crashes with SIGABRT during cleanup, causing all tests to fail.

## Root Cause
Memory allocator conflict between libslvs-static and system libraries:
- libslvs-static was using mimalloc for memory allocation
- System C++ library uses standard malloc/free
- Mixing allocators causes SIGABRT during cleanup

## Attempted Fixes

### 1. ✅ Fixed ID Remapping Issues
- Fixed incorrect ID remapping in `real_slvs_wrapper.c`
- Fixed constraints now use entity IDs directly
- Distance constraints now create parameters as required

### 2. ❌ Removed mimalloc 
- Removed mimalloc from libslvs-static to fix allocator conflicts
- This fixed SIGABRT but broke constraint solving ("Cannot find handle" errors)
- Root cause: Removing mimalloc exposed existing ID mapping bugs

### 3. ⚠️ Current State
- Constraint solving works and produces correct output
- Program crashes with SIGABRT (exit code 134) during cleanup
- All 17 test examples fail due to non-zero exit code
- The CLI is functionally correct but unusable due to crashes

## Test Results
```
Test Results:
  Total:  17
  Passed: 0
  Failed: 17
```

Example output:
```json
{
  "status": "ok",
  "diagnostics": {
    "iters": 1,
    "residual": 0.0,
    "dof": 0,
    "time_ms": 1
  },
  "entities": {
    "origin": {
      "at": [0.0, 0.0, 0.0]
    }
  }
}
```
Exit code: 134 (SIGABRT)

## Next Steps

### Option 1: Fix Memory Management
- Investigate arena-style allocator implementation
- Ensure all allocations use consistent allocator
- May require deeper changes to libslvs-static

### Option 2: Work Around Cleanup
- Suppress SIGABRT during exit
- Use `_exit()` instead of normal cleanup
- Not ideal but would make CLI usable

### Option 3: Alternative Memory Allocator
- Try jemalloc or tcmalloc instead of mimalloc
- May avoid conflicts while maintaining performance

## Critical Notes
- **The project is currently unusable with this bug**
- CI should fail until this is fixed (no workarounds)
- Constraint solving logic is correct - only cleanup is broken