# Communication

## Sends or publishes
- interaction dispatches
- capture acquire/release publications
- drag-drop route results

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`interaction_routing_system` may not smuggle state through undocumented callbacks or widget-local caches.
