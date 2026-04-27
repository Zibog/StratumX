# l10.0-project-bootstrap-service Libraries

## Allowed library classes
- project manifest/bootstrap descriptors
- initial package and workspace seed helpers
- template source resolution adapters
- bootstrap diagnostics and recovery guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l10.0-project-bootstrap-service` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l10.0-project-bootstrap-service` and must stay specific enough to support implementation work without reinterpretation.
