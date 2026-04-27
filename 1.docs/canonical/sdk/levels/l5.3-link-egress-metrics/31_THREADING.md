# Threading

## Concurrency posture for link egress metrics
- publication and mutation of metric_batch_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as metric_batch_id, metric_kind_set, aggregation_window without creating a second writer
- no background task may rewrite already-published link egress metrics rows

## Forbidden concurrency patterns
- hidden mutable mirrors of link egress metrics
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
