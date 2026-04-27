# l8.9-assistant-surface Libraries

## Allowed library classes
- assistant conversation and proposal view models
- explanation/fix/action card descriptors
- assistant attach scope helpers
- human-approval and diff preview widgets

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.9-assistant-surface` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.9-assistant-surface` and must stay specific enough to support implementation work without reinterpretation.
