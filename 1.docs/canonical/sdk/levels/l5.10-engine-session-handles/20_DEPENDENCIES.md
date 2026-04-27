# Dependencies

## Allowed imports
- `compat_profiles`

## Allowed dependents or consumers
- session handle publish surface
- session handle lookup surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`engine_session_handles` may depend only on the listed classes because provides stable opaque handles for public bridge sessions without leaking engine internals.
