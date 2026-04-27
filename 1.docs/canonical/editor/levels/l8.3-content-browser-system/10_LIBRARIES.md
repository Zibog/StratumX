# l8.3-content-browser-system Libraries

## Allowed library classes
- asset row/index adapters
- folder/path and dependency presentation helpers
- import status and artifact badge descriptors
- content filtering and bundle labeling helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.3-content-browser-system` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.3-content-browser-system` and must stay specific enough to support implementation work without reinterpretation.
