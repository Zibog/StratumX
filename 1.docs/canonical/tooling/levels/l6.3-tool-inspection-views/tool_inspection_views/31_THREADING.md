# tool_inspection_views threading

Concurrency posture:
- view generation may run concurrently for different targets
- a given inspection_view_id is immutable after publication
- replacement views must publish a new generation epoch

Threading law:
publication order is deterministic for the identity key of `tool_inspection_views`.

## Replay note
Any concurrency choice for `tool_inspection_views` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
