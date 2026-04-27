# Threading

## Concurrency posture
- rule writes are serialized by tool_kind
- priority ordering must be deterministic for the same rule set
- deny condition evaluation is pure and side-effect free
- rule replacement publishes a new rule id or explicit revision

## Threading law
`tool_activation_rules` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_activation_rules` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
