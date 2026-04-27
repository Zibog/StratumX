# Threading

## Concurrency posture
- one active focus target per focus owner surface
- focus changes are ordered per surface
- focus epoch advances when target, mode, or owner changes
- readers may not infer focus from raw selection without this sidecar

## Threading law
`tool_focus_refs` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_focus_refs` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
