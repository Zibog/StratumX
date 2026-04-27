# l8.2-outliner-system Libraries

## Allowed library classes
- hierarchy tree row records
- sort/filter/group descriptors for scene and asset trees
- selection synchronization helpers for outliner nodes
- row virtualization and visibility helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.2-outliner-system` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.2-outliner-system` and must stay specific enough to support implementation work without reinterpretation.
