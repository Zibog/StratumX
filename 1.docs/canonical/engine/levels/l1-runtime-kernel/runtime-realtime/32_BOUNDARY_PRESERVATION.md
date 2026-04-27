# Boundary Preservation

## Canonical Rule

`engine_runtime_realtime` provides realtime interactive runtime profile. Interactive cadence and latency-sensitive orchestration must stay explicit without polluting headless execution.

## Upward Boundary

**Exports to layers above:**
- Profile config (realtime profile definition)
- Interactive events (realtime execution progression)
- Frame outputs (presentable frames, audio submissions, remote publications)
- Diagnostics (profile diagnostics)

**Canonical consumers:**
- `engine_startup` — Startup orchestration

## Downward Boundary

**Imports from layers below:**
- `engine_runtime` — Execution constitution
- `engine_world` — World truth boundary
- `engine_ecs` — ECS substrate

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Variable presentation contract (must be fixed 60 Hz)
- Hidden wider frame entitlement (16.67 ms hard budget)
- Multi-frame latency reservoir (presentable queue <= 1 frame)
- Long-lived stale visibility reuse (visibility freshness <= 1 stale frame)
- Service-owned present loop (frame ownership runtime-owned only)
- Hidden upload spill into queued frames
- Hidden second queued frame reservoir
- Stale visibility reuse beyond legal window
- Upload spill surviving into later frames
- Service, presentation policy, or synthesis path acquiring second cadence governor
- Queue growth used as hidden stability valve
- Stale visibility accepted to hide upload or extraction debt
- Presentation policy overriding profile degradation order
- Presentation-owned residency or transfer governance
- Diagnostics readback in critical frame path
- Realtime outputs taking ownership of world truth, transport state, or residency governance
- Output-driven queue widening
- Hidden multi-frame buffering
- Imaging-owned publish loop
- Presentation-driven network flush
- Acoustics-owned cadence governor
- Role widening after bootstrap
- Hidden host-role widening inside runtime or network services
- Spending local near-footprint entitlement on remote-only burden
- Using queued frames as substitute for legal role composition
- Merging interactive-60 and listen-host-60 envelopes into undefined middle state
- listen-host-60 consuming local near entitlement by silently widening role classes after bootstrap
