# Threading

## Concurrency posture
- state is single-writer per tool_kind
- state_epoch advances on every semantic activation change
- consumers must not infer activation from rules alone once state is published
- disabled or denied states remain explicit rows

## Threading law
`tool_activation_state` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_activation_state` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
