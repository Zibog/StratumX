# Threading

## Concurrency posture
- requests may be issued concurrently across sessions
- cancel_token is single-writer by issuer session
- superseding a request publishes a new request id rather than mutating prior truth
- preview_runtime consumes requests idempotently

## Threading law
`tool_preview_requests` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_preview_requests` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
