# l10.6-template-preset-and-scaffold-service Libraries

## Allowed library classes
- template/preset descriptors
- scaffold generation helpers
- preset inheritance and override adapters
- seed content and package bootstrap guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l10.6-template-preset-and-scaffold-service` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l10.6-template-preset-and-scaffold-service` and must stay specific enough to support implementation work without reinterpretation.
