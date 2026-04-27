# Threading

## Concurrency posture
- panel refs are single-writer per panel_ref_id
- epoch advances on host/scope changes, not cosmetic widget repaint
- multiple panels may publish concurrently
- tooling services may observe but never own the panel tree

## Threading law
`tool_panel_refs` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_panel_refs` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
