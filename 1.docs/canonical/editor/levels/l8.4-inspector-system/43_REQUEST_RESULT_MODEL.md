# Request Result Model

## Request classes
- inspect target changes
- field edit requests
- apply/revert/diff/validate/bake actions

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
