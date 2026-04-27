# Libraries

## Local record classes
- `MetricBatchId` for `metric_batch_id`
- `MetricKindSet` for `metric_kind_set`
- `MetricAggregationWindow` for `aggregation_window`
- `EngineRuntimeHandle` for `source_runtime_handle`
- `PublicationCursor` for `publication_cursor`
- `MetricQualityFlag` for `quality_flag`

## Shared registries consumed
- `engine_runtime_handles` registry or lookup surface
- `compat_profiles` registry or lookup surface
- `transport_policies` registry or lookup surface

## Library law
`link_egress_metrics` may introduce only record classes that help publish or resolve link egress metrics. It may not invent a parallel semantic vocabulary for adjacent bridge classes.
