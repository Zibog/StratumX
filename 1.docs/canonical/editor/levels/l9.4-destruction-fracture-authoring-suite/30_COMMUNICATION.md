# Communication

## Sends or publishes
- fracture generation requests
- destruction preview refreshes
- contract validation requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`destruction_fracture_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
