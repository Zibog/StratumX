# Threading

## Concurrency posture
- ordering is monotonic within a source scope
- cross-scope diagnostics may interleave concurrently
- events are append-only and never rewritten in place
- severity escalation publishes a new event row or explicit supersession

## Threading law
`tool_diagnostics_events` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_diagnostics_events` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
