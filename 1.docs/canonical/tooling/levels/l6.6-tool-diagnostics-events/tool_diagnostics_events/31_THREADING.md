# tool_diagnostics_events threading

Concurrency posture:
- ordering is monotonic within a source scope
- cross-scope diagnostics may interleave concurrently
- events are append-only and never rewritten in place

Threading law:
publication order is deterministic for the identity key of `tool_diagnostics_events`.

## Replay note
Any concurrency choice for `tool_diagnostics_events` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
