# Dependencies

## Allowed imports
- `engine_runtime_handles`
- `compat_profiles`
- `transport_policies`

## Allowed dependents or consumers
- engine metric export handoff
- L4 metric subscription surface
- bounded metric stream surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`link_egress_metrics` may depend only on the listed classes because publishes bounded metric series and counters for public monitoring and telemetry consumers.
