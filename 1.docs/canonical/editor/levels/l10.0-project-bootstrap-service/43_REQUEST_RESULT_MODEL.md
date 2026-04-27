# Request Result Model

## Request classes
- new/open project requests
- mount and seed plan publications
- bootstrap result and recovery updates

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
