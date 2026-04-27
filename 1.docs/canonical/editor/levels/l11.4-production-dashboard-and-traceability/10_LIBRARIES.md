# l11.4-production-dashboard-and-traceability Libraries

## Allowed library classes
- dashboard metric cards and trace views
- work-item and artifact lineage helpers
- audit correlation adapters
- alert threshold and digest guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l11.4-production-dashboard-and-traceability` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l11.4-production-dashboard-and-traceability` and must stay specific enough to support implementation work without reinterpretation.
