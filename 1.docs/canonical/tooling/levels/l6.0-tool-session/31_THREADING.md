# Threading

## Concurrency posture
- single-writer per tool_session_id
- cross-session publication may run concurrently
- state transitions must be monotonic open→draining→closed
- readers may cache snapshots only by explicit epoch/cursor

## Threading law
`tool_session` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_session` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
