# Link Egress Observations Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| observation_batch_id | ObservationBatchId | required | stable identity for one outbound factual batch | unique per publication cursor |
| source_runtime_handle | EngineRuntimeHandle | required | runtime surface that produced the facts | must resolve through `engine_runtime_handles` |
| source_state_ref | EngineStateRef | required | state snapshot referenced by the batch | must resolve through `engine_state_refs` |
| fact_class_set | ObservationFactClassSet | required | closed set of fact classes carried by the batch | must remain read-only after publication |
| publication_cursor | PublicationCursor | required | ordered egress cursor for observers | monotonic within one session |
| emitted_at_tick | EgressTick | required | outbound publication tick | must not move backward for the same session |

## Local invariant rule
Each field above exists because `link_egress_observations` must publish link egress observations without absorbing adjacent semantic truth.
