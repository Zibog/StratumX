# Climate Theater Front And Distant Storm Operator Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Give operators one exact surface for front injection, cyclone/anticyclone inspection, far-storm observability, and replay.

## Mandatory controls
- create or load front seed;
- inject front into region scope;
- inspect front lifecycle and arrival window;
- view distant observability badges;
- inspect lunar phase coupling;
- replay storm arrival.

## Required inspector fields
- `front_ref`
- `front_lifecycle_state`
- `region_scope`
- `arrival_window`
- `precipitation_class`
- `lunar_phase_code`
- `distant_observability_verdict`

## Disabled reasons
`CLM_DISABLED_NO_WORLD_SCOPE`, `CLM_DISABLED_FRONT_SCHEMA_INVALID`, `CLM_DISABLED_REPLAY_WINDOW_MISSING`
