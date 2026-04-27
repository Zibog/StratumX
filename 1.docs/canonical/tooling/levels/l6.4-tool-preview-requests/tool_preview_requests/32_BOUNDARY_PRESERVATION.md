# tool_preview_requests boundary preservation

Never owns:
- rendered preview truth
- preview cache ownership
- build jobs
- diagnostic issue sets

Boundary law:
`tool_preview_requests` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_preview_requests` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
