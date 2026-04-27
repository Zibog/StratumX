# Communication

## Sends or publishes
- overlay redraw requests
- gizmo interaction results
- hit-test refresh publications

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`overlay_and_gizmo_system` may not smuggle state through undocumented callbacks or widget-local caches.
