# l8.0-editor-shell Libraries

## Allowed library classes
- window frame and shell-state records
- main menu/command bar descriptors
- dock host and shell routing helpers
- startup and session restore guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l8.0-editor-shell` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l8.0-editor-shell` and must stay specific enough to support implementation work without reinterpretation.
