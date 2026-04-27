# Libraries

## Local record classes
- `EngineStateRef` for `state_ref`
- `EngineRuntimeHandle` for `runtime_handle`
- `SnapshotEpoch` for `snapshot_epoch`
- `StateFactClassSet` for `fact_class_set`
- `StateRetentionClass` for `retention_class`

## Shared registries consumed
- `engine_runtime_handles` registry or lookup surface
- `engine_identity_refs` registry or lookup surface

## Library law
`engine_state_refs` may introduce only record classes that help publish or resolve engine state refs. It may not invent a parallel semantic vocabulary for adjacent bridge classes.
