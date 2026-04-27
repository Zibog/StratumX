# Communication

## Sends or publishes
- plugin registration requests
- extension lifecycle publications
- plugin validation and isolation updates

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`plugin_and_extension_host` may not smuggle state through undocumented callbacks or widget-local caches.
