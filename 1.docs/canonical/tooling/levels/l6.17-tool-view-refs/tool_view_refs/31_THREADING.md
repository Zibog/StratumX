# tool_view_refs threading

Concurrency posture:
- view refs are single-writer per view_ref_id
- a host move publishes a new epoch and may publish a successor ref if identity changes
- multiple view refs may refresh concurrently

Threading law:
publication order is deterministic for the identity key of `tool_view_refs`.

## Replay note
Any concurrency choice for `tool_view_refs` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
