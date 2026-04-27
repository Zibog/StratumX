# Engine L4 Binding Map

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

| Engine export surface | L5 module | Surface form | Upper-stack use | Notes |
|---|---|---|---|---|
| packet sink | `link_ingress_packets` | ordered ingress sink | command lowering from `L6` | payload body ownership stays outside sink headers |
| control sink | `link_ingress_controls` | ordered ingress sink | session/runtime control lowering | execution signal semantics remain typed |
| observation batch source | `link_egress_observations` | immutable egress batch | snapshot/stream fanout into inspection and diagnostics | fanout-safe publication only |
| metric batch source | `link_egress_metrics` | immutable egress batch | diagnostics, budget, profiling, playback traces | metrics stay read-only |
| version fact table | `compat_versions` | immutable bridge snapshot | validation, compatibility, migration hints | facts only, no policy ownership |
| capability fact table | `compat_capabilities` | immutable bridge snapshot | feature gating and editor affordance gating | facts only |
| profile fact table | `compat_profiles` | immutable bridge snapshot | budgeting and degradation inputs | cost/profile tuples only |
| session handle table | `engine_session_handles` | immutable bridge snapshot | session binding and runtime bridge attach | no editor semantics |
| object handle table | `engine_object_handles` | immutable bridge snapshot | focused object targeting and preview binding | opaque object targeting only |
| runtime handle table | `engine_runtime_handles` | immutable bridge snapshot | PIE/simulate/runtime bridge control | opaque runtime handles only |
| identity projection table | `engine_identity_refs` | immutable bridge snapshot | stable authoring lookup seeds | read-side only |
| state projection table | `engine_state_refs` | immutable bridge snapshot | preview/inspection/diagnostics projections | read-side only |
| artifact projection table | `engine_artifact_refs` | immutable bridge snapshot | preview/build/release lookup seeds | generated-product only |
| bridge epoch signal | package-wide bind | monotonic invalidation signal | snapshot swap and cursor continuity | required for partial refresh correctness |
