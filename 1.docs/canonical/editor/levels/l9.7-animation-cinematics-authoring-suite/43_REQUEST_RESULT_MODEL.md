# Request Result Model

## Request classes
- track edit requests
- shot and camera rig updates
- timeline preview/playback publications

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
