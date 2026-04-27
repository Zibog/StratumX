# Communication

## Sends or publishes
- diagnostics refreshes
- issue selection publications
- validation re-run requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`diagnostics_surface` may not smuggle state through undocumented callbacks or widget-local caches.
