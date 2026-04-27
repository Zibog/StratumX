# Boundary Preservation

## Canonical Rule

`engine_runtime_headless` provides headless simulation runtime profile. Headless execution must remain clean, deterministic, and free from realtime orchestration noise.

## Upward Boundary

**Exports to layers above:**
- Profile config (headless profile definition)
- Headless events (headless execution progression)
- Simulation outputs (authoritative world deltas, network publications, replay artifacts)
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
- Presentable frame ownership in headless mode
- Borrowing realtime queue semantics
- Host burden exceeding headless proof
- Hidden role widening after bootstrap
- Implicit host widening at startup role bind
- Borrowing presentation reserve for publication entitlement
- Validation role becoming general runtime mode
- Headless validation roles owning cadence, network transport, or storage governance
- Variable authoring cadence (must be fixed 20 Hz)
- Hidden wider tick entitlement (50.00 ms hard budget)
- Replay widening tick cadence
- Hidden IO back-pressure on tick
- Unbounded peer expansion
- Cadence stretching after queue breach
- Masking overload by skipping authoritative publication
- Hidden headless queue reservoir
- Service-owned cadence or tick pacing
- Output-side widening of tick budget
- Hidden debug output growth outside diagnostics law
- Publication queues outliving profile ceilings
- Shadow presentation path, shadow diagnostics spool, or long-lived publication reservoir
