# tool_preview_requests dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.1-tool-selection`
- `l6.12-preview-runtime`

Dependency law:
`tool_preview_requests` may depend only on the listed surfaces because it publishes preview requests flowing from editor/tools into preview_runtime.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
