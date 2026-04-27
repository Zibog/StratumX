# Request Result Model

## Request classes
- cell load/unload requests
- region lock publications
- World chunk save and HLOD bake trigger requests

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
