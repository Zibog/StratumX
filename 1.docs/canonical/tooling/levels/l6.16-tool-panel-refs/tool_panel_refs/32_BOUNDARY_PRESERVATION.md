# tool_panel_refs boundary preservation

Never owns:
- actual layout topology
- widget hierarchy
- panel-local transient UI state
- render timing truth

Boundary law:
`tool_panel_refs` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_panel_refs` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
