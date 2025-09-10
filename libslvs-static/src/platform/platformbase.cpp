#include "solvespace.h"
// mimalloc removed to fix memory allocator conflicts
#include <vector>
#include <cstdlib>

#if defined(WIN32)
#   include <Windows.h>
#endif // defined(WIN32)

namespace SolveSpace {
namespace Platform {

//-----------------------------------------------------------------------------
// Debug output, on Windows.
//-----------------------------------------------------------------------------

#if defined(WIN32)

#if !defined(_alloca)
// Fix for compiling with MinGW.org GCC-6.3.0-1
#define _alloca alloca
#include <malloc.h>
#endif

void DebugPrint(const char *fmt, ...)
{
    va_list va;
    va_start(va, fmt);
    int len = _vscprintf(fmt, va) + 1;
    va_end(va);

    va_start(va, fmt);
    char *buf = (char *)_alloca(len);
    _vsnprintf(buf, len, fmt, va);
    va_end(va);

    // The native version of OutputDebugString, unlike most others,
    // is OutputDebugStringA.
    OutputDebugStringA(buf);
    OutputDebugStringA("\n");

#ifndef NDEBUG
    // Duplicate to stderr in debug builds, but not in release; this is slow.
    fputs(buf, stderr);
    fputc('\n', stderr);
#endif
}

#endif

//-----------------------------------------------------------------------------
// Debug output, on *nix.
//-----------------------------------------------------------------------------

#if !defined(WIN32)

void DebugPrint(const char *fmt, ...) {
    va_list va;
    va_start(va, fmt);
    vfprintf(stderr, fmt, va);
    fputc('\n', stderr);
    va_end(va);
}

#endif

//-----------------------------------------------------------------------------
// Temporary arena.
//-----------------------------------------------------------------------------

// Simple memory pool for temporary allocations
// Replaced mimalloc with standard allocation to fix memory conflicts
struct TempMemoryPool {
    std::vector<void*> allocations;
    
    ~TempMemoryPool() {
        for(void* ptr : allocations) {
            free(ptr);
        }
    }
    
    void clear() {
        for(void* ptr : allocations) {
            free(ptr);
        }
        allocations.clear();
    }
};

static thread_local TempMemoryPool TempArena;

void *AllocTemporary(size_t size) {
    void *ptr = calloc(1, size);
    ssassert(ptr != NULL, "out of memory");
    TempArena.allocations.push_back(ptr);
    return ptr;
}

void FreeAllTemporary() {
    TempArena.clear();
}

}
}
