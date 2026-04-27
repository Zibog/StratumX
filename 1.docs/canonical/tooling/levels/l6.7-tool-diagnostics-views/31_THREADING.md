# Threading

## Concurrency posture
- view refreshes may run concurrently for different filter digests
- a diagnostic_view_id is immutable after publication
- grouping_mode changes publish a new view id
- readers must not infer missing events from stale view caches

## Threading law
`tool_diagnostics_views` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_diagnostics_views` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
