# tool_selection threading

Concurrency posture:
- selection writes are serialized per session
- multiple sessions may publish independent selections concurrently
- selection_epoch must advance on semantic selection change only

Threading law:
publication order is deterministic for the identity key of `tool_selection`.

## Replay note
Any concurrency choice for `tool_selection` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
