# Libraries

## Local record classes
- `ObservationBatchId` for `observation_batch_id`
- `EngineRuntimeHandle` for `source_runtime_handle`
- `EngineStateRef` for `source_state_ref`
- `ObservationFactClassSet` for `fact_class_set`
- `PublicationCursor` for `publication_cursor`
- `EgressTick` for `emitted_at_tick`

## Shared registries consumed
- `engine_runtime_handles` registry or lookup surface
- `engine_state_refs` registry or lookup surface
- `transport_policies` registry or lookup surface
- `compat_profiles` registry or lookup surface

## Library law
`link_egress_observations` may introduce only record classes that help publish or resolve link egress observations. It may not invent a parallel semantic vocabulary for adjacent bridge classes.
