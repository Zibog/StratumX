# Threading

## Concurrency posture
- release intents are append-only and ordered per issuer session
- a publish intent must reference one artifact scope only
- automation may fan out multiple release intents but must preserve provenance
- consumers must legalize destructive release actions

## Threading law
`tool_release_intents` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_release_intents` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
