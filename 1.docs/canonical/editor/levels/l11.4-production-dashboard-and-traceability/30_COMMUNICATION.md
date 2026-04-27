# Communication

## Sends or publishes
- dashboard refreshes
- drill-down selections
- export/report requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`production_dashboard_and_traceability` may not smuggle state through undocumented callbacks or widget-local caches.
