# tool_task_results boundary preservation

Never owns:
- scheduler ownership
- task request truth
- artifact payload storage
- UI notification state

Boundary law:
`tool_task_results` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_task_results` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
