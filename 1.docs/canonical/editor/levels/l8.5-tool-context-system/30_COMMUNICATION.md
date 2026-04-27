# Communication

## Sends or publishes
- tool activation updates
- mode toggles
- tool capability refreshes

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`tool_context_system` may not smuggle state through undocumented callbacks or widget-local caches.
