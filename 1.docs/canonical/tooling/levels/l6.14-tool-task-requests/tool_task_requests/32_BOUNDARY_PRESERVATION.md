# tool_task_requests boundary preservation

Never owns:
- runtime execution state ownership
- final result truth
- session/layout truth
- hidden scheduler queues

Boundary law:
`tool_task_requests` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_task_requests` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
