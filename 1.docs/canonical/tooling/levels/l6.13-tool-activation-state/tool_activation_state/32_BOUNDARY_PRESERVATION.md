# tool_activation_state boundary preservation

Never owns:
- rule ownership
- tool execution payloads
- layout ownership
- selection/focus truth

Boundary law:
`tool_activation_state` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_activation_state` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
