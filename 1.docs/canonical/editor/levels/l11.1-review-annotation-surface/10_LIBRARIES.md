# l11.1-review-annotation-surface Libraries

## Allowed library classes
- annotation/thread descriptors
- review pin/region helpers
- comment resolution and status adapters
- handoff and approval trace helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l11.1-review-annotation-surface` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l11.1-review-annotation-surface` and must stay specific enough to support implementation work without reinterpretation.
