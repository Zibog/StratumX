# tool_view_refs boundary preservation

Never owns:
- rendered framebuffer or widget truth
- camera/control state owned by editor
- preview payload truth
- selection or focus ownership

Boundary law:
`tool_view_refs` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_view_refs` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
