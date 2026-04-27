# tool_view_refs dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.10-workspace-runtime`
- `l6.16-tool-panel-refs`

Dependency law:
`tool_view_refs` may depend only on the listed surfaces because it publishes view refs for view-host coordination without taking ownership of rendered editor views.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
