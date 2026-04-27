# l9.9-ui-hud-authoring-suite Libraries

## Allowed library classes
- widget tree and HUD layout descriptors
- style/theme asset helpers
- screen-binding and localization adapters
- ui preview and safe-zone helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.9-ui-hud-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.9-ui-hud-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
