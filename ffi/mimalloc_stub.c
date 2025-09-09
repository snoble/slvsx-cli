// Stub implementations for mimalloc symbols required by libslvs
// These are dummy implementations that allow linking but don't provide actual functionality

#include <stddef.h>
#include <stdint.h>

// Dummy types
typedef struct mi_stats_t {
    size_t reserved;
    size_t committed;
    size_t reset;
    size_t purged;
    size_t page_committed;
    size_t segments;
    size_t segments_abandoned;
    size_t segments_cache;
    size_t pages;
    size_t pages_abandoned;
    size_t pages_extended;
    size_t page_no_retire;
    size_t mmap_calls;
    size_t commit_calls;
    size_t reset_calls;
    size_t purge_calls;
    size_t pages_purged;
    size_t segments_purged;
    size_t pages_reset;
    size_t segments_reset;
    size_t huge_count;
    size_t huge_peak;
    size_t giant_count;
    size_t giant_peak;
    size_t malloc_count;
    size_t normal_count;
    int64_t normal_bins[74];
} mi_stats_t;

// Global stats variable
mi_stats_t _mi_stats_main = {0};

// Stub functions
void _mi_stat_counter_increase(mi_stats_t* stats, size_t* counter, size_t amount) {
    // No-op
}

void _mi_stat_increase(mi_stats_t* stats, size_t* counter, size_t amount) {
    // No-op
}

void _mi_stat_decrease(mi_stats_t* stats, size_t* counter, size_t amount) {
    // No-op
}

void mi_stats_merge(mi_stats_t* dst, const mi_stats_t* src) {
    // No-op
}

void _mi_prim_reset(void* p, size_t size) {
    // No-op
}

// Additional stub functions that might be needed
void _mi_thread_data_collect(void) {
    // No-op
}

void _mi_heap_collect_ex(void* heap, int force) {
    // No-op
}