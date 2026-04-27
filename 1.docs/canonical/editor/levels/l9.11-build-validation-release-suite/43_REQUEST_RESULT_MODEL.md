# Request Result Model

## Request classes
- validate/bake/build/release requests
- graph refreshes
- suite status publications

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
