# Request Result Model

## Request classes
- row selection publications
- hierarchy refreshes
- context action requests such as Create From Prefab or Save as Scene Chunk

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
