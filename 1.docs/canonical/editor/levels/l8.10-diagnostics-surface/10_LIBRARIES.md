# l8.10-diagnostics-surface Libraries

## Allowed library classes
- issue list/grouping descriptors
- severity and source badges
- diagnostics filtering/search helpers
- cross-link helpers into inspector/content/world

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.10-diagnostics-surface` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.10-diagnostics-surface` and must stay specific enough to support implementation work without reinterpretation.
