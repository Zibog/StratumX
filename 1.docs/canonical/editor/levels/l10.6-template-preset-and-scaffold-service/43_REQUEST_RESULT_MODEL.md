# Request Result Model

## Request classes
- template apply requests
- generated item publications
- scaffold validation/build follow-ups

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
