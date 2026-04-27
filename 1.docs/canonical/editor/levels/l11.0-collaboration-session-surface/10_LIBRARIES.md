# l11.0-collaboration-session-surface Libraries

## Allowed library classes
- multi-user session descriptors
- presence/ownership adapters
- sync invitation and join/leave helpers
- shared review state guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l11.0-collaboration-session-surface` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l11.0-collaboration-session-surface` and must stay specific enough to support implementation work without reinterpretation.
