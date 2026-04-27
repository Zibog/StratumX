# tool_scene_intents threading

Concurrency posture:
- scene intents are ordered per issuer_session_id and target scope
- world edits affecting multiple cells must publish explicit scope sets
- consumers must legalize before mutation

Threading law:
publication order is deterministic for the identity key of `tool_scene_intents`.

## Replay note
Any concurrency choice for `tool_scene_intents` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
