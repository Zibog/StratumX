# l9.5-simulation-ai-authoring-suite Libraries

## Allowed library classes
- AI/simulation graph descriptors
- agent/faction/behavior tuning helpers
- simulation probe and debug adapters
- spawn/needs/economy authoring helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.5-simulation-ai-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.5-simulation-ai-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
