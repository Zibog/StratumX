# Communication

## Sends or publishes
- reload requests
- safe-list updates
- reload result/status publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`script_and_hot_reload_service` may not smuggle state through undocumented callbacks or widget-local caches.
