# Dependencies

## Allowed imports
- `engine_session_handles`
- `engine_identity_refs`

## Allowed dependents or consumers
- object handle publish surface
- object handle lookup surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`engine_object_handles` may depend only on the listed classes because provides stable opaque handles for addressed engine objects visible through public bridge controls or observations.
