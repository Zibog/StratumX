# Dependencies

## Allowed imports
- `compat_versions`
- `compat_capabilities`
- `transport_policies`
- `legality_gates`
- `engine_session_handles`
- `engine_runtime_handles`
- `engine_object_handles`

## Allowed dependents or consumers
- L4 control submit surface
- ingress control publication surface
- control rejection stream
- control acknowledgement mirror

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`link_ingress_controls` may depend only on the listed classes because normalizes mutating control envelopes that travel from public L4 control surfaces into bridge-legal engine-facing command classes.
