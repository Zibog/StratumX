# tool_task_requests threading

Concurrency posture:
- task requests are append-only
- a task_request_id refers to one task kind only
- priority changes publish a successor request or scheduler annotation rather than mutating the request row

Threading law:
publication order is deterministic for the identity key of `tool_task_requests`.

## Replay note
Any concurrency choice for `tool_task_requests` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
