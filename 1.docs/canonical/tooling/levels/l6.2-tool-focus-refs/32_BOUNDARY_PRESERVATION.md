# Boundary Preservation

## Never owned here
- actual OS/window focus
- text-input caret ownership
- multi-target selection truth
- layout activation

## Boundary law
`tool_focus_refs` is a public sidecar mirror, not a stealth owner. It may expose routing, scope, or result references, but it may not absorb upstream editor ownership or downstream runtime/build truth.

## Drift alarm
If `tool_focus_refs` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
