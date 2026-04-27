# l9.0-world-authoring-suite Libraries

## Allowed library classes
- world/cell/region authoring descriptors
- streaming and partition visualization helpers
- data-layer and chunk authority adapters
- world metrics and heatmap helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.0-world-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.0-world-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
