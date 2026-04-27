# Boundary Preservation

## Canonical Rule

`engine_memory_control` preserves only its explicit runtime resource ownership class and may not absorb execution law, world truth ownership, or unrelated service ownership.

## Upward Boundary

**Exports to layers above:**
- Allocator results (allocation success/failure)
- Pool state (memory pool status)
- Reclaim requests (memory reclaim operations)
- Memory diagnostics (allocation metrics and pressure state)
- Allocation descriptors (memory allocation descriptors)
- Request batches (batched allocation requests)
- Result batches (batched allocation results)

**Canonical consumers:**
- `engine_stream_control` — Stream control services
- `engine_transfer_control` — Transfer control services
- `engine_residency_control` — Residency control services
- Higher simulation families requiring memory allocation
- Service layers requiring memory allocation

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_handle` — Stable runtime handles
- `engine_storage_layout` — Storage and page layout descriptors
- `engine_world` — World lifetime anchors
- `engine_runtime` — Runtime legality windows and budget classes

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Hidden fallback to general-purpose heap in critical execution lane
- Moving tick-scratch debt into session-persistent without explicit canon rule
- Fence-bound staging memory retained past release visibility
- Allocator pressure silently spilling into unrelated classes
- Class-ceiling breach without entering canonical response ladder
- Frame-scratch surviving past frame end
- Tick-scratch surviving past authoritative tick boundary
- Cell-scratch surviving past local region/chunk work closure
- General allocator traffic in steady-state critical execution lanes
- Transient work using non-owning arena or pool classes
- Per-class ceiling violations beyond numeric source of truth
- Pressure response latency exceeding 2 realtime frames or 2 headless ticks
