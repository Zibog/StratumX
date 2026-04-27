# Communication

## Sends or publishes
- package search and action requests
- dependency graph refreshes
- package validation/build publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`package_market_and_dependency_service` may not smuggle state through undocumented callbacks or widget-local caches.
