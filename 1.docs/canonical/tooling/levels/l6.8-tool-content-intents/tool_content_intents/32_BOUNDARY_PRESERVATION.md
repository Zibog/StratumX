# tool_content_intents boundary preservation

Never owns:
- actual asset mutation truth
- import/build result truth
- selection ownership
- package manager state

Boundary law:
`tool_content_intents` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_content_intents` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
