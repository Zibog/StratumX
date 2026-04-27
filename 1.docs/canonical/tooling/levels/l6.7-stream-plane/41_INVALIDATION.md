# Invalidation

This contract belongs specifically to the stream plane level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `stream_plane`
- dependency shift in `l6.0-authority-core` that changes `stream_event_id` semantics
- dependency shift in `l6.9-budget-runtime` that changes `stream_kind` semantics
- supersede, deny, or cancel affecting `stream_event_id`
- budget pressure that invalidates disposable outputs of `stream_plane` while preserving authoritative rows

## Invalidation law
Invalidation in `stream_plane` must explicitly name stale records such as `stream_event_id`, `stream_kind`, `source_scope_id`, `ordering_cursor` instead of rebuilding an unnamed mirror.
