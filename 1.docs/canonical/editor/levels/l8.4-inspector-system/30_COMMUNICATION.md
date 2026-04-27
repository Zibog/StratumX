# Communication

## Sends or publishes
- inspect target changes
- field edit requests
- apply/revert/diff/validate/bake actions

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`inspector_system` may not smuggle state through undocumented callbacks or widget-local caches.
