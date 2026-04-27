# tool_panel_refs threading

Concurrency posture:
- panel refs are single-writer per panel_ref_id
- epoch advances on host/scope changes, not cosmetic widget repaint
- multiple panels may publish concurrently

Threading law:
publication order is deterministic for the identity key of `tool_panel_refs`.

## Replay note
Any concurrency choice for `tool_panel_refs` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
