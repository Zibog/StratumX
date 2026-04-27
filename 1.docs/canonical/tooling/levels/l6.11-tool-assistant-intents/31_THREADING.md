# Threading

## Concurrency posture
- assistant intents are immutable after publication
- confidence updates publish a new assistant intent revision or successor row
- lowering must preserve issuer_session_id and provenance
- assistant traffic may not bypass legalization

## Threading law
`tool_assistant_intents` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_assistant_intents` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
