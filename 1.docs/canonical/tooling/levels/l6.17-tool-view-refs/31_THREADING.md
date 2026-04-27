# Threading

## Concurrency posture
- view refs are single-writer per view_ref_id
- a host move publishes a new epoch and may publish a successor ref if identity changes
- multiple view refs may refresh concurrently
- consumers may observe but never mutate rendered view state

## Threading law
`tool_view_refs` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_view_refs` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
