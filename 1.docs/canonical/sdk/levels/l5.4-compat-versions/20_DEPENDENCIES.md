# Dependencies

## Allowed imports
- `compat_profiles`

## Allowed dependents or consumers
- version registry publish surface
- version lookup surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`compat_versions` may depend only on the listed classes because declares protocol versions and structural compatibility bands that every bridge envelope must name.
