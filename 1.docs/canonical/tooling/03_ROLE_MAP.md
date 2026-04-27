# Role Map

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## L6 owns
- minimal authoring authority records
- typed command envelopes
- ordered transactions and undo/revert bindings
- immutable snapshots
- rebuildable indices
- disposable derived projections
- deterministic artifacts and manifests
- bounded streams
- evictable caches
- budget enforcement
- workspace coordination runtime
- validation runtime
- preview runtime
- build runtime
- release runtime

## L6 explicit non-ownership
`L6` does **not** own the editor product UI.
It must not own:
- shell layout or dock composition
- viewport camera state
- panel tab state
- outliner row expansion state
- content-browser filter chips
- inspector staged-edit widgets
- package manager UI widgets
- runtime-inspector panel chrome

Those remain in `editor/`.
`L6` may expose session-safe refs, queue state, cursors, and public runtime bindings only.

## L6A owns
- assistant session lifecycle
- bounded evidence packs
- proposals and proposal staging
- lowering from proposals/plans to `L6` commands
- safety and approval gates
- apply/revert mediation
- assistant-facing UI runtime
- bounded model request runtime

## L7 owns
- compiled campaign graphs
- project, content, world, simulation, and release orchestration
- automation meta
- governance meta
- reporting surfaces

## L7A owns
- prompt normalization
- planning IR construction
- canon reasoning outputs
- generation planning
- optimization alternatives and trade-offs
- migration plans
- model routing policy
