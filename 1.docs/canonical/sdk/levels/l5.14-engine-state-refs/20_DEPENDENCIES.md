# Dependencies

## Allowed imports
- `engine_runtime_handles`
- `engine_identity_refs`

## Allowed dependents or consumers
- state ref publish surface
- state ref lookup surface
- observation export handoff

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`engine_state_refs` may depend only on the listed classes because publishes immutable references to engine state snapshots that may be observed but not mutated by the bridge.
