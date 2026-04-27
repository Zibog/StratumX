# l8.11-build-release-surface Libraries

## Allowed library classes
- build/release job card records
- channel/package target descriptors
- artifact status and publish gating helpers
- run history and retry visualization helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.11-build-release-surface` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.11-build-release-surface` and must stay specific enough to support implementation work without reinterpretation.
