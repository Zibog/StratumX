# Communication

## Sends or publishes
- row selection publications
- hierarchy refreshes
- context action requests such as Create From Prefab or Save as Scene Chunk

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`outliner_system` may not smuggle state through undocumented callbacks or widget-local caches.
