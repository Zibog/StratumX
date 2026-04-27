# Communication

## Sends or publishes
- asset listing refreshes
- dependency/reference queries
- reimport/rebuild/variant/prefab action requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`content_browser_system` may not smuggle state through undocumented callbacks or widget-local caches.
