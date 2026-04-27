# Threading

## Concurrency posture
- task requests are append-only
- a task_request_id refers to one task kind only
- priority changes publish a successor request or scheduler annotation rather than mutating the request row
- consumers may process different requests concurrently

## Threading law
`tool_task_requests` must remain deterministic under replay. Writers synchronize by the declared identity key for the sidecar; readers consume immutable published rows or explicit replacement rows only.

## Replay note
Any concurrency choice for `tool_task_requests` must be auditable from published ids, cursors, or epochs. Implicit locks or hidden queues are illegal.
