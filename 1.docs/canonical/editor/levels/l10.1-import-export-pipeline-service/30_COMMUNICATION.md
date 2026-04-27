# Communication

## Sends or publishes
- import/reimport/export requests
- generated file reveal publications
- dependency invalidation and rebuild requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`import_export_pipeline_service` may not smuggle state through undocumented callbacks or widget-local caches.
