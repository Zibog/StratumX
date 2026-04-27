# l9.4-destruction-fracture-authoring-suite Libraries

## Allowed library classes
- fracture pattern descriptors
- destruction authoring preview helpers
- damage profile and shard grouping helpers
- structural bake and validation adapters

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.4-destruction-fracture-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.4-destruction-fracture-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
