# l9.7-animation-cinematics-authoring-suite Libraries

## Allowed library classes
- animation state and timeline descriptors
- shot/rail/blend helpers
- bind-track and clip payload adapters
- sequencer validation and preview helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.7-animation-cinematics-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.7-animation-cinematics-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
