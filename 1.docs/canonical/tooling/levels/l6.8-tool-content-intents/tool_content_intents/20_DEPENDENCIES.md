# tool_content_intents dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.1-tool-selection`
- `l6.13-tool-activation-state`

Dependency law:
`tool_content_intents` may depend only on the listed surfaces because it publishes content-authoring intents that later become commands, imports, or build jobs.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
