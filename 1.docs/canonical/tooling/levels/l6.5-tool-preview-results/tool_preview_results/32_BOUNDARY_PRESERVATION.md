# tool_preview_results boundary preservation

Never owns:
- preview raster or mesh cache ownership
- request scheduling truth
- diagnostic events
- layout refresh state

Boundary law:
`tool_preview_results` may expose refs and routing metadata only; it may not absorb foreign truth.

## Drift alarm
If `tool_preview_results` starts storing widget state, runtime payloads, or authority truth, the sidecar has crossed its legal boundary and must be split or corrected.
