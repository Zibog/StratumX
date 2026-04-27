# Threading

## Concurrency posture
- intents are append-only and ordered per issuer_session_id
- batch content intents may fan out to multiple task requests
- intent priority is advisory and must not reorder committed authority truth
- consumers must legalize before mutation

## Threading law
`tool_content_intents` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_content_intents` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
