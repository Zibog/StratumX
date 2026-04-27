# tool_focus_refs dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.1-tool-selection`

Dependency law:
`tool_focus_refs` may depend only on the listed surfaces because it publishes published focus refs for one active inspection or interaction target without owning widget focus behavior.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
