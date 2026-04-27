# Dependencies

## Allowed imports
- `compat_capabilities`
- `compat_profiles`
- `compat_verdicts`

## Allowed dependents or consumers
- legality gate registry publish surface
- control legality evaluation surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`legality_gates` may depend only on the listed classes because declares explicit allow/warn/deny gate classes that every mutating ingress control must pass before acceptance.
