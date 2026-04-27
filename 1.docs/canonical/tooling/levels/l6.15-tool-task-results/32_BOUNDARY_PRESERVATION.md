# Boundary Preservation

## Never owned here
- scheduler ownership
- task request truth
- artifact payload storage
- UI notification state

## Boundary law
`tool_task_results` is a public sidecar mirror, not a stealth owner. It may expose routing, scope, or result references, but it may not absorb upstream editor ownership or downstream runtime/build truth.

## Drift alarm
If `tool_task_results` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
