# tool_inspection_views boundary preservation

Never owns:
- live widget trees
- authoritative object mutation truth
- preview rendering results
- diagnostics event truth

Boundary law:
`tool_inspection_views` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_inspection_views` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
