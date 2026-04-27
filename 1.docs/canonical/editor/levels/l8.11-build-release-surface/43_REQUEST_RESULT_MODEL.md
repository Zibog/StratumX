# Request Result Model

## Request classes
- build/release action requests
- status refreshes
- artifact/release inspection publications

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
