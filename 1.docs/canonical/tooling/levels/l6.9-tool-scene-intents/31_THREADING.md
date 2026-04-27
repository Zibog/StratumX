# Threading

## Concurrency posture
- scene intents are ordered per issuer_session_id and target scope
- world edits affecting multiple cells must publish explicit scope sets
- consumers must legalize before mutation
- intent replacement publishes a new intent id rather than patching old intent rows

## Threading law
`tool_scene_intents` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_scene_intents` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
