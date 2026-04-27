# tool_task_results threading

Concurrency posture:
- results are append-only and immutable after publication
- a task_request_id may have multiple result rows only when the task model explicitly declares phase splits
- finished_at_cursor must be monotonic for superseding completions

Threading law:
publication order is deterministic for the identity key of `tool_task_results`.

## Replay note
Any concurrency choice for `tool_task_results` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
