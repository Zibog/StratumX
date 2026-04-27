# Threading

## Concurrency posture
- one preview_result row per completed request outcome
- freshness_epoch must increase on regenerated output
- result rows are append-only summaries
- consumers may compare freshness without mutating origin truth

## Threading law
`tool_preview_results` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_preview_results` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
