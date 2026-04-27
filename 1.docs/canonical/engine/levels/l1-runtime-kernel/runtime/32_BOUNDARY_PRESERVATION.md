# Boundary Preservation

## Canonical Rule

`engine_runtime` provides execution constitution and global parallel ownership. Execution ownership must be centralized to maintain one phase law, one scheduling law, one apply law, and one low-latency law per world instance.

## Upward Boundary

**Exports to layers above:**
- Phase contracts (ingress, read, compute, resource, authority-sync, stage, apply, egress, diagnostics)
- Execution windows (scheduled execution windows)
- Staged outputs (staged apply ordering)
- Pacing decisions (frame pacing)
- Ordered publication decisions (queue and publication law)
- Events (runtime events)
- Diagnostics (runtime diagnostics)

**Canonical consumers:**
- `engine_runtime_headless` — Headless runtime profile
- `engine_runtime_realtime` — Realtime runtime profile
- `engine_stream_control` — Stream control services
- `engine_memory_control` — Memory control services
- `engine_net_transport` — Network transport
- `engine_net_sync` — Network sync
- `engine_net_latency` — Network latency
- `engine_startup` — Startup orchestration

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_handle` — Stable references
- `engine_ecs` — ECS substrate
- `engine_world` — World truth boundary
- `engine_world_region` — Region substrate for partition-aware scheduling

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- World mutation bypassing apply boundary
- Unbounded micro-segmentation to hide spikes
- Family-owned apply ordering
- Nested global scheduler inside family or service crate
- Queue growth hidden behind helper abstractions
- Barrier insertion outside runtime law
- Broad locks across compute phases
- Hidden family-local queues outliving runtime visibility
- Tiny depth with unbounded queue age
- Queue spillover into unrelated allocators without runtime ownership
- Queue-age compliance faked by rotating hidden buffers
- Upload debt hidden in later frames
- Low-latency path widened by diagnostics or replay work
- Stale-frame reuse beyond ceiling
- Upload spill into hidden queued frames
- Queue creep in present queue
- Readback in critical path
- Service-owned publish pacing
