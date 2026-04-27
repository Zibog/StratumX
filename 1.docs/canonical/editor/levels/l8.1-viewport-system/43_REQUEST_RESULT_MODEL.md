# Request Result Model

## Request classes
- viewport redraw requests
- camera and navigation publications
- preview attach/detach events

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
