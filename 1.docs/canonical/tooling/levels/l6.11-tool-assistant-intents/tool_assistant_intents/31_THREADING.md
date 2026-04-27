# tool_assistant_intents threading

Concurrency posture:
- assistant intents are immutable after publication
- confidence updates publish a new assistant intent revision or successor row
- lowering must preserve issuer_session_id and provenance

Threading law:
publication order is deterministic for the identity key of `tool_assistant_intents`.

## Replay note
Any concurrency choice for `tool_assistant_intents` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
