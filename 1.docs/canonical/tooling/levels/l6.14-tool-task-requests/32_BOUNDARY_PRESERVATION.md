# Boundary Preservation

## Never owned here
- runtime execution state ownership
- final result truth
- session/layout truth
- hidden scheduler queues

## Boundary law
`tool_task_requests` is a public sidecar mirror, not a stealth owner. It may expose routing, scope, or result references, but it may not absorb upstream editor ownership or downstream runtime/build truth.

## Drift alarm
If `tool_task_requests` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
