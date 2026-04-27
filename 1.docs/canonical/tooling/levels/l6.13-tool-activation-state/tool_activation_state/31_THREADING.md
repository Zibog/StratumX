# tool_activation_state threading

Concurrency posture:
- state is single-writer per tool_kind
- state_epoch advances on every semantic activation change
- consumers must not infer activation from rules alone once state is published

Threading law:
publication order is deterministic for the identity key of `tool_activation_state`.

## Replay note
Any concurrency choice for `tool_activation_state` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
