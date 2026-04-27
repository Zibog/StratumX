# Request Result Model

## Request classes
- batch launch requests
- progress/status publications
- batch task request emissions

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
