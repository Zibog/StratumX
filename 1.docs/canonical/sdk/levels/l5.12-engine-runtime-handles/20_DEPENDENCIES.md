# Dependencies

## Allowed imports
- `engine_session_handles`

## Allowed dependents or consumers
- runtime handle publish surface
- runtime handle lookup surface

## Forbidden imports
- `tooling/` or `editor/` package truth
- undeclared engine-internal state or caches
- adjacent L5 semantic classes not named above

## Local dependency law
`engine_runtime_handles` may depend only on the listed classes because provides opaque handles for public runtime surfaces such as simulation channels, subscriptions, and attachment scopes.
