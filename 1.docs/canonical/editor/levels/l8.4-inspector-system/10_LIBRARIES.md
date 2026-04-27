# l8.4-inspector-system Libraries

## Allowed library classes
- property grouping schemas
- typed field-editor descriptors
- override/validation badge helpers
- component section expansion/pinning helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.4-inspector-system` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.4-inspector-system` and must stay specific enough to support implementation work without reinterpretation.
