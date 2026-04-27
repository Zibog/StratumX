# tool_activation_rules threading

Concurrency posture:
- rule writes are serialized by tool_kind
- priority ordering must be deterministic for the same rule set
- deny condition evaluation is pure and side-effect free

Threading law:
publication order is deterministic for the identity key of `tool_activation_rules`.

## Replay note
Any concurrency choice for `tool_activation_rules` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
