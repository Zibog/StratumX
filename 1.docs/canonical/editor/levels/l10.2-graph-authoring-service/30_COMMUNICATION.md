# Communication

## Sends or publishes
- graph edit requests
- node/edge selection publications
- graph validation/build requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`graph_authoring_service` may not smuggle state through undocumented callbacks or widget-local caches.
