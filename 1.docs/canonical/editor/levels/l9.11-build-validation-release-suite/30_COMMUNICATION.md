# Communication

## Sends or publishes
- validate/bake/build/release requests
- graph refreshes
- suite status publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`build_validation_release_suite` may not smuggle state through undocumented callbacks or widget-local caches.
