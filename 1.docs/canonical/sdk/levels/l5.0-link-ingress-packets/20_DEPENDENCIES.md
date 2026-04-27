# Dependencies

## Allowed imports
- `compat_versions`
- `transport_policies`
- `legality_gates`
- `engine_session_handles`

## Allowed dependents or consumers
- L4 ingress packet publish surface
- packet rejection surface
- decode-to-control split handoff
- decode-to-observation split handoff

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`link_ingress_packets` may depend only on the listed classes because accepts raw ingress packet envelopes from public L4 entry points before they are split into controls, facts, or version handshakes.
