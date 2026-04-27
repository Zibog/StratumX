# Communication

## Sends or publishes
- material edit requests
- lookdev preview refreshes
- shader or variant build requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`material_lookdev_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
