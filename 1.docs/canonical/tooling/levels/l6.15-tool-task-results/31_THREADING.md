# Threading

## Concurrency posture
- results are append-only and immutable after publication
- a task_request_id may have multiple result rows only when the task model explicitly declares phase splits
- finished_at_cursor must be monotonic for superseding completions
- consumers must dereference result_ref for authoritative payloads

## Threading law
`tool_task_results` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_task_results` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
