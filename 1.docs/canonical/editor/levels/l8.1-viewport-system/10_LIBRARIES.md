# l8.1-viewport-system Libraries

## Allowed library classes
- camera/navigation math helpers
- render-target and viewport projection descriptors
- picking/raycast adapters for editor viewports
- bounded preview bridge helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.1-viewport-system` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.1-viewport-system` and must stay specific enough to support implementation work without reinterpretation.
