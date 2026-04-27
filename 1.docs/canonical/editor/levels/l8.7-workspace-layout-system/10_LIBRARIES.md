# l8.7-workspace-layout-system Libraries

## Allowed library classes
- workspace layout schemas
- dock/undock/split descriptors
- tab persistence and restore helpers
- layout migration/version guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.7-workspace-layout-system` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.7-workspace-layout-system` and must stay specific enough to support implementation work without reinterpretation.
