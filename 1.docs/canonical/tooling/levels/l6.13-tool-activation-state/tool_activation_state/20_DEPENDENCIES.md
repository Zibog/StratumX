# tool_activation_state dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.12-tool-activation-rules`
- `l6.10-workspace-runtime`

Dependency law:
`tool_activation_state` may depend only on the listed surfaces because it publishes active activation state for tools and modes after activation rules are evaluated.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
