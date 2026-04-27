# Communication

## Sends or publishes
- quest/event graph edit requests
- signal track updates
- logic validation previews

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`quest_event_logic_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
