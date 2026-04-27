# Request Result Model

## Request classes
- UI graph edit requests
- layout preview refreshes
- UI validation/build requests

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
