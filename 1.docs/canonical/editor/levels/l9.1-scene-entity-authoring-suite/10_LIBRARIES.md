# l9.1-scene-entity-authoring-suite Libraries

## Allowed library classes
- entity/component authoring descriptors
- prefab instance and override helpers
- entity hierarchy and attachment adapters
- scene chunk/save posture helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.1-scene-entity-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.1-scene-entity-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
