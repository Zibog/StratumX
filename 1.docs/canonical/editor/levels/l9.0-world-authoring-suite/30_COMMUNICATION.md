# Communication

## Sends or publishes
- cell load/unload requests
- region lock publications
- World chunk save and HLOD bake trigger requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`world_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
