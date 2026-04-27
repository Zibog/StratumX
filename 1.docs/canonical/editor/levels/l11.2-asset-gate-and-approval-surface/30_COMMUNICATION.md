# Communication

## Sends or publishes
- approve/reject/escalate requests
- gate queue refreshes
- approval status publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`asset_gate_and_approval_surface` may not smuggle state through undocumented callbacks or widget-local caches.
