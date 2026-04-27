# Engine State Refs Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| state_ref | EngineStateRef | required | stable ref to one immutable state snapshot | must resolve to immutable state only |
| runtime_handle | EngineRuntimeHandle | required | runtime surface that emitted the snapshot | must resolve through `engine_runtime_handles` |
| snapshot_epoch | SnapshotEpoch | required | epoch of the referenced state snapshot | must be monotonic for one runtime source |
| fact_class_set | StateFactClassSet | required | fact classes exposed through the state ref | must remain read-only |
| retention_class | StateRetentionClass | required | bounded retention posture for the ref | must use declared enum and never imply hidden storage |

## Local invariant rule
Each field above exists because `engine_state_refs` must publish engine state refs without absorbing adjacent semantic truth.
