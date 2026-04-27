# l10.4-script-and-hot-reload-service Libraries

## Allowed library classes
- script compilation/reload descriptors
- binding and reload-safety helpers
- sandbox/version boundary adapters
- failure rollback and reload diagnostics guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l10.4-script-and-hot-reload-service` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l10.4-script-and-hot-reload-service` and must stay specific enough to support implementation work without reinterpretation.
