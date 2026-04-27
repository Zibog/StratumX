# tool_diagnostics_views boundary preservation

Never owns:
- event truth
- acknowledgement state
- source-runtime ownership
- layout/widget rendering

Boundary law:
`tool_diagnostics_views` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_diagnostics_views` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
