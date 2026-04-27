# Communication

## Sends or publishes
- join/leave/share requests
- presence updates
- collaboration diagnostics publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`collaboration_session_surface` may not smuggle state through undocumented callbacks or widget-local caches.
