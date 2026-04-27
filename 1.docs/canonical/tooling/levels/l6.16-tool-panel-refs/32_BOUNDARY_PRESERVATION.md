# Boundary Preservation

## Never owned here
- actual layout topology
- widget hierarchy
- panel-local transient UI state
- render timing truth

## Boundary law
`tool_panel_refs` is a public sidecar mirror, not a stealth owner. It may expose routing, scope, or result references, but it may not absorb upstream editor ownership or downstream runtime/build truth.

## Drift alarm
If `tool_panel_refs` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
