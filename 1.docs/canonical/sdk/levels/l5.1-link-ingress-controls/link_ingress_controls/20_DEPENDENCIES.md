# Link Ingress Controls Local Dependencies

## Allowed imports
- `compat_versions`
- `compat_capabilities`
- `transport_policies`
- `legality_gates`
- `engine_session_handles`
- `engine_runtime_handles`
- `engine_object_handles`

## Forbidden expansion
- no undeclared L5 classes
- no L6/L7/L8+ truth
- no hidden caches or state mirrors
