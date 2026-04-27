# l10.2-graph-authoring-service Libraries

## Allowed library classes
- graph node/edge schemas
- graph serialization and migration helpers
- validation and compile-preview adapters
- subgraph/template reuse helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l10.2-graph-authoring-service` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l10.2-graph-authoring-service` and must stay specific enough to support implementation work without reinterpretation.
