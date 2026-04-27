# Communication

## Sends or publishes
- batch launch requests
- progress/status publications
- batch task request emissions

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`automation_and_batch_service` may not smuggle state through undocumented callbacks or widget-local caches.
