# Dependencies

## Allowed imports
- `engine_session_handles`

## Allowed dependents or consumers
- identity ref publish surface
- identity ref lookup surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`engine_identity_refs` may depend only on the listed classes because publishes stable public identity refs that map bridge-facing identity to engine-owned entities without leaking internals.
