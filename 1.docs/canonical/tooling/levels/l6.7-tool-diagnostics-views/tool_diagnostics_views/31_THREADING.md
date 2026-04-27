# tool_diagnostics_views threading

Concurrency posture:
- view refreshes may run concurrently for different filter digests
- a diagnostic_view_id is immutable after publication
- grouping_mode changes publish a new view id

Threading law:
publication order is deterministic for the identity key of `tool_diagnostics_views`.

## Replay note
Any concurrency choice for `tool_diagnostics_views` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
