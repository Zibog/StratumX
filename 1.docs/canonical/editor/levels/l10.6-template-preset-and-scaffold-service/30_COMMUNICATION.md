# Communication

## Sends or publishes
- template apply requests
- generated item publications
- scaffold validation/build follow-ups

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`template_preset_and_scaffold_service` may not smuggle state through undocumented callbacks or widget-local caches.
