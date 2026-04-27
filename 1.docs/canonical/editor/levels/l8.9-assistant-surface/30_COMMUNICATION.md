# Communication

## Sends or publishes
- assistant prompt submits
- proposal review updates
- apply/revert command requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`assistant_surface` may not smuggle state through undocumented callbacks or widget-local caches.
