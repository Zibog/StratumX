# Dependencies

## Allowed imports
- `compat_versions`
- `compat_profiles`

## Allowed dependents or consumers
- transport policy registry publish surface
- transport policy lookup surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`transport_policies` may depend only on the listed classes because declares bounded transport, batching, retry, and framing policies visible to public bridge traffic.
