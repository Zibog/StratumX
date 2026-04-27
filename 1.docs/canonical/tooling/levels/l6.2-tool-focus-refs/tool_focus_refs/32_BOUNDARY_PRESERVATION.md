# tool_focus_refs boundary preservation

Never owns:
- actual OS/window focus
- text-input caret ownership
- multi-target selection truth
- layout activation

Boundary law:
`tool_focus_refs` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_focus_refs` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
