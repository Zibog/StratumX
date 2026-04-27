# tool_diagnostics_events boundary preservation

Never owns:
- grouped issue views
- UI filter state
- mute/acknowledge policy state
- authority mutation truth

Boundary law:
`tool_diagnostics_events` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_diagnostics_events` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
