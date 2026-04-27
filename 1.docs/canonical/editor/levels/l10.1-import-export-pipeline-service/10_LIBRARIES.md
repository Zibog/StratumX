# l10.1-import-export-pipeline-service Libraries

## Allowed library classes
- import/export job descriptors
- source-to-artifact pipeline helpers
- reimport and invalidation adapters
- format/profile mapping helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l10.1-import-export-pipeline-service` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l10.1-import-export-pipeline-service` and must stay specific enough to support implementation work without reinterpretation.
