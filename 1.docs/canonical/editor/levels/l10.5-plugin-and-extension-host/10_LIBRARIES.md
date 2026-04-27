# l10.5-plugin-and-extension-host Libraries

## Allowed library classes
- plugin registry and capability descriptors
- extension ABI/version compatibility helpers
- dock/inspector/command registration adapters
- sandbox loading and unload safety guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l10.5-plugin-and-extension-host` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l10.5-plugin-and-extension-host` and must stay specific enough to support implementation work without reinterpretation.
