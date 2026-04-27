# Dependencies

## Allowed imports
- `compat_profiles`
- `compat_versions`
- `compat_capabilities`

## Allowed dependents or consumers
- compatibility evaluation surface
- compatibility verdict publication surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`compat_verdicts` may depend only on the listed classes because records compatibility decisions derived from versions, capabilities, and profiles without mutating those source registries.
