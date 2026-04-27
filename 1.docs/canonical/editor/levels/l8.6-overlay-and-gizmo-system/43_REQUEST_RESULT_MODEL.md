# Request Result Model

## Request classes
- overlay redraw requests
- gizmo interaction results
- hit-test refresh publications

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
