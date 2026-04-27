# tool_activation_rules boundary preservation

Never owns:
- current activation truth
- widget focus/layout ownership
- tool execution results
- policy state outside declared rules

Boundary law:
`tool_activation_rules` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_activation_rules` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
