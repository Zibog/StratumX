# l10.3-automation-and-batch-service Libraries

## Allowed library classes
- batch job descriptors
- queue and scheduler helpers for editor automation
- task fan-out/fan-in adapters
- automation audit and cancellation guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l10.3-automation-and-batch-service` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l10.3-automation-and-batch-service` and must stay specific enough to support implementation work without reinterpretation.
