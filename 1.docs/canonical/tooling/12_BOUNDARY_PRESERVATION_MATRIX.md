# Boundary Preservation Matrix

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

| Boundary | Must preserve | Must not leak |
|---|---|---|
| `L5 -> L6` | bridge facts, handles, refs, artifact refs, verdict tables, snapshots, batches, ingress publication, and epoch markers only | engine internals, editor-shaped mutable structs |
| `L6 -> L6A` | bounded runtime data, evidence inputs, proposal surfaces, apply/revert results | authority ownership, hidden mutable truth |
| `L6A -> L7A` | bounded goals, bounded context, evidence requests, plan requests, routing requests | raw editor authority state |
| `L7 -> L6` | compiled campaign bundles, task bundles, governance policies, automation requests, reporting requests | frame-level runtime authority |
| `L7A -> L6A` | plan bundles, proposal intents, canon constraints, optimization alternatives, migration plans | direct apply ownership |
| `L8+ editor product surfaces -> tooling public surfaces` | public runtime projections, public refs, public queues, public diagnostics, public manifests, public assistant/reporting surfaces | hidden authority state, hidden queue state, engine internals, bypass transactions |

## Editor/UI ownership law
The editor product owns:
- shell layout and dock composition
- viewport navigation state
- panel/view state
- selection presentation state
- focus presentation state
- inspector staged-edit state
- command palette/search state
- plugin-contributed UI state

Tooling owns only lower-runtime coordination and public request/result state.
Any attempt by tooling to own editor-local panel truth, inspector widget truth, or dock composition is a boundary violation.

## Read discipline
- `editor/` reads authority-backed truth only through `snapshot/index/derived/artifact/stream`
- validation verdicts come from `validation_runtime`
- preview results come from `preview_runtime`
- asset processor, bake, cook, and build queue state come from `build_runtime`
- packaging, release closure, and bundle state come from `release_runtime`

## Write discipline
- all committed authoring mutations go through `command_envelopes -> transaction_ledger`
- all preview requests go through `preview_runtime`
- all validation requests go through `validation_runtime`
- all import/reimport/bake/build requests go through `build_runtime`
- all release/package requests go through `release_runtime`
