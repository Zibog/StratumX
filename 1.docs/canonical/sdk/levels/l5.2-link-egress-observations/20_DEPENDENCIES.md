# Dependencies

## Allowed imports
- `engine_runtime_handles`
- `engine_state_refs`
- `transport_policies`
- `compat_profiles`

## Allowed dependents or consumers
- engine factual snapshot handoff
- L4 observation subscription surface
- bounded observation stream surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`link_egress_observations` may depend only on the listed classes because publishes factual observation batches from engine-facing snapshots back toward public L4 read surfaces.
