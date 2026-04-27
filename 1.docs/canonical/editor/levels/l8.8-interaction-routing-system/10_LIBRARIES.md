# l8.8-interaction-routing-system Libraries

## Allowed library classes
- input route descriptors
- gesture normalization helpers
- command dispatch bridge adapters
- focus/capture legality validators

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.8-interaction-routing-system` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.8-interaction-routing-system` and must stay specific enough to support implementation work without reinterpretation.
