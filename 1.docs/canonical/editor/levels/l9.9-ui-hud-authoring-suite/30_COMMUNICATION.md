# Communication

## Sends or publishes
- UI graph edit requests
- layout preview refreshes
- UI validation/build requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`ui_hud_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
