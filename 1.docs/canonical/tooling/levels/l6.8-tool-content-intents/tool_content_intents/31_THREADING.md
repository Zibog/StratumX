# tool_content_intents threading

Concurrency posture:
- intents are append-only and ordered per issuer_session_id
- batch content intents may fan out to multiple task requests
- intent priority is advisory and must not reorder committed authority truth

Threading law:
publication order is deterministic for the identity key of `tool_content_intents`.

## Replay note
Any concurrency choice for `tool_content_intents` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
