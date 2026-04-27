# l9.6-weather-environment-authoring-suite Libraries

## Allowed library classes
- weather and environment parameter descriptors
- sky/fog/water/fire authoring helpers
- environment timeline/bake adapters
- coverage/zone preview helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.6-weather-environment-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.6-weather-environment-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
