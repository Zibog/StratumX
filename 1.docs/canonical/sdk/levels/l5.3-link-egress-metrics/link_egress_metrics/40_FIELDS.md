# Link Egress Metrics Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| metric_batch_id | MetricBatchId | required | stable identity of one outbound metric batch | unique per source runtime and cursor |
| metric_kind_set | MetricKindSet | required | closed set of metric kinds included | must use registered metric enums only |
| aggregation_window | MetricAggregationWindow | required | window used to aggregate counters or gauges | must be explicit and finite |
| source_runtime_handle | EngineRuntimeHandle | required | runtime surface that produced the metrics | must resolve through `engine_runtime_handles` |
| publication_cursor | PublicationCursor | required | ordered metric egress cursor | monotonic per session |
| quality_flag | MetricQualityFlag | required | quality or degradation state of the batch | must reflect explicit enum, not prose |

## Local invariant rule
Each field above exists because `link_egress_metrics` must publish link egress metrics without absorbing adjacent semantic truth.
