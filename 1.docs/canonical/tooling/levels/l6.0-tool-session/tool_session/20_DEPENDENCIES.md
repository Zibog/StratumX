# tool_session dependencies

Legal dependencies:
- `l6.0-authority-core`

Dependency law:
`tool_session` may depend only on the listed surfaces because it publishes public tool-session identity, lifecycle, and caller scope shared by all downstream sidecar traffic.

## Dependency review note
If an implementation needs a dependency not listed above, the sidecar contract is incomplete and must be amended explicitly before work proceeds.

## Scope note
No self-dependency is legal for `tool_session`; session facts originate here and fan out to all downstream sidecars.
