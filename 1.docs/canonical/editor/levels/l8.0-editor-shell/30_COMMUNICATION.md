# Communication

## Sends or publishes
- shell composition updates
- global mode broadcasts
- dock host registration updates

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`editor_shell` may not smuggle state through undocumented callbacks or widget-local caches.
