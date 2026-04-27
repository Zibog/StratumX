# tool_preview_results threading

Concurrency posture:
- one preview_result row per completed request outcome
- freshness_epoch must increase on regenerated output
- result rows are append-only summaries

Threading law:
publication order is deterministic for the identity key of `tool_preview_results`.

## Replay note
Any concurrency choice for `tool_preview_results` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
