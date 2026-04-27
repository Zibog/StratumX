# Request Result Model

## Request classes
- join/leave/share requests
- presence updates
- collaboration diagnostics publications

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
