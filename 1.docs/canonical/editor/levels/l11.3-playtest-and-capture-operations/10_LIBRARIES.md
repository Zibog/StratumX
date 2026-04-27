# l11.3-playtest-and-capture-operations Libraries

## Allowed library classes
- playtest session and capture descriptors
- runtime attach/capture helpers
- run result and replay adapters
- capture storage/export guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l11.3-playtest-and-capture-operations` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l11.3-playtest-and-capture-operations` and must stay specific enough to support implementation work without reinterpretation.
