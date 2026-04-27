# Communication

## Sends or publishes
- entity mutation requests
- Prefab Apply/Revert / diff / unpack requests
- component add/remove/edit requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`scene_entity_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
