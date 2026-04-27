# Dependencies

## Allowed imports
- `compat_versions`
- `compat_capabilities`

## Allowed dependents or consumers
- profile registry publish surface
- profile lookup surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`compat_profiles` may depend only on the listed classes because bundles versions, capabilities, and support posture into named bridge compatibility profiles.
