# Threading

## Concurrency posture
- view generation may run concurrently for different targets
- a given inspection_view_id is immutable after publication
- replacement views must publish a new generation epoch
- inspection readers must never mutate snapshot truth

## Threading law
`tool_inspection_views` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_inspection_views` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
