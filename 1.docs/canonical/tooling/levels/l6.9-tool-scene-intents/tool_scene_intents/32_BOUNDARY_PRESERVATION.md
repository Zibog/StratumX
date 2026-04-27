# tool_scene_intents boundary preservation

Never owns:
- scene mutation truth
- cell streaming state ownership
- runtime simulation state
- UI hover/selection state

Boundary law:
`tool_scene_intents` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_scene_intents` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
