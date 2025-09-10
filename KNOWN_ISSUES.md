# Known Issues

## CRITICAL BUG: Solver Constraint Handling Broken (2025-09-10)

### Issue
After removing mimalloc to fix SIGABRT crashes, the solver now fails when processing constraints with the error:
```
File libslvs-static/src/dsc.h, line 534, function FindById:
Assertion failed: t != nullptr.
Message: Cannot find handle.
```

### Impact
**⚠️ THE PROJECT IS CURRENTLY UNUSABLE FOR ITS MAIN PURPOSE ⚠️**
- Basic validation and export work correctly
- Simple solving without constraints works
- **Complex constraint solving FAILS in 16 out of 17 test examples**
- **This breaks the core functionality of the constraint solver**

### Root Cause
The memory allocator changes may have affected how handles are stored/retrieved in the solver's internal data structures. The TempMemoryPool replacement for mimalloc's heap may not be preserving some data correctly across solver operations.

### Fix Priority
**CRITICAL - MUST FIX IMMEDIATELY**

No other work should proceed until this is fixed. The project is non-functional without constraint solving.

### Investigation Needed
1. How the solver manages entity handles
2. Whether the TempMemoryPool needs different lifecycle management  
3. If there are other mimalloc-specific behaviors we need to replicate
4. Consider reverting to mimalloc but fixing the SIGABRT differently (e.g., proper shutdown sequence)

### Tests Affected
- test-examples.sh (16 out of 17 examples fail)
- Unit tests pass (they don't use complex constraints)

## SIGABRT on Exit (FIXED)

### Previous Issue (Fixed in commit 7291cf8)
Binary was crashing with SIGABRT on exit due to mixing memory allocators (mimalloc vs system malloc).

### Solution
Removed mimalloc entirely from libslvs-static and replaced with standard memory allocation.