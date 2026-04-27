# l8.6-overlay-and-gizmo-system Libraries

## Allowed library classes
- overlay anchor and gizmo records
- manipulator math helpers
- hit-proxy and hover-state helpers
- viewport overlay layering descriptors

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.6-overlay-and-gizmo-system` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.6-overlay-and-gizmo-system` and must stay specific enough to support implementation work without reinterpretation.
