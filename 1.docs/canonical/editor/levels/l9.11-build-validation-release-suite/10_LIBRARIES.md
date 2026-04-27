# l9.11-build-validation-release-suite Libraries

## Allowed library classes
- gate/checklist descriptors
- validation graph helpers
- artifact package and release channel adapters
- compliance and sign-off visualization helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.11-build-validation-release-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.11-build-validation-release-suite` and must stay specific enough to support implementation work without reinterpretation.
