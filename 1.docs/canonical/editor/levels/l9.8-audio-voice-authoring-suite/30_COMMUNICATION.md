# Communication

## Sends or publishes
- audio/voice edit requests
- audio preview refreshes
- dialogue binding validation requests

## Required posture
- sender and receiver surfaces are explicit
- mutating flows become commands or transaction-visible requests
- preview-like flows stay disposable and bounded

## Audit rule
`audio_voice_authoring_suite` may not smuggle state through undocumented callbacks or widget-local caches.
