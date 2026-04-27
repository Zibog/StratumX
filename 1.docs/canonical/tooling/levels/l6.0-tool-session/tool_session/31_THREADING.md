# tool_session threading

Concurrency posture:
- single-writer per tool_session_id
- cross-session publication may run concurrently
- state transitions must be monotonic open→draining→closed

Threading law:
publication order is deterministic for the identity key of `tool_session`.

## Replay note
Any concurrency choice for `tool_session` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
