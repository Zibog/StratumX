# tool_session boundary preservation

Never owns:
- selection ownership
- panel or view ownership
- preview/build/release result truth
- authority mutation payloads

Boundary law:
`tool_session` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_session` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
