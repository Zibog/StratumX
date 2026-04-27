# tool_release_intents boundary preservation

Never owns:
- actual release artifacts
- signing secrets
- distribution credentials
- package manager installation truth

Boundary law:
`tool_release_intents` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_release_intents` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
