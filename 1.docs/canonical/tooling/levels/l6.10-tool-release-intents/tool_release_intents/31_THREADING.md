# tool_release_intents threading

Concurrency posture:
- release intents are append-only and ordered per issuer session
- a publish intent must reference one artifact scope only
- automation may fan out multiple release intents but must preserve provenance

Threading law:
publication order is deterministic for the identity key of `tool_release_intents`.

## Replay note
Any concurrency choice for `tool_release_intents` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
