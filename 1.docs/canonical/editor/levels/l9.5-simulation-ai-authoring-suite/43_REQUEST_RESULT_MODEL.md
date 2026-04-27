# Request Result Model

## Request classes
- simulation profile updates
- AI preview refreshes
- validation and bake requests for sim data

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
