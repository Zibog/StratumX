# Dependencies

## Allowed imports
- `engine_runtime_handles`
- `engine_state_refs`

## Allowed dependents or consumers
- artifact ref publish surface
- artifact ref lookup surface
- artifact export handoff

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`engine_artifact_refs` may depend only on the listed classes because publishes immutable references to engine-produced artifacts such as captures, compiled bundles, or replay manifests.
