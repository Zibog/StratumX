# l9.2-terrain-landscape-authoring-suite Libraries

## Allowed library classes
- terrain brush and heightfield descriptors
- spline/scatter/erosion helpers
- terrain bake request adapters
- landscape material/weightmap helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.2-terrain-landscape-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.2-terrain-landscape-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
