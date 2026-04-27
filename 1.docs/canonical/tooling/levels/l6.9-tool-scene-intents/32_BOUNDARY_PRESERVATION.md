# Boundary Preservation

## Never owned here
- scene mutation truth
- cell streaming state ownership
- runtime simulation state
- UI hover/selection state

## Boundary law
`tool_scene_intents` is a public sidecar mirror, not a stealth owner. It may expose routing, scope, or result references, but it may not absorb upstream editor ownership or downstream runtime/build truth.

## Drift alarm
If `tool_scene_intents` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
