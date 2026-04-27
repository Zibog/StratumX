# tool_preview_results dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.12-preview-runtime`
- `l6.4-tool-preview-requests`

Dependency law:
`tool_preview_results` may depend only on the listed surfaces because it publishes preview result refs and status summaries returned by preview_runtime.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
