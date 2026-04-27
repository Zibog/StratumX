# L5 To Tools Consumption Map

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

| Upper layer | L5 modules consumed | Access type | Why it matters for the editor dream | Notes |
|---|---|---|---|---|
| `L6 authority_core / command_envelopes / transaction_ledger` | ingress lanes, legality tables, handles, refs | direct typed bridge consumption | lowers legal editor mutations and runtime controls without bypassing the bridge | `L6` owns mutation authority, not `L5` |
| `L6 snapshot_plane / stream_plane / artifact_plane` | observations, metrics, compatibility facts, artifact refs | immutable fanout | feeds outliner/content/inspector/runtime projections, diagnostics, and deterministic artifact status | no widening into UI payloads in `L5` |
| `L6 index_plane / derived_plane` | identity refs, state refs, fact snapshots, epoch signals | rebuildable upper-layer consumption | builds hierarchy, dependency, reference, validation, region/cell, and package projections | indexing stays above the bridge |
| `L6 validation_runtime / preview_runtime / build_runtime / release_runtime` | facts, verdicts, handles, artifact refs, metrics | bounded read consumption | supports validation, previews, asset processor queues, bakes, builds, and release packaging without ownership leakage | all policy remains above `L5` |
| `L6 workspace_runtime` | session handles, epoch markers, attachment-safe refs via `L6` | bounded coordination | coordinates session-safe lower-runtime bindings without owning product UI | editor owns layout/panels/selection/focus presentation |
| `L6A assistant runtime` | observations, metrics, bounded refs via `L6` evidence packs | indirect bounded consumption | assistant can reason over evidence without touching engine internals directly | no direct ownership leak into `L5` |
| `L7 studio orchestration` | profile facts, legality classes, artifact refs via `L6` and compiled task bundles | indirect cold meta consumption | orchestration can compile world/content/build campaigns without entering hot bridge paths | `L7` stays cold |
| `L7A assistant brain` | none directly by default | indirect only | planning may reason over `L6A` packs and `L7` context, not raw bridge internals | planning must not couple to `L5` |
