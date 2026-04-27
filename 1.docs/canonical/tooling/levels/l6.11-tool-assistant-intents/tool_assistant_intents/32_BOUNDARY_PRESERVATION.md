# tool_assistant_intents boundary preservation

Never owns:
- final mutation commands
- hidden model context not published through assistant runtime
- UI chat transcript ownership
- source asset/world truth

Boundary law:
`tool_assistant_intents` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_assistant_intents` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
