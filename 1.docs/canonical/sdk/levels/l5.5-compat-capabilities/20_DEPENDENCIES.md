# Dependencies

## Allowed imports
- `compat_profiles`
- `compat_versions`

## Allowed dependents or consumers
- capability registry publish surface
- capability lookup surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`compat_capabilities` may depend only on the listed classes because declares named capability bits and feature switches visible across the bridge boundary.
