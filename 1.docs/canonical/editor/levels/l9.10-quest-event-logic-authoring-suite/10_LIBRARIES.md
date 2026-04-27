# l9.10-quest-event-logic-authoring-suite Libraries

## Allowed library classes
- quest/event graph descriptors
- condition/trigger/effect helpers
- state machine and progression adapters
- runtime validation and trace link helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.10-quest-event-logic-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.10-quest-event-logic-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
