# tool_focus_refs threading

Concurrency posture:
- one active focus target per focus owner surface
- focus changes are ordered per surface
- focus epoch advances when target, mode, or owner changes

Threading law:
publication order is deterministic for the identity key of `tool_focus_refs`.

## Replay note
Any concurrency choice for `tool_focus_refs` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
