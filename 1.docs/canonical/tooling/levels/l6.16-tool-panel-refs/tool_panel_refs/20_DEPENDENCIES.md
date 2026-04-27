# tool_panel_refs dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.10-workspace-runtime`

Dependency law:
`tool_panel_refs` may depend only on the listed surfaces because it publishes panel refs visible to tooling services while keeping actual layout and widget ownership in editor.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
