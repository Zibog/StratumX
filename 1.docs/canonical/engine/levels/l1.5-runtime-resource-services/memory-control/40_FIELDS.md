# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| frame_scratch_allocator | Frame-lifetime allocator | Required | <= 64 MiB (interactive-60), <= 64 MiB (listen-host-60), <= 8 MiB (headless-20); dies at frame end | engine_memory_control |
| tick_scratch_allocator | Tick-lifetime allocator | Required | <= 96 MiB (interactive-60), <= 128 MiB (listen-host-60), <= 128 MiB (headless-20); dies before next authoritative tick | engine_memory_control |
| cell_scratch_allocator | Cell-lifetime allocator | Required | <= 8 MiB per active chunk, <= 128/160/192 MiB aggregate by profile; dies when local region/chunk work closes | engine_memory_control |
| session_persistent_allocator | Session-lifetime allocator | Required | <= 384 MiB (interactive-60), <= 512 MiB (listen-host-60), <= 512 MiB (headless-20); survives until session shutdown or explicit trim | engine_memory_control |
| streaming_resident_allocator | Streaming-lifetime allocator | Required | <= 1,280 MiB (interactive-60), <= 1,536 MiB (listen-host-60), <= 1,152 MiB (headless-20); survives by residency law only | engine_memory_control |
| staging_backed_allocator | Staging-lifetime allocator | Required | <= 192 MiB (interactive-60), <= 256 MiB (listen-host-60), <= 64 MiB (headless-20); survives only until fence-visible release | engine_memory_control |
| allocation_request | Allocation request descriptor | Required | Must specify class, size, alignment | engine_memory_control |
| allocation_result | Allocation result descriptor | Required | Success/failure, pointer if successful | engine_memory_control |
| pool_state | Memory pool status | Required | Current usage, available capacity, pressure level | engine_memory_control |
| reclaim_request | Memory reclaim descriptor | Required | Class, amount, urgency | engine_memory_control |
| pressure_state | Pressure response state | Required | Current pressure level, active response ladder step | engine_memory_control |

## Invariant Rules

- Allocator classes are lifetime-partitioned and may not borrow legality from one another
- Frame-scratch dies at frame end and may not cross into next frame
- Tick-scratch dies before next authoritative tick and may not cross apply boundary
- Cell-scratch dies when local region/chunk work closes
- Session-persistent survives until session shutdown or explicit persistent-cache trim
- Streaming-resident survives by residency law only
- Staging-backed survives only until fence-visible release
- Per-class ceilings frozen by STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md
- Class-ceiling breach must enter canonical response ladder within <= 2 realtime frames or <= 2 headless ticks
- Pressure response follows canonical ladder: trim caches, reuse/compact pools, deny optional operations, escalate to residency demotion, fail if ceiling crossed
- Hidden fallback to general-purpose heap in critical execution lane is illegal
- Moving tick-scratch debt into session-persistent without explicit canon rule is illegal
- Fence-bound staging memory retained past release visibility is illegal
