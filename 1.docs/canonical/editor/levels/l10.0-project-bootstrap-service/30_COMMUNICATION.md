# Communication

## Sends or publishes
- new/open project requests
- mount and seed plan publications
- bootstrap result and recovery updates

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`project_bootstrap_service` may not smuggle state through undocumented callbacks or widget-local caches.
