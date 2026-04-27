# Communication

## Sends or publishes
- annotation create/update requests
- review state publications
- approval handoff requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`review_annotation_surface` may not smuggle state through undocumented callbacks or widget-local caches.
