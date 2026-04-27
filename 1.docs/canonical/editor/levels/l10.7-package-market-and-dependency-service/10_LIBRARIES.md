# l10.7-package-market-and-dependency-service Libraries

## Allowed library classes
- package manifest and dependency graph descriptors
- version/channel resolution helpers
- mount/export/import adapters
- package install/update/remove safety guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l10.7-package-market-and-dependency-service` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l10.7-package-market-and-dependency-service` and must stay specific enough to support implementation work without reinterpretation.
