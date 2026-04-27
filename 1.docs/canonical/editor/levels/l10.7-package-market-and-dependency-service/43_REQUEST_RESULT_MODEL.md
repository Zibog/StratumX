# Request Result Model

## Request classes
- package search and action requests
- dependency graph refreshes
- package validation/build publications

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
