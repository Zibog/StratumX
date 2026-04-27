# Link Egress Observations Local Libraries

## Local vocabulary
- `ObservationBatchId` for `observation_batch_id`
- `EngineRuntimeHandle` for `source_runtime_handle`
- `EngineStateRef` for `source_state_ref`
- `ObservationFactClassSet` for `fact_class_set`
- `PublicationCursor` for `publication_cursor`
- `EgressTick` for `emitted_at_tick`

## Audit rule
Every local type above must exist to support this semantic class, not a neighboring one.
