# l8.5-tool-context-system Libraries

## Allowed library classes
- tool mode enums and context records
- mode transition validators
- context capability descriptors
- active-tool scope and legality helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.5-tool-context-system` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.5-tool-context-system` and must stay specific enough to support implementation work without reinterpretation.
