# tool_diagnostics_views dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.6-tool-diagnostics-events`
- `l6.5-derived-plane`

Dependency law:
`tool_diagnostics_views` may depend only on the listed surfaces because it publishes filtered and grouped diagnostics views for consumers that need stable issue presentations.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
