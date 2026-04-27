# l11.5-learning-onboarding-and-help-surface Libraries

## Allowed library classes
- help topic and onboarding step descriptors
- guided flow helpers
- contextual doc-link adapters
- progress tracking and hint suppression guards

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l11.5-learning-onboarding-and-help-surface` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l11.5-learning-onboarding-and-help-surface` and must stay specific enough to support implementation work without reinterpretation.
