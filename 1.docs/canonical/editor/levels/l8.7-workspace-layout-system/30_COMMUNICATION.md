# Communication

## Sends or publishes
- layout save/restore publications
- dock attach/detach updates
- workspace migration requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`workspace_layout_system` may not smuggle state through undocumented callbacks or widget-local caches.
