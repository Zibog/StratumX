# Threading

## Concurrency posture
- selection writes are serialized per session
- multiple sessions may publish independent selections concurrently
- selection_epoch must advance on semantic selection change only
- selection snapshots are read-mostly and immutable after publication

## Threading law
`tool_selection` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_selection` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
