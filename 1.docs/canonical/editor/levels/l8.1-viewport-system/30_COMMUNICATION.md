# Communication

## Sends or publishes
- viewport redraw requests
- camera and navigation publications
- preview attach/detach events

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`viewport_system` may not smuggle state through undocumented callbacks or widget-local caches.
