# tool_selection boundary preservation

Never owns:
- widget highlight state
- hover-only ephemeral cursor state
- object truth beyond published refs
- mutation commands

Boundary law:
`tool_selection` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_selection` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
