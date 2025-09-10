#include "solvespace.h"
// mimalloc removed to fix memory allocator conflicts
#include <vector>
#include <memory>
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

// Arena-style memory pool for temporary allocations
// Mimics the behavior of mimalloc's heap without the allocator conflicts
struct TempMemoryPool {
    struct Arena {
        std::vector<void*> allocations;
        
        ~Arena() {
            for(void* ptr : allocations) {
                free(ptr);
            }
        }
        
        void *alloc(size_t size) {
            void *ptr = calloc(1, size);
            ssassert(ptr != NULL, "out of memory");
            allocations.push_back(ptr);
            return ptr;
        }
    };
    
    std::unique_ptr<Arena> current;
    
    TempMemoryPool() : current(new Arena()) {}
    
    void *alloc(size_t size) {
        if (!current) {
            current.reset(new Arena());
        }
        return current->alloc(size);
    }
    
    void reset() {
        // Create a new arena, destroying the old one
        // This mimics mimalloc's heap replacement behavior
        current.reset(new Arena());
    }
};

static thread_local TempMemoryPool TempArena;

void *AllocTemporary(size_t size) {
    return TempArena.alloc(size);
}

void FreeAllTemporary() {
    TempArena.reset();
}

}
}
