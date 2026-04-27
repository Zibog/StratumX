# l9.3-material-lookdev-authoring-suite Libraries

## Allowed library classes
- material graph and parameter descriptors
- shader variant and lookdev preview helpers
- slot mapping and override adapters
- texture channel and compression helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.3-material-lookdev-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.3-material-lookdev-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
