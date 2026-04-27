# tool_preview_requests threading

Concurrency posture:
- requests may be issued concurrently across sessions
- cancel_token is single-writer by issuer session
- superseding a request publishes a new request id rather than mutating prior truth

Threading law:
publication order is deterministic for the identity key of `tool_preview_requests`.

## Replay note
Any concurrency choice for `tool_preview_requests` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
