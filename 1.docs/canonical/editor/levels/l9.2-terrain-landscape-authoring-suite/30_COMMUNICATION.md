# Communication

## Sends or publishes
- terrain sculpt/paint requests
- terrain preview refreshes
- terrain bake/build publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`terrain_landscape_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
