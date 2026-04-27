# tool_selection dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.4-index-plane`

Dependency law:
`tool_selection` may depend only on the listed surfaces because it publishes published selection refs emitted by editor surfaces so tooling services can target the same objects without owning UI selection truth.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.
