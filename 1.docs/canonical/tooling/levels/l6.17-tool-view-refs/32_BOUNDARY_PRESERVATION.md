# Boundary Preservation

## Never owned here
- rendered framebuffer or widget truth
- camera/control state owned by editor
- preview payload truth
- selection or focus ownership

## Boundary law
`tool_view_refs` is a public sidecar mirror, not a stealth owner. It may expose routing, scope, or result references, but it may not absorb upstream editor ownership or downstream runtime/build truth.

## Drift alarm
If `tool_view_refs` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
