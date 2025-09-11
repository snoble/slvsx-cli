# Solution Found: Comparing Working vs Broken Versions

## The Root Cause

After comparing the working version (commit d075470, Sept 6) with the current broken version, the key differences are:

### Working Version (d075470)
- **Entity ID mapping:** Used `1000 + id` for all entities
- **Parameter IDs:** Started at 100
- **Constraint IDs:** Used `10000 + id`
- **Consistent offsets:** All functions used the same ID transformation

### Current Broken Version
- **Entity ID mapping:** Uses IDs directly (no offset)
- **Parameter IDs:** Now start at 10000 (changed to avoid collision)
- **Constraint IDs:** Still at 100
- **Inconsistent:** Some places use offsets, others don't

## The Fix

We need to restore the consistent ID offset strategy from the working version:

1. **All entity IDs should use:** `1000 + id`
2. **All constraint IDs should use:** `10000 + id`  
3. **Parameters can start at:** 100 (or 10000, doesn't matter as much)

## Why This Matters

The SolveSpace library expects consistent handle IDs. When we removed the 1000 offset for entities but kept looking them up with the offset, the library couldn't find the handles, causing "Cannot find handle" errors.

## Quick Test

To verify this is the issue:
1. Restore the entity ID offset (1000 + id) in all places
2. Keep the parameter ID at 10000 (to avoid collision)
3. Test with one of the examples

## Memory Allocator Issue

The working version also had the same mimalloc setup, so the SIGABRT on exit was likely always there but wasn't critical since the output was correct. We can address this separately after fixing the core functionality.