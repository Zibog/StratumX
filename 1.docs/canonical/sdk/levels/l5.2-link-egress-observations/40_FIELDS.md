# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| observation_batch_id | ObservationBatchId | required | stable identity for one outbound factual batch | unique per publication cursor |
| source_runtime_handle | EngineRuntimeHandle | required | runtime surface that produced the facts | must resolve through `engine_runtime_handles` |
| source_state_ref | EngineStateRef | required | state snapshot referenced by the batch | must resolve through `engine_state_refs` |
| fact_class_set | ObservationFactClassSet | required | closed set of fact classes carried by the batch | must remain read-only after publication |
| publication_cursor | PublicationCursor | required | ordered egress cursor for observers | monotonic within one session |
| emitted_at_tick | EgressTick | required | outbound publication tick | must not move backward for the same session |

## No hidden store law
All semantic truth in `link_egress_observations` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.
