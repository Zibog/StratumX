# Invalidation

This contract belongs specifically to the preview runtime level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `preview_runtime`
- dependency shift in `l6.3-snapshot-plane` that changes `preview_run_id` semantics
- dependency shift in `l6.4-index-plane` that changes `preview_request_id` semantics
- dependency shift in `l6.5-derived-plane` that changes `preview_kind` semantics
- dependency shift in `l6.7-stream-plane` that changes `source_snapshot_set` semantics
- supersede, deny, or cancel affecting `preview_run_id`
- budget pressure that invalidates disposable outputs of `preview_runtime` while preserving authoritative rows

## Invalidation law
Invalidation in `preview_runtime` must explicitly name stale records such as `preview_run_id`, `preview_request_id`, `preview_kind`, `source_snapshot_set` instead of rebuilding an unnamed mirror.
